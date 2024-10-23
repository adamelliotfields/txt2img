mod client;
mod error;

use clap::Parser;
use regex::Regex;
use std::fs::File;
use std::io::Write;

pub use client::{BaseClient, StableDiffusionClient};
pub use error::GenError;

/// Defines the command-line arguments using Clap
#[derive(Parser, Debug)]
#[command(name = "gen", version, about = "Image generation CLI.", long_about = None)]
pub struct Args {
    /// The text prompt to generate the image (required)
    pub prompt: String,

    /// Negative prompt
    #[arg(short = 'n', long)]
    pub negative_prompt: Option<String>,

    /// Seed for deterministic generation
    #[arg(long)]
    pub seed: Option<u64>,

    /// Height of the image
    #[arg(long, default_value = "1024")]
    pub height: u32,

    /// Width of the image
    #[arg(long, default_value = "1024")]
    pub width: u32,

    /// Guidance scale
    #[arg(short, long, default_value = "10.0")]
    pub guidance_scale: f32,

    /// Inference steps
    #[arg(short = 's', long, default_value = "50")]
    pub num_inference_steps: u32,

    // TODO: eventually this will be a mapping of friendlier model identifiers to their actual repos
    /// Model to use
    #[arg(
        short,
        long,
        default_value = "stabilityai/stable-diffusion-xl-base-1.0"
    )]
    pub model: String,

    /// Output file path
    #[arg(short, long, default_value = "output.png")]
    pub out: String,
}

/// Struct representing the request body for the API
#[derive(serde::Serialize, Debug)]
pub struct RequestBody {
    pub inputs: String,
    pub parameters: Parameters,
}

/// Struct representing the parameters for the API request
#[derive(serde::Serialize, Debug)]
pub struct Parameters {
    pub seed: u64,
    pub width: u32,
    pub height: u32,
    pub guidance_scale: f32,
    pub num_inference_steps: u32,
    pub negative_prompt: String,
}

/// Struct representing the error response from the API
#[derive(serde::Deserialize, Debug)]
pub struct ErrorResponse {
    pub error: String,
}

/// Validates the user prompt using RegEx
pub fn validate_prompt(prompt: &str) -> Result<(), GenError> {
    let re = Regex::new(r"^[a-zA-Z0-9\s,.\-!]+$").unwrap();
    if re.is_match(prompt) {
        Ok(())
    } else {
        Err(GenError::InvalidPrompt)
    }
}

/// Writes the image bytes to a file
pub fn write_image(path: &str, image_bytes: &[u8]) -> Result<(), GenError> {
    let mut file = File::create(path).map_err(GenError::IoError)?;
    file.write_all(image_bytes).map_err(GenError::IoError)?;
    Ok(())
}
