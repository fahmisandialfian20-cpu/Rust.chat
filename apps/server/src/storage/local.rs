use std::path::PathBuf;
use async_trait::async_trait;
use tokio::fs;
use tokio::io::AsyncWriteExt;

use super::FileStorage;

pub struct LocalStorage {
    base_path: PathBuf,
    base_url: String,
}

impl LocalStorage {
    pub fn new(base_path: PathBuf, base_url: String) -> Self {
        Self { base_path, base_url }
    }

    fn full_path(&self, key: &str) -> PathBuf {
        self.base_path.join(key)
    }
}

#[async_trait]
impl FileStorage for LocalStorage {
    async fn upload(&self, key: &str, data: &[u8], _content_type: &str) -> Result<(), String> {
        let path = self.full_path(key);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).await.map_err(|e| e.to_string())?;
        }
        let mut file = fs::File::create(&path).await.map_err(|e| e.to_string())?;
        file.write_all(data).await.map_err(|e| e.to_string())?;
        Ok(())
    }

    async fn download_url(&self, key: &str) -> Result<String, String> {
        Ok(format!("{}/files/{}", self.base_url.trim_end_matches('/'), key))
    }

    async fn delete(&self, key: &str) -> Result<(), String> {
        let path = self.full_path(key);
        fs::remove_file(path).await.map_err(|e| e.to_string())
    }
}
