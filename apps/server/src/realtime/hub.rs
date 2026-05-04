use crate::realtime::events::WsEvent;
use std::sync::Arc;
use tokio::sync::broadcast;

pub type Hub = Arc<RealtimeHub>;

pub struct RealtimeHub {
    pub tx: broadcast::Sender<String>,
}

impl RealtimeHub {
    pub fn new(capacity: usize) -> Self {
        let (tx, _) = broadcast::channel(capacity);
        Self { tx }
    }

    pub fn subscribe(&self) -> broadcast::Receiver<String> {
        self.tx.subscribe()
    }

    pub fn publish(&self, event: WsEvent) {
        let _ = self.tx.send(event.to_json());
    }

    pub fn publish_to_channel(&self, _channel_id: &str, event: WsEvent) {
        self.publish(event);
    }
}

impl Default for RealtimeHub {
    fn default() -> Self {
        Self::new(1000)
    }
}
