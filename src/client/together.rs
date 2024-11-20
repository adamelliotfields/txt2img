use std::collections::HashMap;
use std::env;
use std::time::Duration;

use anyhow::{bail, Context, Result};
use log::debug;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use serde_json::json;

use crate::cli::Args;
use crate::config::get_or_init_config;

use super::Client;

const ENV: &str = "TOGETHER_API_KEY";
const URL: &str = "https://api.together.xyz/v1/images/generations";

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
    fn new() -> Result<Self> {
        let config = get_or_init_config()?;
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
            .timeout(Duration::from_secs(config.timeout))
            .build()
            .context("System network error (together.rs)")?;

        Ok(Self { client })
    }

    /// Generate an image using the Together API
    async fn generate(
        &self,
        args: &Args,
    ) -> Result<Vec<u8>> {
        let model_config = args.get_model_config()?;
        let mut request_body = HashMap::new();

        // Build dynamic parameters based on the model configuration
        request_body.insert("model".to_string(), json!(model_config.name));
        request_body.insert("prompt".to_string(), json!(args.get_prompt()?));

        if model_config.width.is_some() {
            request_body.insert("width".to_string(), json!(args.get_width()?));
        }

        if model_config.height.is_some() {
            request_body.insert("height".to_string(), json!(args.get_height()?));
        }

        if model_config.steps.is_some() {
            request_body.insert("steps".to_string(), json!(args.get_steps()?));
        }

        // Add seed if preset
        if let Some(seed) = args.get_seed()? {
            request_body.insert("seed".to_string(), json!(seed));
        }

        // Add options if present
        if let Some(options) = &model_config.options {
            for (key, value) in options {
                request_body.insert(key.clone(), value.clone());
            }
        }

        debug!("Sending request to Together API");
        let response = self
            .client
            .post(URL)
            .json(&request_body)
            .send()
            .await?;

        // Handle the response
        if response.status().is_success() {
            let together_response: TogetherResponse = response.json().await?;

            debug!("Parsing first response from Together API");
            let image_url = together_response
                .data
                .first()
                .unwrap()
                .url
                .clone();

            debug!("Fetching image result");
            let image_response = self
                .client
                .get(image_url)
                .send()
                .await?;

            if image_response.status().is_success() {
                debug!("Parsing second response from Together API");
                let bytes = image_response.bytes().await?;
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
