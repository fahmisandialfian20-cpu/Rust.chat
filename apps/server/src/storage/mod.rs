use async_trait::async_trait;

pub mod local;
pub mod provider;

#[async_trait]
pub trait FileStorage: Send + Sync {
    async fn upload(&self, key: &str, data: &[u8], content_type: &str) -> Result<(), String>;
    async fn download_url(&self, key: &str) -> Result<String, String>;
    async fn delete(&self, key: &str) -> Result<(), String>;
}
