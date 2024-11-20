use anyhow::{Context, Result};
use clap::{ArgAction, Parser};
use strum::VariantNames;

use crate::config::{get_or_init_config, Model, ModelConfig, Service};

/// Command line arguments
#[derive(Parser, Debug)]
#[command(name = "gen", version, about = "Rusty image generation CLI", long_about = None)]
pub struct Args {
    /// The text to guide the generation (required)
    #[arg(required_unless_present_any = ["help", "list_models", "list_services", "version"])]
    pub prompt: Option<String>,

    /// Negative prompt
    #[arg(short = 'n', long)]
    pub negative_prompt: Option<String>,

    /// Model to use
    #[arg(short, long, hide_possible_values = true)]
    pub model: Option<Model>,

    /// Service to use
    #[arg(short, long, hide_possible_values = true)]
    pub service: Option<Service>,

    /// Seed for reproducibility
    #[arg(long)]
    pub seed: Option<u64>,

    /// Inference steps
    #[arg(long)]
    pub steps: Option<u32>,

    /// Guidance scale
    #[arg(long)]
    pub cfg: Option<f32>,

    /// Width of the image
    #[arg(long)]
    pub width: Option<u32>,

    /// Height of the image
    #[arg(long)]
    pub height: Option<u32>,

    /// Output file path
    #[arg(short, long, default_value = "image.jpg")]
    pub out: Option<String>,

    /// Suppress progress bar
    #[arg(short, long, action = ArgAction::SetTrue, conflicts_with = "debug")]
    pub quiet: bool,

    /// Use debug logging
    #[arg(long, action = ArgAction::SetTrue, conflicts_with = "quiet")]
    pub debug: bool,

    /// Print models
    #[arg(long, action = ArgAction::SetTrue, conflicts_with = "list_services")]
    pub list_models: bool,

    /// Print services
    #[arg(long, action = ArgAction::SetTrue, conflicts_with = "list_models")]
    pub list_services: bool,
}

// https://docs.rs/clap/latest/clap/struct.Arg.html#implementations
impl Args {
    /// Get the prompt
    pub fn get_prompt(&self) -> Result<Option<&str>> {
        // Validated by Clap
        Ok(self.prompt.as_deref())
    }

    /// Get the negative prompt or None
    pub fn get_negative_prompt(&self) -> Result<Option<&str>> {
        Ok(self
            .negative_prompt
            .as_deref()
            .or_else(|| {
                // Try the model config but don't error
                self.get_model_config()
                    .ok()?
                    .negative_prompt
                    .as_deref()
            }))
    }

    /// Get the models for the current service
    pub fn get_models(&self) -> Result<&Vec<ModelConfig>> {
        let config = get_or_init_config()?;
        match self.get_service()? {
            Service::Hf => Ok(&config.services.hf.models),
            Service::Together => Ok(&config.services.together.models),
        }
    }

    /// Get the model ID with default fallback
    pub fn get_model(&self) -> Result<&Model> {
        // Model is a ValueEnum validated by Clap
        if let Some(model) = &self.model {
            return Ok(model);
        }

        let config = get_or_init_config()?;
        match self.get_service()? {
            Service::Hf => Ok(&config.services.hf.default_model),
            Service::Together => Ok(&config.services.together.default_model),
        }
    }

    /// Get the model config for the current service
    pub fn get_model_config(&self) -> Result<&ModelConfig> {
        let model_id = self.get_model()?;
        let model_config = self
            .get_models()?
            .iter()
            .find(|m| m.id == *model_id) // deref to compare values not references
            .context(format!("Model `{}` not in config (cli.rs)", model_id))?;
        Ok(model_config)
    }

    /// Get the services
    pub fn get_services(&self) -> Result<&'static [&'static str]> {
        Ok(Service::VARIANTS)
    }

    /// Get the service or error if not supported
    pub fn get_service(&self) -> Result<&Service> {
        // Service is a ValueEnum validated by Clap
        if let Some(service) = &self.service {
            return Ok(service);
        }

        let config = get_or_init_config()?;
        Ok(&config.default_service)
    }

    /// Get the seed
    pub fn get_seed(&self) -> Result<Option<u64>> {
        // Numeric types, booleans, and chars implement the Copy trait.
        // They can be copied by duplicating bits in memory; they don't need to be dereferenced.
        // https://doc.rust-lang.org/std/marker/trait.Copy.html
        Ok(self.seed)
    }

    /// Get the number of steps or error if not configured
    pub fn get_steps(&self) -> Result<u32> {
        if let Some(steps) = self.steps {
            return Ok(steps);
        }
        let steps = self
            .get_model_config()?
            .steps
            .context("No default `steps` in config (cli.rs)")?;
        Ok(steps)
    }

    /// Get the guidance scale or error if not configured
    pub fn get_cfg(&self) -> Result<f32> {
        if let Some(cfg) = self.cfg {
            return Ok(cfg);
        }
        let cfg = self
            .get_model_config()?
            .cfg
            .context("No default `cfg` in config (cli.rs)")?;
        Ok(cfg)
    }

    /// Get the width or error if not configured
    pub fn get_width(&self) -> Result<u32> {
        if let Some(width) = self.width {
            return Ok(width);
        }
        let width = self
            .get_model_config()?
            .width
            .context("No default `width` in config (cli.rs)")?;
        Ok(width)
    }

    /// Get the height or error if not configured
    pub fn get_height(&self) -> Result<u32> {
        if let Some(height) = self.height {
            return Ok(height);
        }
        let height = self
            .get_model_config()?
            .height
            .context("No default `height` in config (cli.rs)")?;
        Ok(height)
    }

    /// Get the output file path
    pub fn get_out(&self) -> Result<&str> {
        Ok(self
            .out
            .as_deref()
            .unwrap_or("image.jpg"))
    }

    /// Get the quiet flag
    pub fn get_quiet(&self) -> Result<bool> {
        Ok(self.quiet)
    }

    /// Get the debug flag
    pub fn get_debug(&self) -> Result<bool> {
        Ok(self.debug)
    }

    /// Get the list models flag
    pub fn get_list_models(&self) -> Result<bool> {
        Ok(self.list_models)
    }

    /// Get the list services flag
    pub fn get_list_services(&self) -> Result<bool> {
        Ok(self.list_services)
    }
}
