#[derive(Debug)]
pub enum FalError {
    InvalidCredentials,
    RequestError(reqwest::Error),
}
