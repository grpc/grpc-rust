/*
 *
 * Copyright 2025 gRPC authors.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to
 * deal in the Software without restriction, including without limitation the
 * rights to use, copy, modify, merge, publish, distribute, sublicense, and/or
 * sell copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
 * FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS
 * IN THE SOFTWARE.
 *
 */

use std::ffi::OsString;
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;

use grpc::client::interceptor::InvokeExt as _;
use grpc::credentials::LocalChannelCredentials;
use grpc_rustls::RootCertificates;
use grpc_rustls::StaticProvider;
use grpc_rustls::client::ClientTlsConfig as GrpcClientTlsConfig;
use grpc_rustls::client::RustlsChannelCredentials;
use interop::client::GrpcMetadataInterceptor;
use interop::client::InteropTest;
use interop::client::InteropTestUnimplemented;
use interop::client::MetadataInterceptor;
use interop::client_prost;
use interop::client_protobuf;
use tonic::transport::Certificate;
use tonic::transport::ClientTlsConfig;
use tonic::transport::Endpoint;

#[derive(Debug)]
struct Opts {
    server_host: String,
    server_port: u16,
    server_host_override: Option<String>,
    use_tls: bool,
    use_test_ca: Option<bool>,
    ca_file: Option<String>,
    test_case: Vec<Testcase>,
    codec: Codec,
    additional_metadata: Option<String>,
}

#[derive(Debug)]
enum Codec {
    Prost,
    Protobuf,
}

impl FromStr for Codec {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "prost" => Ok(Codec::Prost),
            "protobuf" => Ok(Codec::Protobuf),
            _ => Err(format!("Invalid codec: {}", s)),
        }
    }
}

impl Opts {
    fn parse() -> Result<Self, pico_args::Error> {
        // interop/test.sh passes the boolean flags bare (`--use_tls`); gRPC's
        // interop harness passes them with a value (`--use_tls=true`).
        let args: Vec<OsString> = std::env::args_os()
            .skip(1)
            .map(|arg| match arg.to_str() {
                Some("--use_tls") => OsString::from("--use_tls=true"),
                Some("--use_test_ca") => OsString::from("--use_test_ca=true"),
                _ => arg,
            })
            .collect();
        let mut pargs = pico_args::Arguments::from_vec(args);
        Ok(Self {
            server_host: pargs
                .opt_value_from_str("--server_host")?
                .unwrap_or_else(|| "localhost".to_string()),
            server_port: pargs.opt_value_from_str("--server_port")?.unwrap_or(10000),
            server_host_override: pargs.opt_value_from_str("--server_host_override")?,
            use_tls: pargs.opt_value_from_str("--use_tls")?.unwrap_or(false),
            use_test_ca: pargs.opt_value_from_str("--use_test_ca")?,
            ca_file: pargs.opt_value_from_str("--ca_file")?,
            test_case: pargs.value_from_fn("--test_case", |test_case| {
                test_case.split(',').map(Testcase::from_str).collect()
            })?,
            codec: pargs.value_from_str("--codec")?,
            additional_metadata: pargs.opt_value_from_str("--additional_metadata")?,
        })
    }

    // Whether to trust the test CA instead of the platform's root CAs. Without
    // `--use_test_ca`, local servers get the test CA, so interop/test.sh
    // works without passing the flag.
    fn effective_use_test_ca(&self) -> bool {
        self.use_test_ca.unwrap_or_else(|| {
            self.ca_file.is_some()
                || matches!(self.server_host.as_str(), "localhost" | "127.0.0.1" | "::1")
        })
    }
}

