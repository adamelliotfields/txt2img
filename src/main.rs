use std::time::{Duration, Instant};

use anyhow::{bail, Result};
use clap::Parser;
use colored::Colorize;
use indicatif::{ProgressBar, ProgressStyle};
use log::{debug, error};

use gen::{create_client, init_logger, write_image, Cli, ModelKind};

async fn run() -> Result<()> {
    // Start timer
    let start = Instant::now();

    // Parse command line arguments
    let cli = Cli::parse();

    // Initialize logger
    let quiet = cli.get_quiet()?;
    let debug = cli.get_debug()?;
    let multi = init_logger(debug)?;

    // Handle list services flag
    if cli.get_list_services()? {
        for service in cli.get_services()? {
            println!("{}", service);
        }
        return Ok(());
    }

    // Handle list models flag
    if cli.get_list_models()? {
        for model in cli.get_models()? {
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

    // Generate
    let model = cli.get_model()?;
    let service = cli.get_service()?;
    let timeout = cli.get_timeout()?;
    let client = create_client(service, timeout)?;

    // Update progress
    if let Some(pb) = &pb {
        match model.kind {
            ModelKind::Image => pb.set_message("Generating image"),
            ModelKind::Text => pb.set_message("Generating text"),
        }
    }

    if model.kind == ModelKind::Image {
        let image_bytes = client.generate_image(&cli).await?;

        // Update progress
        if let Some(pb) = &pb {
            pb.set_message("Saving image");
        }

        // Save
        let file_path = write_image(cli.get_out()?, &image_bytes)?;

        // Take ownership of progress bar and stop it
        if let Some(pb) = pb {
            debug!("Stopping progress bar");
            let stop = format!("{:.2}", start.elapsed().as_secs_f32());
            let message = format!("Generated {} in {}s", file_path.blue(), stop.blue()).to_string();
            pb.finish_with_message(message);
        }

        Ok(())
    } else if model.kind == ModelKind::Text {
        let text = client.generate_text(&cli).await?;

        // Take ownership of progress bar and stop it
        if let Some(pb) = pb {
            debug!("Stopping progress bar");
            pb.finish_and_clear();
        }

        // Print
        println!("{}", text);

        Ok(())
    } else {
        if let Some(pb) = pb {
            debug!("Stopping progress bar");
            pb.finish_and_clear();
        }

        bail!("Unsupported model kind: {}", model.kind)
    }
}

#[tokio::main]
async fn main() {
    if let Err(e) = run().await {
        error!("{} (main.rs)", e);
        std::process::exit(1);
    }
}
