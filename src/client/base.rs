use crate::error::GenError;
use crate::Args;

#[async_trait::async_trait]
pub trait BaseClient {
    async fn predict(&self, args: &Args) -> Result<Vec<u8>, GenError>;
}
