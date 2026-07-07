use std::{
    fs::File,
    io::{BufWriter, Write as _},
    path::{Path, PathBuf},
    time::Instant,
};

use protox::prost::Message as _;
use quote::quote;
use tonic_prost_build::FileDescriptorSet;

fn main() {
    println!("Running codegen...");

    let start = Instant::now();

    // tonic-health
    codegen(
        &PathBuf::from(std::env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .join("tonic-health"),
        &["proto/health.proto"],
        &["proto"],
        &PathBuf::from("src/generated"),
        &PathBuf::from("src/generated/grpc_health_v1_fds.rs"),
        true,
        true,
    );

    // tonic-reflection
    codegen(
        &PathBuf::from(std::env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .join("tonic-reflection"),
        &["proto/reflection_v1.proto"],
        &["proto"],
        &PathBuf::from("src/generated"),
        &PathBuf::from("src/generated/reflection_v1_fds.rs"),
        true,
        true,
    );
    codegen(
        &PathBuf::from(std::env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .join("tonic-reflection"),
        &["proto/reflection_v1alpha.proto"],
        &["proto"],
        &PathBuf::from("src/generated"),
        &PathBuf::from("src/generated/reflection_v1alpha1_fds.rs"),
        true,
        true,
    );

    // tonic-types
    codegen(
        &PathBuf::from(std::env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .join("tonic-types"),
        &["proto/status.proto", "proto/error_details.proto"],
        &["proto"],
        &PathBuf::from("src/generated"),
        &PathBuf::from("src/generated/types_fds.rs"),
        false,
        false,
    );

    // grpc
    codegen(
        &PathBuf::from(std::env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .join("grpc"),
        &["proto/echo/echo.proto"],
        &["proto"],
        &PathBuf::from("src/generated"),
        &PathBuf::from("src/generated/echo_fds.rs"),
        true,
        true,
    );

    #[cfg(feature = "xds")]
    regenerate_xds();

    println!("Codgen completed: {}ms", start.elapsed().as_millis());
}

/// Regenerates the checked-in xDS message structs under grpc/src/xds/generated.
///
/// Message-only (no service stubs). Each proto is generated in its OWN protoc
/// invocation with a shared crate mapping, so every cross-file reference
/// resolves to an in-crate module path (`crate::xds::generated::<pkg>::<file>`)
/// instead of the generator's default single flat namespace — which collides
/// for same-named messages across packages (e.g. `TypedStruct` in `udpa.type.v1`
/// and `xds.type.v3`). Gated behind the `xds` feature so the common codegen path
/// doesn't pull in the protobuf/protoc toolchain.
#[cfg(feature = "xds")]
fn regenerate_xds() {
    use std::path::PathBuf;

    let grpc_dir = PathBuf::from(std::env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("grpc");
    let third_party = grpc_dir.join("proto/xds/third_party");
    let out_dir = grpc_dir.join("src/xds/generated");

    // The five vendored dependency roots double as protoc include paths.
    let include_dirs: Vec<PathBuf> = [
        "envoy",
        "xds",
        "protoc-gen-validate",
        "googleapis",
        "cel-spec",
    ]
    .iter()
    .map(|d| third_party.join(d))
    .collect();

    // protoc's own include dir provides the well-known types (imported by many
    // vendored protos) plus `google/protobuf/descriptor.proto` (imported by the
    // option-defining protos), so protoc can parse every import.
    let protoc = protoc_gen_rust_grpc::protoc();
    let protoc_include = protoc
        .parent()
        .and_then(|p| p.parent())
        .map(|p| p.join("include"))
        .expect("could not derive protoc include dir");

    // Collect every vendored proto as an include-relative import path, e.g.
    // "envoy/config/route/v3/route_components.proto".
    let mut protos: Vec<String> = Vec::new();
    for dir in &include_dirs {
        let mut found = Vec::new();
        collect_protos(dir, &mut found);
        for p in found {
            protos.push(p.strip_prefix(dir).unwrap().to_string_lossy().into_owned());
        }
    }
    protos.sort();
    protos.dedup();

    // Start from a clean output directory.
    if out_dir.exists() {
        std::fs::remove_dir_all(&out_dir).unwrap();
    }
    std::fs::create_dir_all(&out_dir).unwrap();

    // Generate each proto in its OWN invocation: within a single-file invocation
    // there are no in-crate name collisions. Every OTHER file is declared as a
    // dependency mapped to its in-crate module path, so cross-file references
    // resolve to `crate::xds::generated::...` instead of one flat namespace.
    // Using protobuf_codegen's typed `Dependency` API lets it own the
    // `crate_mapping.txt` format rather than us hand-writing it.
    //
    // `CodeGen::new()` eagerly reads `OUT_DIR` (it targets build scripts); set
    // it before the first call — the value is overridden by `.output_dir()`.
    // SAFETY: codegen's `main` is single-threaded up to this point.
    unsafe {
        std::env::set_var("OUT_DIR", &out_dir);
    }

    // Deps shared by every invocation: well-known types resolve to the external
    // `protobuf_well_known_types` crate. `descriptor.proto` is imported only to
    // DEFINE custom options (validate rules, xDS/udpa annotations); nothing
    // references its types so we don't generate it, but protoc still calls
    // GetCrateName on it while emitting the (deleted) entry point and FATALs on
    // an unmapped import — so map it benignly too.
    let mut base_deps = protobuf_well_known_types::get_dependency("protobuf_well_known_types");
    base_deps.push(protobuf_codegen::Dependency {
        crate_name: "protobuf_well_known_types".to_string(),
        proto_import_paths: vec![protoc_include.clone()],
        proto_files: vec!["google/protobuf/descriptor.proto".to_string()],
    });

    // The vendored roots plus protoc's bundle (for the well-known types) resolve
    // every import, so per-file deps carry only the crate mapping (no extra
    // search paths).
    let includes: Vec<PathBuf> = include_dirs
        .iter()
        .cloned()
        .chain(std::iter::once(protoc_include.clone()))
        .collect();

    for file in &protos {
        let mut deps = base_deps.clone();
        for other in &protos {
            if other != file {
                deps.push(protobuf_codegen::Dependency {
                    crate_name: crate_module_path(other),
                    proto_import_paths: Vec::new(),
                    proto_files: vec![other.clone()],
                });
            }
        }
        protobuf_codegen::CodeGen::new()
            .protoc_path(&protoc)
            .input(file)
            .includes(includes.iter())
            .output_dir(&out_dir)
            .dependency(deps)
            .generate_and_compile()
            .unwrap_or_else(|e| panic!("xDS codegen failed for {file}: {e}"));
    }

    // Drop protoc's per-invocation artifacts: the flat `generated.rs`
    // aggregators (we build our own nested module tree) and the `crate_mapping.txt`
    // protobuf_codegen writes into the output dir.
    remove_files_named(&out_dir, "generated.rs");
    let _ = std::fs::remove_file(out_dir.join("crate_mapping.txt"));

    // Emit the wrapper module tree: each file becomes a `pub mod <stem>` that
    // re-exports its `.u.pb.rs` (so the file's own `super::` sibling refs
    // resolve), nested under package modules mirroring the proto directories.
    write_module_tree(&out_dir);

    println!(
        "Regenerated {} xDS protos -> {}",
        protos.len(),
        out_dir.display()
    );
}

/// Maps an include-relative proto path to its in-crate module path, e.g.
/// `envoy/config/route/v3/route_components.proto` ->
/// `crate::xds::generated::envoy::config::route::v3::route_components`.
#[cfg(feature = "xds")]
fn crate_module_path(proto_rel: &str) -> String {
    let stem = proto_rel.strip_suffix(".proto").unwrap_or(proto_rel);
    let path: Vec<String> = stem.split('/').map(raw_ident).collect();
    format!("crate::xds::generated::{}", path.join("::"))
}

/// Wraps a path segment as a raw identifier when it is a Rust keyword.
#[cfg(feature = "xds")]
fn raw_ident(seg: &str) -> String {
    const KEYWORDS: &[&str] = &[
        "as", "break", "const", "continue", "else", "enum", "extern", "false", "fn", "for", "if",
        "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub", "ref", "return",
        "static", "struct", "trait", "true", "type", "unsafe", "use", "where", "while", "async",
        "await", "dyn", "abstract", "become", "box", "do", "final", "macro", "override", "priv",
        "typeof", "unsized", "virtual", "yield", "try", "gen",
    ];
    if KEYWORDS.contains(&seg) {
        format!("r#{seg}")
    } else {
        seg.to_string()
    }
}

/// Recursively writes the wrapper module tree (`mod.rs` per directory plus a
/// `<stem>.rs` per generated `.u.pb.rs`) mirroring the proto package layout.
#[cfg(feature = "xds")]
fn write_module_tree(dir: &std::path::Path) {
    const ALLOW: &str = "#![allow(missing_docs, unreachable_pub, non_camel_case_types, \
        non_snake_case, non_upper_case_globals, unused, clippy::all)]";
    const HEADER: &str = "// @generated by `cargo run -p codegen --features xds`. Do not edit.";

    let mut subdirs: Vec<String> = Vec::new();
    let mut stems: Vec<String> = Vec::new();
    for entry in std::fs::read_dir(dir).unwrap() {
        let path = entry.unwrap().path();
        let name = path.file_name().unwrap().to_string_lossy().into_owned();
        if path.is_dir() {
            subdirs.push(name);
            write_module_tree(&path);
        } else if let Some(stem) = name.strip_suffix(".u.pb.rs") {
            stems.push(stem.to_string());
        }
    }
    subdirs.sort();
    stems.sort();

    // A sub-package and a message file must not share a name (they'd map to the
    // same module). Not expected in the xDS tree.
    for s in &stems {
        assert!(
            !subdirs.contains(s),
            "name conflict in {}: `{s}` is both a proto file and a package dir",
            dir.display()
        );
    }

    // Per-file wrapper: `<stem>.rs` re-exports `<stem>.u.pb.rs`.
    for stem in &stems {
        std::fs::write(
            dir.join(format!("{stem}.rs")),
            format!(
                "{HEADER}\n{ALLOW}\n#[path = {file:?}]\nmod internal;\npub use internal::*;\n",
                file = format!("{stem}.u.pb.rs"),
            ),
        )
        .unwrap();
    }

    // Directory module: declare sub-packages then file wrappers.
    let mut m = format!("{HEADER}\n{ALLOW}\n");
    for s in &subdirs {
        m.push_str(&format!(
            "#[path = {:?}]\npub mod {};\n",
            format!("{s}/mod.rs"),
            raw_ident(s)
        ));
    }
    for t in &stems {
        m.push_str(&format!(
            "#[path = {:?}]\npub mod {};\n",
            format!("{t}.rs"),
            raw_ident(t)
        ));
    }
    std::fs::write(dir.join("mod.rs"), m).unwrap();
}

/// Recursively removes every file named `name` under `dir`.
#[cfg(feature = "xds")]
fn remove_files_named(dir: &std::path::Path, name: &str) {
    for entry in std::fs::read_dir(dir).unwrap() {
        let path = entry.unwrap().path();
        if path.is_dir() {
            remove_files_named(&path, name);
        } else if path.file_name().is_some_and(|n| n == name) {
            let _ = std::fs::remove_file(&path);
        }
    }
}

/// Recursively collects `*.proto` file paths under `dir` into `out`.
#[cfg(feature = "xds")]
fn collect_protos(dir: &std::path::Path, out: &mut Vec<std::path::PathBuf>) {
    for entry in std::fs::read_dir(dir).unwrap() {
        let path = entry.unwrap().path();
        if path.is_dir() {
            collect_protos(&path, out);
        } else if path.extension().is_some_and(|e| e == "proto") {
            out.push(path);
        }
    }
}

fn codegen(
    root_dir: &Path,
    iface_files: &[&str],
    include_dirs: &[&str],
    out_dir: &Path,
    file_descriptor_set_path: &Path,
    build_client: bool,
    build_server: bool,
) {
    let tempdir = tempfile::Builder::new()
        .prefix("tonic-codegen-")
        .tempdir()
        .unwrap();

    let iface_files = iface_files.iter().map(|&path| root_dir.join(path));
    let include_dirs = include_dirs.iter().map(|&path| root_dir.join(path));
    let out_dir = root_dir.join(out_dir);
    let file_descriptor_set_path = root_dir.join(file_descriptor_set_path);

    let fds = protox::compile(iface_files, include_dirs).unwrap();

    write_fds(&fds, &file_descriptor_set_path);

    tonic_prost_build::configure()
        .build_client(build_client)
        .build_server(build_server)
        .build_transport(false)
        .out_dir(&tempdir)
        .compile_fds(fds)
        .unwrap();

    for path in std::fs::read_dir(tempdir.path()).unwrap() {
        let path = path.unwrap().path();
        let to = out_dir.join(
            path.file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .strip_suffix(".rs")
                .unwrap()
                .replace('.', "_")
                + ".rs",
        );
        std::fs::copy(&path, &to).unwrap();
    }
}

fn write_fds(fds: &FileDescriptorSet, path: &Path) {
    const GENERATED_COMMENT: &str = "// This file is @generated by codegen.";

    let mut file_header = String::new();

    let mut fds = fds.clone();

    for fd in fds.file.iter() {
        let Some(source_code_info) = &fd.source_code_info else {
            continue;
        };

        for location in &source_code_info.location {
            for comment in &location.leading_detached_comments {
                file_header += comment;
            }
        }
    }

    for fd in fds.file.iter_mut() {
        fd.source_code_info = None;
    }

    let fds_raw = fds.encode_to_vec();
    let tokens = quote! {
        /// Byte encoded FILE_DESCRIPTOR_SET.
        pub const FILE_DESCRIPTOR_SET: &[u8] = &[#(#fds_raw),*];
    };
    let ast = syn::parse2(tokens).unwrap();
    let formatted = prettyplease::unparse(&ast);

    let mut writer = BufWriter::new(File::create(path).unwrap());

    writer.write_all(GENERATED_COMMENT.as_bytes()).unwrap();
    writer.write_all(b"\n").unwrap();

    if !file_header.is_empty() {
        let file_header = comment_out(&file_header);
        writer.write_all(file_header.as_bytes()).unwrap();
        writer.write_all(b"\n").unwrap();
    }

    writer.write_all(formatted.as_bytes()).unwrap()
}

fn comment_out(s: &str) -> String {
    s.split('\n')
        .map(|line| format!("// {line}"))
        .collect::<Vec<String>>()
        .join("\n")
}