fn format_host_port(host: &str, port: u16) -> String {
    if host.contains(':') && !host.starts_with('[') {
        format!("[{host}]:{port}")
    } else {
        format!("{host}:{port}")
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    interop::trace_init();

    let matches = Opts::parse()?;
    let use_test_ca = matches.effective_use_test_ca();
    let ca_file = matches.ca_file.as_deref().unwrap_or("interop/data/ca.pem");
    let host_port = format_host_port(&matches.server_host, matches.server_port);

    let test_cases = matches.test_case;

    let (mut client, mut unimplemented_client): (
        Box<dyn InteropTest>,
        Box<dyn InteropTestUnimplemented>,
    ) = match matches.codec {
        Codec::Prost => {
            let scheme = if matches.use_tls { "https" } else { "http" };
            let mut endpoint = Endpoint::try_from(format!("{scheme}://{host_port}"))?
                .timeout(Duration::from_secs(5))
                .concurrency_limit(30);

            if matches.use_tls {
                let mut tls_config = ClientTlsConfig::new();
                if use_test_ca {
                    let ca = Certificate::from_pem(std::fs::read_to_string(ca_file)?);
                    let domain = matches
                        .server_host_override
                        .as_deref()
                        .unwrap_or("foo.test.google.fr");
                    tls_config = tls_config.ca_certificate(ca).domain_name(domain);
                } else {
                    tls_config = tls_config.with_native_roots();
                    if let Some(domain) = &matches.server_host_override {
                        tls_config = tls_config.domain_name(domain.clone());
                    }
                }
                endpoint = endpoint.tls_config(tls_config)?;
            }

            let channel = endpoint.connect().await?;
            let interceptor = MetadataInterceptor::new(matches.additional_metadata.as_deref())?;
            let channel =
                tonic::service::interceptor::InterceptedService::new(channel, interceptor);

            (
                Box::new(client_prost::TestClient::new(channel.clone())),
                Box::new(client_prost::UnimplementedClient::new(channel)),
            )
        }
        Codec::Protobuf => {
            let target = format!("dns:///{host_port}");
            let channel = if matches.use_tls {
                let _ = rustls::crypto::ring::default_provider().install_default();

                let creds = if use_test_ca {
                    let root_certs = RootCertificates::from_pem(std::fs::read_to_string(ca_file)?);
                    RustlsChannelCredentials::new(
                        GrpcClientTlsConfig::new()
                            .with_root_certificates_provider(StaticProvider::new(root_certs)),
                    )?
                } else {
                    RustlsChannelCredentials::new(GrpcClientTlsConfig::new())?
                };
                let mut builder = grpc::client::Channel::builder(&target, Arc::new(creds));
                let authority = matches
                    .server_host_override
                    .as_deref()
                    .or(use_test_ca.then_some("test.test.google.fr"));
                if let Some(authority) = authority {
                    builder = builder.authority(authority);
                }
                builder.build()
            } else {
                let mut builder = grpc::client::Channel::builder(
                    &target,
                    Arc::new(LocalChannelCredentials::new()),
                );
                if let Some(authority) = &matches.server_host_override {
                    builder = builder.authority(authority);
                }
                builder.build()
            };
            let interceptor = GrpcMetadataInterceptor::new(matches.additional_metadata.as_deref())?;
            let channel = channel.with_interceptor(interceptor);

            (
                Box::new(client_protobuf::TestClient::new(channel.clone())),
                Box::new(client_protobuf::UnimplementedClient::new(channel)),
            )
        }
    };

    let mut failures = Vec::new();

    for test_case in test_cases {
        println!("{test_case:?}:");
        let mut test_results = Vec::new();

        match test_case {
            Testcase::EmptyUnary => client.empty_unary(&mut test_results).await,
            Testcase::LargeUnary => client.large_unary(&mut test_results).await,
            Testcase::ClientStreaming => client.client_streaming(&mut test_results).await,
            Testcase::ServerStreaming => client.server_streaming(&mut test_results).await,
            Testcase::PingPong => client.ping_pong(&mut test_results).await,
            Testcase::EmptyStream => client.empty_stream(&mut test_results).await,
            Testcase::StatusCodeAndMessage => {
                client.status_code_and_message(&mut test_results).await
            }
            Testcase::SpecialStatusMessage => {
                client.special_status_message(&mut test_results).await
            }
            Testcase::UnimplementedMethod => client.unimplemented_method(&mut test_results).await,
            Testcase::UnimplementedService => {
                unimplemented_client
                    .unimplemented_service(&mut test_results)
                    .await
            }
            Testcase::CustomMetadata => client.custom_metadata(&mut test_results).await,
            _ => unimplemented!(),
        }

        for result in test_results {
            println!("  {result}");

            if result.is_failed() {
                failures.push(result);
            }
        }
    }

    if !failures.is_empty() {
        println!("{} tests failed", failures.len());
        std::process::exit(1);
    }

    Ok(())
}

#[derive(Debug, strum::EnumString)]
#[strum(serialize_all = "snake_case")]
enum Testcase {
    EmptyUnary,
    CacheableUnary,
    LargeUnary,
    ClientCompressedUnary,
    ServerCompressedUnary,
    ClientStreaming,
    ClientCompressedStreaming,
    ServerStreaming,
    ServerCompressedStreaming,
    PingPong,
    EmptyStream,
    ComputeEngineCreds,
    JwtTokenCreds,
    Oauth2AuthToken,
    PerRpcCreds,
    CustomMetadata,
    StatusCodeAndMessage,
    SpecialStatusMessage,
    UnimplementedMethod,
    UnimplementedService,
    CancelAfterBegin,
    CancelAfterFirstResponse,
    TimeoutOnSleepingServer,
    ConcurrentLargeUnary,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_host_port_brackets_ipv6_literals() {
        assert_eq!(format_host_port("localhost", 8080), "localhost:8080");
        assert_eq!(format_host_port("::1", 8080), "[::1]:8080");
        assert_eq!(format_host_port("[::1]", 8080), "[::1]:8080");
    }
}
