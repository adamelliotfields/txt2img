use std::collections::HashMap;
use std::sync::OnceLock;

use anyhow::{Context, Result};
use clap::ValueEnum;
use config::{Config, File, FileFormat};
use serde::{Deserialize, Serialize};
use strum::{Display, VariantNames};

/// Enum for supported OpenAI image styles
#[derive(Clone, Debug, Deserialize, Display, PartialEq, Serialize, ValueEnum, VariantNames)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum OpenAIImageStyle {
    Natural,
    Vivid,
}

/// Enum for supported models
#[derive(Clone, Debug, Deserialize, Display, PartialEq, Serialize, ValueEnum, VariantNames)]
#[strum(serialize_all = "kebab-case")]
#[serde(rename_all = "kebab-case")]
pub enum ModelId {
    Dalle2,
    Dalle3,
    Flux11Pro,
    FluxPro,
    FluxDev,
    FluxSchnell,
    Sd35Large,
    Sd35LargeTurbo,
    Sdxl,
}

/// Schema for a model configuration
#[derive(Debug, Deserialize, Serialize)]
pub struct Model {
    pub id: ModelId,
    pub name: String,
    // Image parameters
    pub height: Option<u16>,
    pub width: Option<u16>,
    pub cfg: Option<f32>,
    pub steps: Option<u8>,
    pub style: Option<OpenAIImageStyle>,
    pub negative_prompt: Option<String>,
    // Misc settings
    #[serde(default)]
    pub options: Option<HashMap<String, serde_json::Value>>,
}

/// Enum for supported services
#[derive(Clone, Debug, Deserialize, Display, PartialEq, Serialize, ValueEnum, VariantNames)]
#[strum(serialize_all = "kebab-case")]
#[serde(rename_all = "kebab-case")]
pub enum ServiceId {
    Hf,
    Openai,
    Together,
}

/// Default service configuration
#[derive(Debug, Deserialize, Serialize)]
pub struct DefaultService {
    pub id: ServiceId,
}

/// Schema for a service's default model
#[derive(Debug, Deserialize, Serialize)]
pub struct DefaultModel {
    pub id: ModelId,
}

/// Schema for a service configuration
#[derive(Debug, Deserialize, Serialize)]
pub struct Service {
    pub id: ServiceId,
    pub default: DefaultModel,
    pub models: Vec<Model>,
}

/// Service configurations
#[derive(Debug, Deserialize, Serialize)]
pub struct Services {
    pub default: DefaultService,
    pub hf: Service,
    pub openai: Service,
    pub together: Service,
}

// Thread-safe initialization
pub static SERVICES: OnceLock<Services> = OnceLock::new();

/// Initialize the global configuration
pub fn get_or_init_services() -> Result<&'static Services> {
    if SERVICES.get().is_none() {
        // Load services from services.toml
        // CLI arguments are parsed later and override these
        let services = Config::builder()
            .add_source(File::from_str(include_str!("services.toml"), FileFormat::Toml))
            .build()
            .context("Failed to load src/services.toml (services.rs)")?
            .try_deserialize::<Services>()
            .context("Failed to parse services (services.rs)")?;

        // Can safely unwrap because this only runs once
        SERVICES.set(services).unwrap();
    }

    // Can safely unwrap because we just set it
    Ok(SERVICES.get().unwrap())
}
