use std::sync::Arc;
use tokio::sync::{Mutex, Notify};
use tokio::time::{Duration, interval};

#[derive(Debug, Clone)]
pub struct RateLimiter {
    /// Текущее количество доступных запросов
    available: Arc<Mutex<usize>>,
    /// Для уведомления о появлении новых запросов
    notify: Arc<Notify>,
    /// Максимальное количество запросов в секунду
    capacity: usize,
}

impl RateLimiter {
    pub fn new(capacity: usize) -> Self {
        let limiter = Self {
            available: Arc::new(Mutex::new(capacity)),
            notify: Arc::new(Notify::new()),
            capacity,
        };

        // Запускаем поток восстановления
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

                // Восстанавливаем количество до capacity
                let mut avail = available.lock().await;
                if *avail < capacity {
                    *avail = capacity;
                    println!("🔄 Восстановлено до {} запросов", capacity);

                    // Уведомляем ВСЕХ ожидающих, что появились запросы
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
