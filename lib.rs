use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;
use std::time::Duration;

pub struct AsyncExecutor {
    size: u32,
    handling_count: Arc<AtomicU32>,
}

impl AsyncExecutor {
    pub fn new(size: u32) -> Self {
        Self {
            size,
            handling_count: Arc::new(AtomicU32::new(0)),
        }
    }

    pub async fn spawn<F>(&self, f: F)
    where
        F: futures::Future + Send + 'static,
        F::Output: Send + 'static,
    {
        loop {
            let handling_count = self.handling_count.clone();
            if handling_count.load(Ordering::Relaxed) < self.size {
                handling_count.fetch_add(1, Ordering::Relaxed);

                tokio::spawn(async move {
                    f.await;
                    handling_count.fetch_sub(1, Ordering::Relaxed);
                });
                break;
            }

            tokio::time::sleep(Duration::from_millis(50)).await;
        }
    }
}
