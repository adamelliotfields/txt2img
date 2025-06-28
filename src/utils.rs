use std::path::Path;
use std::time::Duration;

use anyhow::{bail, Context, Result};
use image::{load_from_memory, ImageFormat};
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use indicatif_log_bridge::LogWrapper;
use log::debug;
use simplelog::{ColorChoice, Config as LogConfig, LevelFilter, TermLogger, TerminalMode};

/// Writes the image bytes to a file
pub fn write_image(
    path: &str,
    image_bytes: &[u8],
) -> Result<String> {
    let base = Path::new(path)
        .file_stem()
        .and_then(|s| s.to_str())
        .context(format!("`{path}` is not a valid (util.rs)"))?;

    let ext = Path::new(path)
        .extension()
        .and_then(|e| e.to_str())
        .context(format!("`{path}` has no extension (util.rs)"))?;

    let (file, format) = match ext {
        "jpg" => (format!("{base}.jpg"), ImageFormat::Jpeg),
        "jpeg" => (format!("{base}.jpeg"), ImageFormat::Jpeg),
        "png" => (format!("{base}.png"), ImageFormat::Png),
        "webp" => (format!("{base}.webp"), ImageFormat::WebP),
        _ => bail!("Unsupported image format `{ext}` (util.rs)"),
    };

    debug!("Decoding {file}");
    let dynamic_image = load_from_memory(image_bytes).context("Failed to decode image (util.rs)")?;

    debug!("Writing {file} to disk");
    dynamic_image
        .save_with_format(&file, format)
        .context(format!("Failed to save image to {file} (util.rs)"))?;

    Ok(file)
}

/// Initialize the logger with debug level
pub fn init_logger(is_debug: bool) -> Result<MultiProgress> {
    let multi_progress = MultiProgress::new();
    let logger = TermLogger::new(
        if is_debug {
            LevelFilter::Debug
        } else {
            LevelFilter::Warn
        },
        LogConfig::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    );

    let log_wrapper = LogWrapper::new(multi_progress.clone(), logger);
    log_wrapper.try_init()?;

    Ok(multi_progress)
}

/// Create a progress bar if not in quiet mode
pub fn create_progress_bar(
    quiet: bool,
    multi_progress: &MultiProgress,
) -> Option<ProgressBar> {
    if !quiet {
        debug!("Starting progress bar");
        let pb = multi_progress.add(ProgressBar::new_spinner());
        pb.enable_steady_tick(Duration::from_millis(80));
        pb.set_style(
            // https://github.com/sindresorhus/cli-spinners/blob/main/spinners.json
            ProgressStyle::with_template("{spinner:.blue} {msg}")
                .unwrap()
                .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏", "✓"]),
        );
        Some(pb)
    } else {
        None
    }
}
