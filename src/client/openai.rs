use std::collections::HashMap;
use std::env;
use std::time::Duration;

use anyhow::{bail, Context, Result};
use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use log::debug;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use serde_json::json;

use crate::cli::Cli;
use crate::services::ModelId;

use super::Client;

const ENV: &str = "OPENAI_API_KEY";
const URL: &str = "https://api.openai.com/v1/images/generations";

/// Image response entity
#[derive(serde::Deserialize, Debug)]
struct OpenAIImage {
    b64_json: String,
    // Default to None instead of a deserialization error
    #[serde(default)]
    revised_prompt: Option<String>,
}

/// Response from the OpenAI API
#[derive(serde::Deserialize, Debug)]
struct OpenAIImageResponse {
    data: Vec<OpenAIImage>,
}

/// Error response entity
#[derive(serde::Deserialize, Debug)]
struct OpenAIError {
    message: String,
}

/// Error response from the OpenAI API
#[derive(serde::Deserialize, Debug)]
struct OpenAIErrorResponse {
    error: OpenAIError,
}

/// OpenAI API client
#[derive(Debug)]
pub struct OpenAIClient {
    pub client: reqwest::Client,
}

#[async_trait::async_trait]
impl Client for OpenAIClient {
    fn new(timeout: u64) -> Result<Self> {
        let token = env::var(ENV).context(format!("`{}` not set (openai.rs)", ENV))?;
        let mut headers = HeaderMap::new();

        // https://platform.openai.com/docs/api-reference/images
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("{} {}", "Bearer", token))?, // fails on invalid characters
        );

        debug!("Creating OpenAI client");
        let client = reqwest::Client::builder()
            .default_headers(headers)
            .timeout(Duration::from_secs(timeout))
            .build()
            .context("System network error (openai.rs)")?;

        Ok(Self { client })
    }

    /// Generate an image using the OpenAI API
    async fn generate(
        &self,
        cli: &Cli,
    ) -> Result<Vec<u8>> {
        let model = cli.get_model()?;
        let mut request_body = HashMap::new();

        // Build dynamic parameters based on the model configuration
        request_body.insert("model".to_string(), json!(model.name));
        request_body.insert("prompt".to_string(), json!(cli.get_prompt()?));

        let width = cli.get_width()?;
        let height = cli.get_height()?;
        request_body.insert("size".to_string(), json!(format!("{}x{}", width, height)));

        if model.id == ModelId::Dalle3 {
            request_body.insert("style".to_string(), json!(cli.get_style()?));
        }

        // Add options if present
        // Note that there is no seed for DALL-E
        if let Some(options) = &model.options {
            for (key, value) in options {
                request_body.insert(key.clone(), value.clone());
            }
        }

        debug!("Sending request to OpenAI API");
        let response = match self
            .client
            .post(URL)
            .json(&request_body)
            .send()
            .await
        {
            Ok(response) => response,
            Err(e) if e.is_timeout() => {
                let t = cli.get_timeout()?;
                bail!("Request timed out after {} seconds (openai.rs)", t)
            }
            Err(e) => {
                bail!("{} (openai.rs)", e)
            }
        };

        // Handle the response
        if response.status().is_success() {
            debug!("Parsing response from OpenAI API");
            let openai_response: OpenAIImageResponse = response.json().await?;

            if let Some(openai_image) = openai_response.data.first() {
                let b64_json = &openai_image.b64_json;

                // Only on DALL-E 3
                if let Some(revised_prompt) = &openai_image.revised_prompt {
                    debug!("Revised prompt: {}", revised_prompt);
                }

                let image_bytes = STANDARD
                    .decode(b64_json)
                    .context("Failed to decode base64 image (openai.rs)")?;

                Ok(image_bytes)
            } else {
                bail!("No image data found in response (openai.rs)")
            }
        } else {
            // Error generating image
            let error_response: OpenAIErrorResponse = response.json().await?;
            bail!("{} (openai.rs)", error_response.error.message)
        }
    }
}
