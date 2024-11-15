mod hf;
mod together;

use anyhow::Result;

use crate::cli::Args;
use crate::config::Service;

pub use self::hf::HuggingFaceClient;
pub use self::together::TogetherClient;

#[async_trait::async_trait]
pub trait Client {
    // Only return `Self` if the trait is `Sized`
    fn new() -> Result<Self>
    where
        Self: Sized;

    async fn generate(
        &self,
        args: &Args,
    ) -> Result<Vec<u8>>;
}

/// Create a client based on the service
pub fn create_client(service: &Service) -> Result<Box<dyn Client>> {
    // The `dyn` keyword is used to create a trait object.
    // We return a boxed trait object for runtime polymorphism, so we can handle different types of clients.
    match service {
        Service::Hf => {
            let client = HuggingFaceClient::new()?;
            Ok(Box::new(client))
        }
        Service::Together => {
            let client = TogetherClient::new()?;
            Ok(Box::new(client))
        }
    }
}
