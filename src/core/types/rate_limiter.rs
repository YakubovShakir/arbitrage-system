// pub struct Rateimiter {
//     pub limit_per_second: u8,
//     pub cur
// }

// use std::sync::Arc;
// use tokio::sync::Semaphore;
// use tokio::time::{interval, Duration};

// struct RateLimiter {
//     semaphore: Arc<Semaphore>,
// }

// impl RateLimiter {
//     async fn new(requests_per_second: usize) -> Self {
//         let semaphore = Arc::new(Semaphore::new(requests_per_second));

//         // Периодически добавляем новые токены
//         let semaphore_clone = Arc::clone(&semaphore);
//         tokio::spawn(async move {
//             let mut interval = interval(Duration::from_secs(1));
//             loop {
//                 interval.tick().await;
//                 // Добавляем новые разрешения
//                 semaphore_clone.add_permits(requests_per_second);
//             }
//         });

//         RateLimiter { semaphore }
//     }

//     async fn acquire(&self) {
//         let _permit = self.semaphore.acquire().await.unwrap();
//     }
// }
