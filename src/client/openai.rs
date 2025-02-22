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

use super::Client;

const ENV: &str = "OPENAI_API_KEY";
const URL: &str = "https://api.openai.com/v1";

/// Image response entity
#[derive(serde::Deserialize, Debug)]
struct OpenAIImage {
    b64_json: String,
    // Default to None instead of a deserialization error
    #[serde(default)]
    revised_prompt: Option<String>,
}

/// Image response from the OpenAI API
#[derive(serde::Deserialize, Debug)]
struct OpenAIImageResponse {
    data: Vec<OpenAIImage>,
}

/// Text message entity
#[derive(serde::Deserialize, Debug)]
struct OpenAITextMessage {
    content: String,
}

/// Text message choice entity
#[derive(serde::Deserialize, Debug)]
struct OpenAITextChoice {
    message: OpenAITextMessage,
}

/// Text response from the OpenAI API
#[derive(serde::Deserialize, Debug)]
struct OpenAITextResponse {
    choices: Vec<OpenAITextChoice>,
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
    async fn generate_image(
        &self,
        cli: &Cli,
    ) -> Result<Vec<u8>> {
        let model = cli.get_model()?;
        let mut request_body = HashMap::new();

        // Build dynamic parameters based on the model configuration
        request_body.insert("model".to_string(), json!(model.name));
        request_body.insert("prompt".to_string(), json!(cli.get_prompt()?));
        request_body.insert("response_format".to_string(), json!("b64_json"));
        request_body.insert("n".to_string(), json!(1));

        let width = cli.get_width()?;
        let height = cli.get_height()?;
        request_body.insert("size".to_string(), json!(format!("{}x{}", width, height)));

        if model.style.is_some() {
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
                bail!("Request timed out after {} seconds (openai.rs)", t)
            }
            Err(e) => {
                bail!("{} (openai.rs)", e)
            }
        };

        // Handle the response
        if response.status().is_success() {
            debug!("Parsing response from OpenAI API");
            let image_response: OpenAIImageResponse = response.json().await?;

            if let Some(image_data) = image_response.data.first() {
                let b64_json = &image_data.b64_json;

                if let Some(revised_prompt) = &image_data.revised_prompt {
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

    /// Generate text using the OpenAI API
    async fn generate_text(
        &self,
        cli: &Cli,
    ) -> Result<String> {
        let model = cli.get_model()?;
        let mut request_body = HashMap::new();

        // Static parameters
        request_body.insert("modalities".to_string(), json!(["text"]));
        request_body.insert("stream".to_string(), json!(false));
        request_body.insert("n".to_string(), json!(1));

        // Build dynamic parameters based on the model configuration
        request_body.insert("model".to_string(), json!(model.name));

        if model.frequency.is_some() {
            request_body.insert("frequency_penalty".to_string(), json!(cli.get_frequency()?));
        }

        if model.presence.is_some() {
            request_body.insert("presence_penalty".to_string(), json!(cli.get_presence()?));
        }

        if model.temperature.is_some() {
            request_body.insert("temperature".to_string(), json!(cli.get_temperature()?));
        }

        // TODO: for reasoning models use "developer" role, otherwise use "system"
        let system_prompt = cli.get_system_prompt()?;
        let user_prompt = cli.get_prompt()?;
        request_body.insert(
            "messages".to_string(),
            json!([{"role": "system", "content": system_prompt}, {"role": "user", "content": user_prompt}]),
        );

        debug!("Sending request to OpenAI API");
        let text_url = format!("{}/chat/completions", URL);
        let response = match self
            .client
            .post(text_url)
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
            let text_response: OpenAITextResponse = response.json().await?;

            if let Some(openai_choice) = text_response.choices.first() {
                let text_message = &openai_choice.message.content;
                Ok(text_message.to_string())
            } else {
                bail!("No text content found in response (openai.rs)")
            }
        } else {
            // Error generating image
            let error_response: OpenAIErrorResponse = response.json().await?;
            bail!("{} (openai.rs)", error_response.error.message)
        }
    }
}
