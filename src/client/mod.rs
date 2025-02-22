mod hf;
mod openai;
mod together;

use anyhow::Result;

use crate::cli::Cli;
use crate::services::ServiceId;

pub use self::hf::HuggingFaceClient;
pub use self::openai::OpenAIClient;
pub use self::together::TogetherClient;

#[async_trait::async_trait]
pub trait Client {
    // The where clause prevents `new` from being called on trait objects (e.g., `dyn Client`).
    // Trait objects are unsized, and returning `Self` requires the size to be known at compile-time.
    fn new(timeout: u64) -> Result<Self>
    where
        Self: Sized;

    async fn generate_image(
        &self,
        cli: &Cli,
    ) -> Result<Vec<u8>>;

    async fn generate_text(
        &self,
        cli: &Cli,
    ) -> Result<String>;
}

/// Create a client based on the service
pub fn create_client(
    service: &ServiceId,
    timeout: &u64,
    // The `dyn` keyword is used to create a trait object.
    // We return a boxed trait object for runtime polymorphism, so we can handle different types of clients.
) -> Result<Box<dyn Client>> {
    // Dereference the timeout
    let timeout_value = *timeout;
    match service {
        ServiceId::Hf => {
            let client = HuggingFaceClient::new(timeout_value)?;
            Ok(Box::new(client))
        }
        ServiceId::Openai => {
            let client = OpenAIClient::new(timeout_value)?;
            Ok(Box::new(client))
        }
        ServiceId::Together => {
            let client = TogetherClient::new(timeout_value)?;
            Ok(Box::new(client))
        }
    }
}
