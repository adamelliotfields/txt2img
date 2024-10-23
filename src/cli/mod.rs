use clap::Parser;

/// The Args struct defines the command-line arguments using Clap
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
