use std::{sync::Arc, time::Duration};
use tokio::{sync::RwLock, time::Instant};

pub struct CacheData<T> {
    value: RwLock<Option<Arc<T>>>,
    timestamp: RwLock<Option<Instant>>,
    ttl: Duration,
}

impl<T> CacheData<T> {
    pub fn new(ttl: Duration) -> Self {
        Self {
            value: RwLock::new(None),
            timestamp: RwLock::new(None),
            ttl: ttl,
        }
    }

    pub async fn get(&self) -> Option<Arc<T>> {
        // ВСЕГДА сначала timestamp, потом value (как в set)
        let timestamp_guard = self.timestamp.read().await;
        let value_guard = self.value.read().await;

        let value = value_guard.as_ref()?;
        let timestamp = timestamp_guard.as_ref()?;

        if timestamp.elapsed() > self.ttl {
            return None;
        }

        Some(value.clone())
    }

    pub async fn set(&self, value: T) {
        let mut timestamp_quard = self.timestamp.write().await;
        let mut value_quard = self.value.write().await;

        *value_quard = Some(Arc::new(value));
        *timestamp_quard = Some(Instant::now());
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
