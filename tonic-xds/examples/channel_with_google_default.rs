//! Call an xDS-fronted service through GCP Traffic Director (`google_default`).
//!
//! Supplies the Application Default Credentials (ADC) token as xDS call
//! credentials by implementing `TonicCallCredentials` directly against
//! `google-cloud-auth`.
//!
//! Needs a `google_default` bootstrap + ADC:
//!
//! ```sh
//! GRPC_XDS_BOOTSTRAP=/path/to/bootstrap.json \
//!     cargo run -p tonic-xds --example channel_with_google_default --features "testutil tls-ring"
//! ```

use std::sync::Arc;

use google_cloud_auth::credentials::{AccessTokenCredentials, Builder};
use tonic_xds::testutil::proto::helloworld::{HelloRequest, greeter_client::GreeterClient};
use tonic_xds::{TonicCallCredentials, XdsChannelBuilder, XdsChannelConfig, XdsUri};

const CLOUD_PLATFORM_SCOPE: &str = "https://www.googleapis.com/auth/cloud-platform";

/// Fetches ADC tokens directly from `google-cloud-auth`.
#[derive(Debug)]
struct AdcTonicCallCredentials {
    creds: AccessTokenCredentials,
}

impl AdcTonicCallCredentials {
    fn new() -> std::result::Result<Self, Box<dyn std::error::Error>> {
        let creds = Builder::default()
            .with_scopes([CLOUD_PLATFORM_SCOPE])
            .build_access_token_credentials()?;
        Ok(Self { creds })
    }
}

#[tonic::async_trait]
impl TonicCallCredentials for AdcTonicCallCredentials {
    async fn get_request_metadata(
        &self,
        metadata: &mut tonic::metadata::MetadataMap,
    ) -> std::result::Result<(), tonic::Status> {
        let token = self
            .creds
            .access_token()
            .await
            .map_err(|e| tonic::Status::unauthenticated(e.to_string()))?;
        let mut value =
            tonic::metadata::AsciiMetadataValue::try_from(format!("Bearer {}", token.token))
                .map_err(|e| tonic::Status::invalid_argument(e.to_string()))?;
        // Mark the bearer token sensitive so it is not accidentally logged.
        value.set_sensitive(true);
        metadata.insert("authorization", value);
        Ok(())
    }

    fn requires_secure_transport(&self) -> bool {
        true
    }
}

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let target_str = std::env::var("XDS_TARGET").unwrap_or_else(|_| "xds:///my-service".into());
    let target = XdsUri::parse(&target_str)?;

    let creds: Arc<dyn TonicCallCredentials> = Arc::new(AdcTonicCallCredentials::new()?);

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
