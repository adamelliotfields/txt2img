use std::collections::HashMap;
use std::sync::OnceLock;

use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use serde_json::Value;
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

/// Get or initialize the default services configuration
pub fn get_or_init_services() -> &'static Services {
    static SERVICES: OnceLock<Services> = OnceLock::new();
    SERVICES.get_or_init(|| Services {
        default: DefaultService { id: ServiceId::Hf },
        hf: Service {
            id: ServiceId::Hf,
            default: DefaultModel {
                id: ModelId::Sd35LargeTurbo,
            },
            models: vec![
                Model {
                    id: ModelId::Sd35LargeTurbo,
                    name: "stabilityai/stable-diffusion-3.5-large-turbo".to_string(),
                    height: Some(1024),
                    width: Some(1024),
                    steps: Some(4),
                    cfg: None,
                    style: None,
                    negative_prompt: None,
                    options: None,
                },
                Model {
                    id: ModelId::Sd35Large,
                    name: "stabilityai/stable-diffusion-3.5-large".to_string(),
                    height: Some(1024),
                    width: Some(1024),
                    steps: Some(28),
                    cfg: Some(3.5),
                    style: None,
                    negative_prompt: None,
                    options: None,
                },
                Model {
                    id: ModelId::Sdxl,
                    name: "stabilityai/stable-diffusion-xl-base-1.0".to_string(),
                    height: Some(1024),
                    width: Some(1024),
                    steps: Some(50),
                    cfg: Some(7.5),
                    style: None,
                    negative_prompt: None,
                    options: None,
                },
                Model {
                    id: ModelId::FluxSchnell,
                    name: "black-forest-labs/FLUX.1-schnell".to_string(),
                    height: Some(1024),
                    width: Some(1024),
                    steps: Some(4),
                    cfg: None,
                    style: None,
                    negative_prompt: None,
                    options: None,
                },
                Model {
                    id: ModelId::FluxDev,
                    name: "black-forest-labs/FLUX.1-dev".to_string(),
                    height: Some(1024),
                    width: Some(1024),
                    steps: Some(28),
                    cfg: Some(3.0),
                    style: None,
                    negative_prompt: None,
                    options: None,
                },
            ],
        },
        openai: Service {
            id: ServiceId::Openai,
            default: DefaultModel { id: ModelId::Dalle3 },
            models: vec![
                Model {
                    id: ModelId::Dalle3,
                    name: "dall-e-3".to_string(),
                    height: Some(1024),
                    width: Some(1024),
                    steps: None,
                    cfg: None,
                    style: Some(OpenAIImageStyle::Vivid),
                    negative_prompt: None,
                    options: Some(HashMap::from([(
                        "quality".to_string(),
                        Value::String("standard".to_string()),
                    )])),
                },
                Model {
                    id: ModelId::Dalle2,
                    name: "dall-e-2".to_string(),
                    height: Some(1024),
                    width: Some(1024),
                    steps: None,
                    cfg: None,
                    style: None,
                    negative_prompt: None,
                    options: None,
                },
            ],
        },
        together: Service {
            id: ServiceId::Together,
            default: DefaultModel {
                id: ModelId::FluxSchnell,
            },
            models: vec![
                Model {
                    id: ModelId::FluxSchnell,
                    name: "black-forest-labs/FLUX.1-schnell-Free".to_string(),
                    height: Some(1024),
                    width: Some(1024),
                    steps: Some(4),
                    cfg: None,
                    style: None,
                    negative_prompt: None,
                    options: None,
                },
                Model {
                    id: ModelId::FluxDev,
                    name: "black-forest-labs/FLUX.1-dev".to_string(),
                    height: Some(1024),
                    width: Some(1024),
                    steps: Some(28),
                    cfg: Some(3.0),
                    style: None,
                    negative_prompt: None,
                    options: None,
                },
                Model {
                    id: ModelId::FluxPro,
                    name: "black-forest-labs/FLUX.1-pro".to_string(),
                    height: Some(1024),
                    width: Some(1024),
                    steps: Some(40),
                    cfg: Some(2.5),
                    style: None,
                    negative_prompt: None,
                    options: None,
                },
                Model {
                    id: ModelId::Flux11Pro,
                    name: "black-forest-labs/FLUX.1.1-pro".to_string(),
                    height: Some(1024),
                    width: Some(1024),
                    steps: None,
                    cfg: None,
                    style: None,
                    negative_prompt: None,
                    options: None,
                },
            ],
        },
    })
}
