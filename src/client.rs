use super::error::FalError;

pub enum ClientCredentials {
    Key(String),
    FromEnv(String),
    KeyPair(String, String),
}

impl ClientCredentials {
    pub fn from_env() -> Self {
        // get env var
        if let Ok(key) = std::env::var("FAL_API_KEY") {
            return Self::Key(key);
        }

        if let Ok(key_id) = std::env::var("FAL_KEY_ID") {
            if let Ok(secret) = std::env::var("FAL_SECRET") {
                return Self::KeyPair(key_id, secret);
            }
        }

        panic!("FAL_API_KEY or FAL_KEY_ID and FAL_SECRET must be set in the environment");
    }

    pub fn from_key(key: &str) -> Self {
        Self::Key(key.to_string())
    }

    pub fn from_key_pair(key_id: &str, secret: &str) -> Self {
        Self::KeyPair(key_id.to_string(), secret.to_string())
    }
}

pub struct FalClient {
    credentials: ClientCredentials,
}
