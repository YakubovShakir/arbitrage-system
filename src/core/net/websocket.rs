use futures_util::{SinkExt, StreamExt};
use std::{sync::Arc, time::Duration};
use tokio::sync::{Mutex, RwLock};
use tokio::task::JoinHandle;
use tokio_tungstenite::{connect_async, tungstenite::Message};

#[derive(Debug, Clone)]
pub struct WebSocketClient {
    connection_url: String,
    state: Arc<RwLock<Option<String>>>,
    listen_handle: Arc<Mutex<Option<JoinHandle<()>>>>,
}

impl WebSocketClient {
    pub fn new(url: &str) -> Self {
        WebSocketClient {
            connection_url: url.to_string(),
            state: Arc::new(RwLock::new(None)),
            listen_handle: Arc::new(Mutex::new(None)),
        }
    }

    pub async fn run_with_reconnect(&self, subscribe_message: Option<&str>) -> ! {
        loop {
            // Отменяем старую задачу если есть
            {
                let mut handle_guard = self.listen_handle.lock().await;
                if let Some(handle) = handle_guard.take() {
                    handle.abort();
                }
            }

            // Подключаемся
            match self.connect_and_listen(subscribe_message).await {
                Ok(handle) => {
                    {
                        let mut handle_guard = self.listen_handle.lock().await;
                        *handle_guard = Some(handle);
                    }

                    // Создаем клон для использования в select
                    let listen_handle_clone = self.listen_handle.clone();

                    // Ждем либо час, либо пока задача не завершится
                    tokio::select! {
                        _ = tokio::time::sleep(Duration::from_secs(3600)) => {
                            println!("Принудительное переподключение через 1 час");
                            // Продолжаем цикл
                        }
                        _ = Self::wait_for_task_completion(listen_handle_clone) => {
                            println!("Соединение разорвано, переподключаемся...");
                            // Продолжаем цикл переподключения
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Ошибка подключения: {}. Повтор через 5 сек.", e);
                    tokio::time::sleep(Duration::from_secs(5)).await;
                }
            }
        }
    }

    // Вспомогательный метод для ожидания завершения задачи
    async fn wait_for_task_completion(listen_handle: Arc<Mutex<Option<JoinHandle<()>>>>) {
        let mut handle_guard = listen_handle.lock().await;
        if let Some(handle) = handle_guard.take() {
            // Теперь у нас владение handle, можем его awaitить
            let _ = handle.await;
        }
    }

    async fn connect_and_listen(
        &self,
        subscribe_message: Option<&str>,
    ) -> Result<JoinHandle<()>, Box<dyn std::error::Error + Send + Sync>> {
        println!("Подключаемся к {}", self.connection_url);

        let (ws_stream, _) = connect_async(&self.connection_url).await?;
        println!("WebSocket соединение установлено");

        let (mut write, mut read) = ws_stream.split();

        // Подписываемся если нужно
        if let Some(msg) = subscribe_message {
            write
                .send(Message::Text(msg.into()))
                .await
                .map_err(|e| format!("Failed to send subscribe message: {}", e))?;
        }

        let state = self.state.clone();
        let mut write_clone = write;

        // Запускаем задачу слушателя
        let handle = tokio::spawn(async move {
            while let Some(message_result) = read.next().await {
                match message_result {
                    Ok(Message::Text(text)) => {
                        let mut state_guard = state.write().await;
                        *state_guard = Some(text.to_string());
                        println!("💾 [WebSocket] State обновлен");
                    }
                    Ok(Message::Ping(data)) => {
                        if let Err(e) = write_clone.send(Message::Pong(data)).await {
                            eprintln!("Failed to send pong: {}", e);
                            break;
                        }
                    }
                    Ok(Message::Close(frame)) => {
                        println!("Сервер закрыл соединение: {:?}", frame);
                        break;
                    }
                    Err(e) => {
                        eprintln!("Ошибка WebSocket: {}", e);
                        break;
                    }
                    _ => {}
                }
            }
            println!("Задача слушателя завершена");
        });

        Ok(handle)
    }

    pub async fn get_state(&self) -> Option<String> {
        let state_guard = self.state.read().await;
        state_guard.clone()
    }
}
