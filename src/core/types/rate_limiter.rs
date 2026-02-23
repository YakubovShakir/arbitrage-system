use std::sync::Arc;
use tokio::sync::{Mutex, Notify};
use tokio::time::{Duration, interval};

#[derive(Debug, Clone)]
pub struct RateLimiter {
    available: Arc<Mutex<usize>>,
    notify: Arc<Notify>,
    capacity: usize,
}

impl RateLimiter {
    pub fn new(capacity: usize) -> Self {
        let limiter = Self {
            available: Arc::new(Mutex::new(capacity)),
            notify: Arc::new(Notify::new()),
            capacity,
        };

        limiter.start_refill();

        limiter
    }

    fn start_refill(self: &Self) {
        let available = Arc::clone(&self.available);
        let notify = Arc::clone(&self.notify);
        let capacity = self.capacity;

        tokio::spawn(async move {
            let mut interval = interval(Duration::from_secs(1));

            loop {
                interval.tick().await;

                let mut avail = available.lock().await;
                if *avail < capacity {
                    *avail = capacity;
                    // println!("🔄 Восстановлено до {} запросов", capacity);
                    notify.notify_waiters();
                }
            }
        });
    }

    /// Получить доступ с ожиданием
    pub async fn acquire(&self) {
        loop {
            // Проверяем текущее количество
            let mut avail = self.available.lock().await;

            if *avail > 0 {
                // Есть доступные запросы - используем один
                *avail -= 1;
                println!("✅ Запрос разрешен. Осталось: {}", *avail);
                return;
            }

            // Нет доступных запросов - будем ждать
            println!("⏳ Нет доступных запросов, ожидание...");

            // Важно: отпускаем блокировку перед ожиданием!
            let notify = self.notify.notified();
            drop(avail); // Освобождаем Mutex
            notify.await; // Ждем уведомления о восстановлении
        }
    }

    /// Попытаться получить доступ без ожидания
    pub async fn try_acquire(&self) -> bool {
        let mut avail = self.available.lock().await;
        if *avail > 0 {
            *avail -= 1;
            true
        } else {
            false
        }
    }

    /// Узнать текущее количество доступных запросов
    pub async fn available(&self) -> usize {
        *self.available.lock().await
    }

    /// Проверить, есть ли доступные запросы
    pub async fn has_available(&self) -> bool {
        self.available().await > 0
    }
}
