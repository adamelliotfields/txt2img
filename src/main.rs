use std::time::{Duration, Instant};

use anyhow::Result;
use clap::Parser;
use colored::Colorize;
use indicatif::{ProgressBar, ProgressStyle};
use log::{debug, error};

use gen::{create_client, init_logger, write_image, Args};

async fn run() -> Result<()> {
    // Start timer
    let start = Instant::now();

    // Parse command line arguments
    let args = Args::parse();

    // Initialize logger
    let quiet = args.get_quiet()?;
    let debug = args.get_debug()?;
    let multi = init_logger(debug)?;

    // Handle list services flag
    if args.get_list_services()? {
        for service in args.get_services()? {
            println!("{}", service);
        }
        return Ok(());
    }

    // Handle list models flag
    if args.get_list_models()? {
        for model in args.get_models()? {
            println!("{}", model.id);
        }
        return Ok(());
    }

    // Create progress bar and start it
    let pb = if !quiet {
        debug!("Starting progress bar");
        let pb = multi.add(ProgressBar::new_spinner());
        pb.enable_steady_tick(Duration::from_millis(80));
        pb.set_style(
            // https://github.com/sindresorhus/cli-spinners/blob/main/spinners.json
            ProgressStyle::with_template("{spinner:.blue} {msg}")
                .unwrap()
                .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏", "✓"]), // dots
        );
        Some(pb)
    } else {
        None
    };

    // Update progress
    if let Some(pb) = &pb {
        pb.set_message("Generating image");
    }

    // Generate
    let service = args.get_service()?;
    let timeout = args.get_timeout()?;
    let client = create_client(service, timeout)?;
    let image_bytes = client.generate(&args).await?;

    // Update progress
    if let Some(pb) = &pb {
        pb.set_message("Saving image");
    }

    // Save
    let file_path = write_image(args.get_out()?, &image_bytes)?;

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
