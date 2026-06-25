//! Call an xDS-fronted service through GCP Traffic Director (`google_default`).
//!
//! Shows two ways to supply the Application Default Credentials (ADC) token as
//! xDS call credentials, selected by `XDS_CRED_SOURCE`:
//!
//! - `adc` (default): implement `CallCredentials` directly against
//!   `google-cloud-auth`. No dependency on the `grpc` / `grpc-google` crates,
//!   which are a preview and "not recommended for any production use".
//! - `grpc-google`: bridge `grpc_google::GcpCallCredentials` into the seam (a
//!   convenience wrapper, but it pulls in the preview `grpc` crate).
//!
//! Needs a `google_default` bootstrap + ADC:
//!
//! ```sh
//! GRPC_XDS_BOOTSTRAP=/path/to/bootstrap.json \
//!     cargo run -p tonic-xds --example channel_with_google_default --features "testutil tls-ring"
//! ```

use std::collections::HashMap;
use std::sync::Arc;

use google_cloud_auth::credentials::{AccessTokenCredentials, Builder};
use grpc::attributes::Attributes;
use grpc::credentials::SecurityLevel;
// `GrpcCallCredentials`: trait in scope so its methods resolve on `inner`.
use grpc::credentials::call::{
    CallCredentials as GrpcCallCredentials, CallDetails, ClientConnectionSecurityInfo,
};
use grpc::metadata::{KeyAndValueRef, MetadataMap};
use grpc_google::{GcpCallCredentials, TokenProvider};
use tonic_xds::testutil::proto::helloworld::{HelloRequest, greeter_client::GreeterClient};
use tonic_xds::{XdsChannelBuilder, XdsChannelConfig, XdsUri};
use xds_client::{CallCredentials, Error, Result};

const ADS_METHOD: &str =
    "envoy.service.discovery.v3.AggregatedDiscoveryService/StreamAggregatedResources";
const CLOUD_PLATFORM_SCOPE: &str = "https://www.googleapis.com/auth/cloud-platform";

/// Fetches ADC tokens directly from `google-cloud-auth`.
#[derive(Debug)]
struct AdcCallCredentials {
    creds: AccessTokenCredentials,
}

impl AdcCallCredentials {
    fn new() -> std::result::Result<Self, Box<dyn std::error::Error>> {
        let creds = Builder::default()
            .with_scopes([CLOUD_PLATFORM_SCOPE])
            .build_access_token_credentials()?;
        Ok(Self { creds })
    }
}

#[tonic::async_trait]
impl CallCredentials for AdcCallCredentials {
    async fn get_request_metadata(&self) -> Result<HashMap<String, String>> {
        let token = self
            .creds
            .access_token()
            .await
            .map_err(|e| Error::CallCredentials(e.to_string()))?;
        Ok(HashMap::from([(
            "authorization".to_string(),
            format!("Bearer {}", token.token),
        )]))
    }

    fn requires_secure_transport(&self) -> bool {
        true
    }
}

/// Bridges `grpc_google::GcpCallCredentials` into the seam.
#[derive(Debug)]
struct XdsGcpCallCredentials<P> {
    inner: GcpCallCredentials<P>,
    server_uri: String,
}

#[tonic::async_trait]
impl<P> CallCredentials for XdsGcpCallCredentials<P>
where
    // TokenProvider implies Sync + Debug + 'static; Send is required separately.
    P: TokenProvider + Send,
{
    async fn get_request_metadata(&self) -> Result<HashMap<String, String>> {
        let call_details = CallDetails::new(self.server_uri.clone(), ADS_METHOD);
        let auth_info = ClientConnectionSecurityInfo::new(
            "tls",
            SecurityLevel::PrivacyAndIntegrity,
            Attributes::new(),
        );

        let mut md = MetadataMap::new();
        self.inner
            .get_metadata(&call_details, &auth_info, &mut md)
            .await
            // StatusError has no Display impl; use its message.
            .map_err(|e| Error::CallCredentials(e.message().to_owned()))?;

        // GcpCallCredentials emits a single ASCII `authorization` header.
        let mut out = HashMap::new();
        for kv in md.iter() {
            if let KeyAndValueRef::Ascii(key, value) = kv {
                out.insert(key.as_str().to_owned(), value.to_str().to_owned());
            }
        }
        Ok(out)
    }

    fn requires_secure_transport(&self) -> bool {
        self.inner.minimum_channel_security_level() == SecurityLevel::PrivacyAndIntegrity
    }
}

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let target_str = std::env::var("XDS_TARGET").unwrap_or_else(|_| "xds:///my-service".into());
    let target = XdsUri::parse(&target_str)?;

    let source = std::env::var("XDS_CRED_SOURCE").unwrap_or_else(|_| "adc".into());
    let creds: Arc<dyn CallCredentials> = match source.as_str() {
        "adc" => Arc::new(AdcCallCredentials::new()?),
        "grpc-google" => {
            let gcp = GcpCallCredentials::new_application_default()
                .map_err(|e| format!("failed to load ADC: {e}"))?;
            Arc::new(XdsGcpCallCredentials {
                inner: gcp,
                server_uri: target_str.clone(),
            })
        }
        other => return Err(format!("unknown XDS_CRED_SOURCE: {other}").into()),
    };

    let channel =
        XdsChannelBuilder::new(XdsChannelConfig::new(target).with_call_credentials(creds))
            .build_grpc_channel()?;

    let mut client = GreeterClient::new(channel);
    let response = client
        .say_hello(HelloRequest {
            name: "xds-gcp".into(),
        })
        .await?;

    println!("RESPONSE = {}", response.into_inner().message);
    Ok(())
}
