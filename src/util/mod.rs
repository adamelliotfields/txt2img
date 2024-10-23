use crate::error::GenError;

use regex::Regex;
use std::fs::File;
use std::io::Write;

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
