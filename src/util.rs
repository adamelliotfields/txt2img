use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

use anyhow::{bail, Context, Result};

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

    // Infer the image type from the bytes
    // https://github.com/bojand/infer#supported-types
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
    writer
        .write_all(image_bytes)
        .context("Unable to write image data (util.rs)")?;
    writer
        .flush()
        .context("Failed to flush buffer (util.rs)")?;

    Ok(file_path)
}
