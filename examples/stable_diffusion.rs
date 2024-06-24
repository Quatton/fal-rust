use fal_rust::{
    client::{ClientCredentials, FalClient},
    utils::download_image,
};
use serde::{Deserialize, Serialize};
use tokio::task::JoinSet;

#[derive(Debug, Serialize, Deserialize)]
struct ImageResult {
    url: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Output {
    images: Vec<ImageResult>,
}

#[tokio::main]
async fn main() {
    let client = FalClient::new(ClientCredentials::from_env());

    let res = client
        .run(
            "fal-ai/stable-cascade",
            serde_json::json!({
                "prompt": "A large waterfall in the middle of a volcano, surrounded by lush greenery and children's playground equipment.",
            }),
        )
        .await
        .unwrap();

    let output: Output = res.json::<Output>().await.unwrap();

    let url = output.images[0].url.clone();
    let filename = url.split('/').last().unwrap();

    download_image(&url, format!("{}/{}", "images", filename).as_str())
        .await
        .unwrap();
}
