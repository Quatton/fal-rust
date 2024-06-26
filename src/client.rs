use reqwest::{header, Response};

use super::error::FalError;

#[derive(Debug, Clone)]
pub enum ClientCredentials {
    Key(String),
    FromEnv(String),
    KeyPair(String, String),
}

const ENV_CANDIDATES: [&str; 2] = ["FAL_KEY", "FAL_API_KEY"];

impl ClientCredentials {
    pub fn from_env() -> Self {
        dotenvy::dotenv().ok();

        for candidate in ENV_CANDIDATES {
            if let Ok(key) = std::env::var(candidate) {
                return Self::Key(key);
            }
        }

        if let Ok(key_id) = std::env::var("FAL_KEY_ID") {
            if let Ok(secret) = std::env::var("FAL_SECRET") {
                return Self::KeyPair(key_id, secret);
            }
        }

        panic!("FAL_KEY or FAL_KEY_ID and FAL_SECRET must be set in the environment");
    }

    pub fn from_key(key: &str) -> Self {
        Self::Key(key.to_string())
    }

    pub fn from_key_pair(key_id: &str, secret: &str) -> Self {
        Self::KeyPair(key_id.to_string(), secret.to_string())
    }
}

#[derive(Debug, Clone)]
pub struct FalClient {
    credentials: ClientCredentials,
}

impl FalClient {
    pub fn build_url(&self, path: &str) -> String {
        let host = std::env::var("FAL_RUN_HOST").unwrap_or("fal.run".to_string());
        let base_url = format!("https://{}", host);
        format!("{}/{}", base_url, path.trim_start_matches('/'))
    }

    pub fn new(credentials: ClientCredentials) -> Self {
        Self { credentials }
    }

    fn client(&self) -> reqwest::Client {
        let mut header = header::HeaderMap::new();
        let creds = match &self.credentials {
            ClientCredentials::Key(key) => key.clone(),
            ClientCredentials::KeyPair(key_id, secret) => format!("{}:{}", key_id, secret),
            ClientCredentials::FromEnv(_) => panic!("FAL_API_KEY must be set in the environment"),
        };

        header.insert("Authorization", format!("Key {}", creds).parse().unwrap());

        reqwest::Client::builder()
            .default_headers(header)
            .build()
            .unwrap()
    }

    pub async fn run<T: serde::Serialize>(
        &self,
        funtion_id: &str,
        inputs: T,
    ) -> Result<Response, FalError> {
        let client = self.client();
        let url = self.build_url(funtion_id);
        let res = client.post(&url).json(&inputs).send().await;

        match res {
            Ok(res) => {
                if res.status().is_success() {
                    Ok(res)
                } else {
                    Err(FalError::InvalidCredentials)
                }
            }
            Err(e) => Err(FalError::RequestError(e)),
        }
    }
}
