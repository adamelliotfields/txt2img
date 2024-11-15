use std::collections::HashMap;
use std::sync::OnceLock;

use anyhow::{Context, Result};
use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use strum::{Display, VariantNames};

// Supported models
#[derive(Clone, Debug, Deserialize, Display, PartialEq, Serialize, ValueEnum, VariantNames)]
#[strum(serialize_all = "kebab-case")]
#[serde(rename_all = "kebab-case")]
pub enum Model {
    Sd35Lg,
    Sd35LgTurbo,
    Sdxl,
    FluxDev,
    FluxSchnell,
    Flux11Pro,
}

/// Configuration for a model
#[derive(Debug, Deserialize, Serialize)]
pub struct ModelConfig {
    pub id: Model,
    pub name: String,
    pub height: Option<u32>,
    pub width: Option<u32>,
    pub cfg: Option<f32>,
    pub steps: Option<u32>,
    pub negative_prompt: Option<String>,
    #[serde(default)]
    pub options: Option<HashMap<String, serde_json::Value>>,
}

/// Supported services
#[derive(Clone, Debug, Deserialize, Display, PartialEq, Serialize, ValueEnum, VariantNames)]
#[strum(serialize_all = "kebab-case")]
#[serde(rename_all = "kebab-case")]
pub enum Service {
    Hf,
    Together,
}

/// Configuration for a service
#[derive(Debug, Deserialize, Serialize)]
pub struct ServiceConfig {
    pub id: Service,
    pub models: Vec<ModelConfig>,
    pub default_model: Model,
}

/// Service configurations
#[derive(Debug, Deserialize, Serialize)]
pub struct ServiceConfigs {
    pub hf: ServiceConfig,
    pub together: ServiceConfig,
}

/// App configuration
#[derive(Debug, Deserialize, Serialize)]
pub struct AppConfig {
    pub timeout: u64,
    pub services: ServiceConfigs,
    pub default_service: Service,
}

pub static CONFIG: OnceLock<AppConfig> = OnceLock::new();

// TODO: Override some defaults with environment variables
/// Initialize the global configuration
pub fn get_or_init_config() -> Result<&'static AppConfig> {
    if CONFIG.get().is_none() {
        // Load configuration from default.toml.
        // CLI arguments are parsed later and override these.
        let config = config::Config::builder()
            .add_source(config::File::from_str(
                include_str!("config.toml"),
                config::FileFormat::Toml,
            ))
            .build()
            .context("Failed to load config (config/mod.rs)")?
            .try_deserialize::<AppConfig>()
            .context("Failed to parse config (config/mod.rs)")?;

        // Can safely unwrap because this only runs once
        CONFIG.set(config).unwrap();
    }

    // Can safely unwrap because we just set it
    Ok(CONFIG.get().unwrap())
}
