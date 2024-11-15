use std::time::Instant;

use clap::Parser;

use gen::{create_client, get_or_init_config, write_image, Args};

async fn run() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize configuration
    get_or_init_config()?;

    // Parse command line arguments
    let args = Args::parse();

    // Handle list services flag
    if args.get_list_services() {
        for service in args.get_services() {
            println!("{}", service);
        }
        return Ok(());
    }

    // Handle list models flag
    if args.get_list_models() {
        for model in args.get_models()? {
            println!("{}", model.id);
        }
        return Ok(());
    }

    // Start timer
    let start = Instant::now();

    // Initialize generator
    let client = create_client(args.get_service()?)?;

    // Generate image
    let image_bytes = client.generate(&args).await?;

    // Write image to file
    let file_path = write_image(args.get_out(), &image_bytes)?;

    // Print elapsed time and exit
    let elapsed = start.elapsed().as_secs_f32();
    println!("Generated {} in {:.2}s", file_path, elapsed);
    Ok(())
}

#[tokio::main]
async fn main() {
    if let Err(err) = run().await {
        eprintln!("{} (main.rs)", err);
        std::process::exit(1);
    }
}
