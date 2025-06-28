use std::time::Instant;

use anyhow::{bail, Result};
use clap::Parser;
use colored::Colorize;
use log::{debug, error};
use tokio::select;

use txt2img::{create_client, create_progress_bar, init_logger, write_image, Cli};

async fn run() -> Result<()> {
    // Start timer
    let start = Instant::now();

    // Parse command line arguments
    let cli = Cli::parse();

    // Initialize logger
    let multi_progress = init_logger(cli.debug)?;

    // Handle list services flag
    if cli.list_services {
        for service in cli.get_services()? {
            println!("{service}");
        }
        return Ok(());
    }

    // Handle list models flag
    if cli.list_models {
        for model in cli.get_models()? {
            println!("{}", model.id);
        }
        return Ok(());
    }

    // Pin the future so its memory location doesn't change after polling
    let shutdown = tokio::signal::ctrl_c();
    tokio::pin!(shutdown);

    // Create progress bar and start it
    let pb = create_progress_bar(cli.quiet, &multi_progress);

    // Create client
    let service = cli.get_service()?;
    let client = create_client(service, &cli.timeout)?;

    // Update progress
    if let Some(pb) = &pb {
        pb.set_message("Generating image");
    }

    // Generate image
    let image_bytes = select! {
        // Start the block with `biased` to poll futures from top to bottom
        biased;
        _ = &mut shutdown => {
            if let Some(pb) = pb { pb.finish_and_clear(); }
            bail!("Operation cancelled by user");
        },
        result = client.generate_image(&cli) => result?,
    };

    // Update progress
    if let Some(pb) = &pb {
        pb.set_message("Saving image");
    }

    // Save
    let file_path = write_image(&cli.out, &image_bytes)?;

    // Take ownership of progress bar and stop it
    if let Some(pb) = pb {
        debug!("Stopping progress bar");
        let stop = format!("{:.2}", start.elapsed().as_secs_f32());
        let message = format!("Generated {} in {}s", file_path.blue(), stop.blue()).to_string();
        pb.finish_with_message(message);
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    if let Err(e) = run().await {
        error!("{} (main.rs)", e);
        std::process::exit(1);
    }
}
