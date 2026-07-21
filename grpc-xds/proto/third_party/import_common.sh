#!/bin/bash
# Shared helper for the per-directory xDS proto import scripts.
#
# Each third_party/<name>/import.sh sources this file, sets the variables
# below, and then calls import_protos:
#
#   VERSION                upstream revision (tag or commit) — informational
#   DOWNLOAD_URL           tarball URL for the pinned upstream revision
#   DOWNLOAD_BASE_DIR      top-level directory inside the extracted tarball
#   SOURCE_PROTO_BASE_DIR  path within the tarball that roots the proto tree
#   FILES                  array of proto paths to copy, relative to the root
#
# The destination directory is inferred from the calling script's location and
# protos are written directly under <caller_dir>. Re-importing first removes any
# previously-imported files (all *.proto, plus LICENSE and NOTICE) so stale files
# never linger. LICENSE is always copied; NOTICE only when the upstream ships one.

import_protos() {
  local dest_dir
  dest_dir="$(cd "$(dirname "${BASH_SOURCE[1]}")" && pwd)"

  pushd "${dest_dir}" > /dev/null

  # put the repo in a tmp directory
  local tmpdir
  tmpdir="$(mktemp -d)"
  trap 'rm -rf "${tmpdir}"' RETURN

  curl -Ls "${DOWNLOAD_URL}" | tar xz -C "${tmpdir}"

  # Remove previously-imported files so stale protos never linger, then prune
  # any package directories left empty.
  find . -name '*.proto' -delete
  find . -type d -empty -delete
  rm -f LICENSE NOTICE

  cp -p "${tmpdir}/${DOWNLOAD_BASE_DIR}/LICENSE" LICENSE
  if [[ -f "${tmpdir}/${DOWNLOAD_BASE_DIR}/NOTICE" ]]; then
    cp -p "${tmpdir}/${DOWNLOAD_BASE_DIR}/NOTICE" NOTICE
  fi

  # copy proto files to project directory
  local total=${#FILES[@]} copied=0 file
  for file in "${FILES[@]}"; do
    mkdir -p "$(dirname "${file}")"
    cp -p "${tmpdir}/${SOURCE_PROTO_BASE_DIR}/${file}" "${file}" && (( ++copied ))
  done

  popd > /dev/null

  echo "Imported ${copied} files."
  if (( copied != total )); then
    echo "Failed importing $(( total - copied )) files." 1>&2
    return 1
  fi
}
