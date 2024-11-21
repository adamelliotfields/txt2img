use std::sync::OnceLock;

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

/// App configuration
#[derive(Debug, Deserialize, Serialize)]
pub struct AppConfig {
    pub timeout: Option<u64>,
}

// Thread-safe lazy initialization
pub static CONFIG: OnceLock<AppConfig> = OnceLock::new();

/// Initialize the global configuration
pub fn get_or_init_config() -> Result<&'static AppConfig> {
    if CONFIG.get().is_none() {
        // Load configuration from config.toml
        // CLI arguments are parsed later and override these
        let config = config::Config::builder()
            .add_source(config::File::from_str(
                include_str!("config.toml"),
                config::FileFormat::Toml,
            ))
            .build()
            .context("Failed to load src/config.toml (config.rs)")?
            .try_deserialize::<AppConfig>()
            .context("Failed to parse config (config.rs)")?;

        // Can safely unwrap because this only runs once
        CONFIG.set(config).unwrap();
    }

    // Can safely unwrap because we just set it
    Ok(CONFIG.get().unwrap())
}
