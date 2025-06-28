use std::collections::HashMap;
use std::env;
use std::time::Duration;

use anyhow::{bail, Context, Result};
use log::debug;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use serde_json::json;

use crate::cli::Cli;

use super::Client;

const ENV: &str = "HF_TOKEN";
const URL: &str = "https://api-inference.huggingface.co/models";

/// Request body for the Hugging Face API
#[derive(serde::Serialize, Debug)]
struct HuggingFaceRequest {
    inputs: String,
    parameters: HashMap<String, serde_json::Value>,
}

/// Error response body from the Hugging Face API
#[derive(serde::Deserialize, Debug)]
struct HuggingFaceErrorResponse {
    error: String,
}

#[derive(Debug)]
pub struct HuggingFaceClient {
    pub client: reqwest::Client,
}

#[async_trait::async_trait]
impl Client for HuggingFaceClient {
    fn new(timeout: u64) -> Result<Self> {
        let token = env::var(ENV).context(format!("`{ENV}` not set (hf.rs)"))?;
        let mut headers = HeaderMap::new();

        // https://huggingface.co/docs/api-inference/en/parameters
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("CONTENT_TYPE_JSON"));
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {token}"))?, // fails on invalid characters
        );

        // Wait for the model to load rather than 503
        headers.insert(
            HeaderName::from_static("x-wait-for-model"),
            HeaderValue::from_static("true"),
        );

        // Don't use cached generations
        headers.insert(
            HeaderName::from_static("x-use-cache"),
            HeaderValue::from_static("false"),
        );

        debug!("Creating Hugging Face client");
        let client = reqwest::Client::builder()
            .default_headers(headers)
            .timeout(Duration::from_secs(timeout))
            .build()
            .context("System network error (hf.rs)")?;

        Ok(Self { client })
    }

    /// Generate an image using the Hugging Face API
    async fn generate_image(
        &self,
        cli: &Cli,
    ) -> Result<Vec<u8>> {
        let model = cli.get_model()?;
        let mut parameters = HashMap::new();

        // Build parameters based on the model configuration
        if model.width.is_some() {
            parameters.insert("width".to_string(), json!(cli.get_width()?));
        }

        if model.height.is_some() {
            parameters.insert("height".to_string(), json!(cli.get_height()?));
        }

        if model.cfg.is_some() {
            parameters.insert("guidance_scale".to_string(), json!(cli.get_cfg()?));
        }

        if model.steps.is_some() {
            parameters.insert("num_inference_steps".to_string(), json!(cli.get_steps()?));
        }

        if model.negative_prompt.is_some() {
            parameters.insert("negative_prompt".to_string(), json!(cli.get_negative_prompt()?));
        }

        // Add seed if present
        if let Some(seed) = cli.seed {
            parameters.insert("seed".to_string(), json!(seed));
        }

        // Add options if present
        if let Some(options) = &model.options {
            for (key, value) in options {
                parameters.insert(key.clone(), value.clone());
            }
        }

        // Append the model ID to the base URL
        let api_url = format!("{URL}/{}", model.name);

        // Get the prompt (can safely unwrap because it's required by Clap)
        let inputs = cli.get_prompt()?.unwrap().to_string();

        // Build the request body
        let request_body = HuggingFaceRequest { parameters, inputs };

        // Send the request
        debug!("Sending request to Hugging Face API");
        let response = match self.client.post(api_url).json(&request_body).send().await {
            Ok(response) => response,
            Err(e) if e.is_timeout() => {
                bail!("Request timed out after {} seconds (hf.rs)", cli.timeout)
            }
            Err(e) => {
                bail!("{e} (hf.rs)")
            }
        };

        // Handle the response
        if response.status().is_success() {
            debug!("Parsing response from Hugging Face API");
            let bytes = response.bytes().await?;
            Ok(bytes.to_vec())
        } else {
            let error_response: HuggingFaceErrorResponse = response.json().await?;
            bail!("{} (hf.rs)", error_response.error)
        }
    }
}
