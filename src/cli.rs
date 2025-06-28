use std::sync::LazyLock;

use anyhow::{Context, Result};
use clap::{ArgAction, ArgGroup, Parser};
use colored::Colorize;
use strum::VariantNames;

use crate::services::{get_or_init_services, Model, ModelId, OpenAIImageStyle, ServiceId};

const PARAMETERS: &str = "Parameters";

// Lazy initialization so we can style the text with Colorize instead of hard-coding ANSI codes
pub static AFTER_HELP: LazyLock<String> = LazyLock::new(|| {
    format!(
        "{}\n  {}\n  {}\n  {}",
        "Environment Variables:".bold().underline(),
        "HF_TOKEN                 Required for Hugging Face",
        "OPENAI_API_KEY           Required for OpenAI",
        "TOGETHER_API_KEY         Required for Together.ai",
    )
});

/// Command line interface
#[derive(Parser, Debug)]
#[command(
    name = "txt2img",
    version,
    about = "Text-to-image generation with cloud models.",
    after_help = AFTER_HELP.as_str(), // same as `&*`
)]
#[command(group(
    ArgGroup::new("parameters")
        .args(["negative_prompt", "width", "height", "cfg", "steps", "seed", "style", "out"])
        .multiple(true)
        .required(false),
))]
pub struct Cli {
    /// The text to guide the generation (required)
    #[arg(required_unless_present_any = ["help", "list_models", "list_services", "version"])]
    pub prompt: Option<String>,

    /// Model to use
    #[arg(short, long, hide_possible_values = true)]
    pub model: Option<ModelId>,

    /// Service to use
    #[arg(short, long, hide_possible_values = true)]
    pub service: Option<ServiceId>,

    /// Timeout in seconds
    #[arg(short, long, default_value_t = 60)] // use default_value_t for numeric or other types
    pub timeout: u64, // passed to Duration::from_secs

    /// Suppress progress
    #[arg(short, long, action = ArgAction::SetTrue, conflicts_with = "debug")]
    pub quiet: bool,

    /// Debug logging
    #[arg(long, action = ArgAction::SetTrue, conflicts_with = "quiet")]
    pub debug: bool,

    /// Print models
    #[arg(long, action = ArgAction::SetTrue, conflicts_with = "list_services")]
    pub list_models: bool,

    /// Print services
    #[arg(long, action = ArgAction::SetTrue, conflicts_with = "list_models")]
    pub list_services: bool,

    /// Negative prompt
    #[arg(
        long,
        short = 'n', // single quotes for char literal
        help_heading = PARAMETERS,
    )]
    pub negative_prompt: Option<String>,

    /// Inference steps
    #[arg(long, help_heading = PARAMETERS)]
    pub steps: Option<u8>,

    /// Classifier-free guidance scale
    #[arg(long, help_heading = PARAMETERS)]
    pub cfg: Option<f32>, // half-precision (f16) isn't supported yet

    /// Width of the image
    #[arg(long, help_heading = PARAMETERS)]
    pub width: Option<u16>,

    /// Height of the image
    #[arg(long, help_heading = PARAMETERS)]
    pub height: Option<u16>,

    /// Seed for reproducibility
    #[arg(long, help_heading = PARAMETERS)]
    pub seed: Option<u64>,

    /// Image style (OpenAI only)
    #[arg(
        long,
        value_enum,
        default_value = "vivid",
        help_heading = PARAMETERS
    )]
    pub style: OpenAIImageStyle,

    /// Output file path
    #[arg(
        short,
        long,
        default_value = "image.png",
        help_heading = PARAMETERS
    )] // use default_value for strings
    pub out: String,
}

// https://docs.rs/clap/latest/clap/struct.Arg.html#implementations
impl Cli {
    /// Get the prompt
    pub fn get_prompt(&self) -> Result<Option<&str>> {
        // Validated by Clap
        Ok(self.prompt.as_deref())
    }

    /// Get the negative prompt or None
    pub fn get_negative_prompt(&self) -> Result<Option<&str>> {
        Ok(self.negative_prompt.as_deref().or_else(|| {
            // Try the model config but don't error
            self.get_model().ok()?.negative_prompt.as_deref()
        }))
    }

    /// Get the models for the current service
    pub fn get_models(&self) -> Result<&Vec<Model>> {
        let services = get_or_init_services()?;
        match self.get_service()? {
            ServiceId::Hf => Ok(&services.hf.models),
            ServiceId::Openai => Ok(&services.openai.models),
            ServiceId::Together => Ok(&services.together.models),
        }
    }

    /// Get the model config for the current service
    pub fn get_model(&self) -> Result<&Model> {
        // let model_id = self.get_model_id()?;
        let model_id = if let Some(model) = &self.model {
            model
        } else {
            let services = get_or_init_services()?;
            match self.get_service()? {
                ServiceId::Hf => &services.hf.default.id,
                ServiceId::Openai => &services.openai.default.id,
                ServiceId::Together => &services.together.default.id,
            }
        };
        let model = self
            .get_models()?
            .iter()
            .find(|m| m.id == *model_id) // deref to compare values not references
            .context(format!("Unsupported model `{model_id}` (cli.rs)"))?;
        Ok(model)
    }

    /// Get the services
    pub fn get_services(&self) -> Result<&'static [&'static str]> {
        Ok(ServiceId::VARIANTS)
    }

    /// Get the service
    pub fn get_service(&self) -> Result<&ServiceId> {
        // Service is a ValueEnum validated by Clap
        if let Some(service) = &self.service {
            return Ok(service);
        }

        let services = get_or_init_services()?;
        Ok(&services.default.id)
    }

    /// Get the number of steps
    pub fn get_steps(&self) -> Result<u8> {
        if let Some(steps) = self.steps {
            return Ok(steps);
        }
        let steps = self.get_model()?.steps.unwrap();
        Ok(steps)
    }

    /// Get the guidance scale
    pub fn get_cfg(&self) -> Result<f32> {
        if let Some(cfg) = self.cfg {
            return Ok(cfg);
        }
        let cfg = self.get_model()?.cfg.unwrap();
        Ok(cfg)
    }

    /// Get the width
    pub fn get_width(&self) -> Result<u16> {
        if let Some(width) = self.width {
            return Ok(width);
        }
        let width = self.get_model()?.width.unwrap();
        Ok(width)
    }

    /// Get the height
    pub fn get_height(&self) -> Result<u16> {
        if let Some(height) = self.height {
            return Ok(height);
        }
        let height = self.get_model()?.height.unwrap();
        Ok(height)
    }
}
