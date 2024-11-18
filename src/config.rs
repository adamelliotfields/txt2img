use std::collections::HashMap;
use std::sync::OnceLock;

use anyhow::{Context, Result};
use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use strum::{Display, VariantNames};

// Schema for supported models
#[derive(Clone, Debug, Deserialize, Display, PartialEq, Serialize, ValueEnum, VariantNames)]
#[strum(serialize_all = "kebab-case")]
#[serde(rename_all = "kebab-case")]
pub enum Model {
    Sd35Large,
    Sd35LargeTurbo,
    Sdxl,
    FluxDev,
    FluxSchnell,
    Flux11Pro,
}

/// Schema for a model configuration
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

/// Schema for supported services
#[derive(Clone, Debug, Deserialize, Display, PartialEq, Serialize, ValueEnum, VariantNames)]
#[strum(serialize_all = "kebab-case")]
#[serde(rename_all = "kebab-case")]
pub enum Service {
    Hf,
    Together,
}

/// Schema for a for a service configuration
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
            .context("Failed to parse config - check the schemas (config.rs)")?;

        // Can safely unwrap because this only runs once
        CONFIG.set(config).unwrap();
    }

    // Can safely unwrap because we just set it
    Ok(CONFIG.get().unwrap())
}
