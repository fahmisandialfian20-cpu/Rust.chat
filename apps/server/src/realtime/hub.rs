use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{broadcast, RwLock};
use uuid::Uuid;

pub type Hub = Arc<RealtimeHub>;

pub struct RealtimeHub {
    channels: Arc<RwLock<HashMap<Uuid, broadcast::Sender<String>>>>,
    capacity: usize,
}

impl RealtimeHub {
    pub fn new(capacity: usize) -> Self {
        Self {
            channels: Arc::new(RwLock::new(HashMap::new())),
            capacity,
        }
    }

    pub async fn subscribe(&self, channel_id: Uuid) -> broadcast::Receiver<String> {
        let mut channels = self.channels.write().await;
        let sender = channels.entry(channel_id).or_insert_with(|| {
            let (tx, _) = broadcast::channel(self.capacity);
            tx
        });
        sender.subscribe()
    }

    pub async fn publish_to_channel(&self, channel_id: Uuid, event: impl Into<String>) {
        let channels = self.channels.read().await;
        if let Some(sender) = channels.get(&channel_id) {
            let _ = sender.send(event.into());
        }
    }
}

impl Default for RealtimeHub {
    fn default() -> Self {
        Self::new(1000)
    }
}
