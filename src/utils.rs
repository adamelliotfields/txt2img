use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

use anyhow::{bail, Context, Result};
use indicatif::MultiProgress;
use indicatif_log_bridge::LogWrapper;
use log::debug;
use simplelog::{ColorChoice, Config as LogConfig, LevelFilter, TermLogger, TerminalMode};

/// Writes the image bytes to a file. The extension is inferred from the bytes.
pub fn write_image(
    path: &str,
    image_bytes: &[u8],
) -> Result<String> {
    // Get the base name of the file path
    let base = Path::new(path)
        .file_stem()
        .and_then(|s| s.to_str())
        .context(format!("`{}` is not a valid path (util.rs)", path))?;

    // https://github.com/bojand/infer#supported-types
    debug!("Inferring image type");
    let kind = infer::get(image_bytes).context("Infer couldn't detect the file type (util.rs)")?;
    let mime = kind.mime_type();

    if !kind.mime_type().starts_with("image/") {
        bail!(format!("Server sent {} for image (util.rs)", mime));
    }

    let ext = kind.extension();
    let file_path = format!("{}.{}", base, ext);
    let file =
        File::create(&file_path).context(format!("Couldn't write to {} (util.rs)", file_path))?;

    let mut writer = BufWriter::new(file);

    debug!("Writing bytes to disk");
    writer
        .write_all(image_bytes)
        .context("Unable to write image data (util.rs)")?;

    debug!("Flushing write buffer");
    writer
        .flush()
        .context("Failed to flush buffer (util.rs)")?;

    Ok(file_path)
}

/// Initialize the logger with optional debug level
pub fn init_logger(
    // Using `Into` so the caller doesn't have to wrap the value in `Some`
    is_debug: impl Into<Option<bool>> + Default,
) -> Result<MultiProgress> {
    let multi = MultiProgress::new();
    let debug = is_debug.into().unwrap_or_default(); // default for bool is false

    let logger = TermLogger::new(
        if debug {
            LevelFilter::Debug
        } else {
            LevelFilter::Warn
        },
        LogConfig::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    );

    let log_wrapper = LogWrapper::new(multi.clone(), logger);
    log_wrapper.try_init()?;
    Ok(multi)
}
