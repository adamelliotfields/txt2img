use thiserror::Error;

#[derive(Error, Debug)]
pub enum GenError {
    #[error("Failed to generate image: {0}")]
    GenerationError(String),

    #[error("Invalid prompt")]
    InvalidPrompt,

    #[error("Environment variable `HF_TOKEN` not set")]
    MissingToken,

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Request error: {0}")]
    ReqwestError(#[from] reqwest::Error),
}
