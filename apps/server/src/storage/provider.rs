use std::sync::Arc;

use super::local::LocalStorage;
use super::FileStorage;

pub fn create_storage_provider(
    provider: &str,
    local_dir: &str,
    base_url: &str,
) -> Arc<dyn FileStorage> {
    match provider {
        "local" => {
            let path = std::path::PathBuf::from(local_dir);
            Arc::new(LocalStorage::new(path, base_url.to_string()))
        }
        _ => {
            let path = std::path::PathBuf::from(local_dir);
            Arc::new(LocalStorage::new(path, base_url.to_string()))
        }
    }
}
