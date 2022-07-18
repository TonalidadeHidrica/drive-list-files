use google_drive3::{
    hyper,
    hyper_rustls::HttpsConnectorBuilder,
    oauth2::{self, InstalledFlowAuthenticator, InstalledFlowReturnMethod},
    DriveHub,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = hyper::Client::builder().build(
        HttpsConnectorBuilder::new()
            .with_native_roots()
            .https_or_http()
            .enable_http1()
            .enable_http2()
            .build(),
    );
    let secret = oauth2::read_application_secret(".google/client_secret.json").await?;
    let auth = InstalledFlowAuthenticator::builder(secret, InstalledFlowReturnMethod::HTTPRedirect)
        .persist_tokens_to_disk(".google/tokens.json")
        .build()
        .await
        .unwrap();
    let drive = DriveHub::new(client, auth);

    let res = drive.files().list().doit().await;
    println!("{:?}", res);

    Ok(())
}
