use crate::client::BaseClient;
use crate::error::GenError;
use crate::{Args, ErrorResponse, Parameters, RequestBody};

use rand::Rng;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use std::env;

#[derive(Debug)]
pub struct StableDiffusionClient {
    client: reqwest::Client,
    api_url: String,
}

// Using XL because there are never warm 1.5 endpoints; can use 1.5 when local inference is implemented
impl StableDiffusionClient {
    pub fn new() -> Result<Self, GenError> {
        let token = env::var("HF_TOKEN").map_err(|_| GenError::MissingToken)?;
        let api_url = format!(
            "https://api-inference.huggingface.co/models/stabilityai/stable-diffusion-xl-base-1.0"
        );

        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", token)).unwrap(),
        );
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .map_err(|e| GenError::GenerationError(e.to_string()))?;

        Ok(Self { client, api_url })
    }
}

#[async_trait::async_trait]
impl BaseClient for StableDiffusionClient {
    async fn predict(&self, args: &Args) -> Result<Vec<u8>, GenError> {
        // Handle seed being None
        let seed = match args.seed {
            Some(s) => s,
            _ => rand::thread_rng().gen::<u64>(),
        };

        // Handle negative prompt being None
        let negative_prompt = args.negative_prompt.as_deref().unwrap_or("").to_owned();

        // Build the request body
        let request_body = RequestBody {
            inputs: args.prompt.clone(),
            parameters: Parameters {
                seed,
                negative_prompt,
                width: args.width,
                height: args.height,
                guidance_scale: args.guidance_scale,
                num_inference_steps: args.num_inference_steps,
            },
        };

        // Send the request
        let response = self
            .client
            .post(&self.api_url)
            .json(&request_body)
            .send()
            .await?;

        // Handle the response
        if response.status().is_success() {
            let bytes = response.bytes().await?;
            Ok(bytes.to_vec())
        } else {
            let error_response: ErrorResponse = response.json().await?;
            Err(GenError::GenerationError(error_response.error))
        }
    }
}
