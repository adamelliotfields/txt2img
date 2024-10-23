use clap::Parser;
use tokio;

use gen::{validate_prompt, write_image, Args, BaseClient, StableDiffusionClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    // Validate prompt
    validate_prompt(&args.prompt)?;

    // Initialize generator
    let client = StableDiffusionClient::new()?;

    // Generate image
    let image_bytes = client.predict(&args).await?;

    // Write image to file
    write_image(&args.out, &image_bytes)?;
    println!("Image saved to {}", args.out);
    Ok(())
}
