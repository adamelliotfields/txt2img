use std::collections::HashMap;
use std::env;
use std::time::Duration;

use anyhow::{bail, Context, Result};
use log::debug;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use serde_json::json;

use crate::cli::Cli;

use super::Client;

const ENV: &str = "TOGETHER_API_KEY";
const URL: &str = "https://api.together.xyz/v1";

/// Image response entity
#[derive(serde::Deserialize, Debug)]
struct TogetherImage {
    url: String,
}

/// Response from the Together API
#[derive(serde::Deserialize, Debug)]
struct TogetherResponse {
    data: Vec<TogetherImage>,
}

/// Error response entity
#[derive(serde::Deserialize, Debug)]
struct TogetherError {
    message: String,
}

/// Error response from the Together API
#[derive(serde::Deserialize, Debug)]
struct TogetherErrorResponse {
    error: TogetherError,
}

/// Together API client
#[derive(Debug)]
pub struct TogetherClient {
    pub client: reqwest::Client,
}

#[async_trait::async_trait]
impl Client for TogetherClient {
    fn new(timeout: u64) -> Result<Self> {
        let token = env::var(ENV).context(format!("`{}` not set (together.rs)", ENV))?;
        let mut headers = HeaderMap::new();

        // https://docs.together.ai/reference/post_images-generations
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("{} {}", "Bearer", token))?, // fails on invalid characters
        );

        debug!("Creating Together client");
        let client = reqwest::Client::builder()
            .default_headers(headers)
            .timeout(Duration::from_secs(timeout))
            .build()
            .context("System network error (together.rs)")?;

        Ok(Self { client })
    }

    /// Generate an image using the Together API
    async fn generate_image(
        &self,
        cli: &Cli,
    ) -> Result<Vec<u8>> {
        let model = cli.get_model()?;
        let mut request_body = HashMap::new();

        // Build dynamic parameters based on the model configuration
        request_body.insert("model".to_string(), json!(model.name));
        request_body.insert("prompt".to_string(), json!(cli.get_prompt()?));
        request_body.insert("output_format".to_string(), json!("png"));
        request_body.insert("response_format".to_string(), json!("url"));
        request_body.insert("n".to_string(), json!(1));

        // The model's supported parameters have default values in the config
        if model.width.is_some() {
            request_body.insert("width".to_string(), json!(cli.get_width()?));
        }

        if model.height.is_some() {
            request_body.insert("height".to_string(), json!(cli.get_height()?));
        }

        if model.steps.is_some() {
            request_body.insert("steps".to_string(), json!(cli.get_steps()?));
        }

        if model.cfg.is_some() {
            // The CLI arg is "cfg" but the API expects "guidance"
            request_body.insert("guidance".to_string(), json!(cli.get_cfg()?));
        }

        // Add seed if preset
        if let Some(seed) = cli.get_seed()? {
            request_body.insert("seed".to_string(), json!(seed));
        }

        // Add options if present
        if let Some(options) = &model.options {
            for (key, value) in options {
                request_body.insert(key.clone(), value.clone());
            }
        }

        debug!("Sending request to Together API");
        let image_url = format!("{}/images/generations", URL);
        let response = match self
            .client
            .post(image_url)
            .json(&request_body)
            .send()
            .await
        {
            Ok(response) => response,
            Err(e) if e.is_timeout() => {
                let t = cli.get_timeout()?;
                bail!("Request timed out after {} seconds (together.rs)", t)
            }
            Err(e) => {
                bail!("{} (together.rs)", e)
            }
        };

        // Handle the response
        if response.status().is_success() {
            let together_response: TogetherResponse = response.json().await?;

            debug!("Parsing first response from Together API");
            let response_image_url = together_response
                .data
                .first()
                .unwrap()
                .url
                .clone();

            debug!("Fetching image result");
            let response_image = self
                .client
                .get(response_image_url)
                .send()
                .await?;

            if response_image.status().is_success() {
                debug!("Parsing second response from Together API");
                let bytes = response_image.bytes().await?;
                Ok(bytes.to_vec())
            } else {
                // Error fetching image after successful generation
                bail!("Failed to fetch image after successful generation (together.rs)")
            }
        } else {
            // Error generating image
            let error_response: TogetherErrorResponse = response.json().await?;
            bail!("{} (together.rs)", error_response.error.message)
        }
    }
}
