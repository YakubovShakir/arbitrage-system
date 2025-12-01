use futures_util::{SinkExt, StreamExt};
use std::collections::VecDeque;
use std::pin::Pin;
use std::{sync::Arc, time::Duration};
use tokio::net::TcpStream;
use tokio::sync::{Mutex, RwLock};
use tokio::task::JoinHandle;
use tokio_tungstenite::tungstenite::http::Response;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};
use tokio_tungstenite::{connect_async, tungstenite::Message};

pub type BinaryMessageHandler = Arc<dyn Fn(prost::bytes::Bytes) -> Option<String> + Send + Sync>;

pub type ConnectionHandler = Arc<
    dyn Fn() -> Pin<
            Box<
                dyn Future<
                        Output = Result<
                            (
                                WebSocketStream<MaybeTlsStream<TcpStream>>,
                                Response<Option<Vec<u8>>>,
                            ),
                            Box<dyn std::error::Error + Send + Sync>,
                        >,
                    > + Send,
            >,
        > + Send
        + Sync,
>;

#[derive(Clone)]
pub struct WebSocketClient {
    connection_url: String,
    state: Arc<RwLock<Option<String>>>,
    pub state_is_streamed: bool,
    streamed_state: Arc<RwLock<Option<VecDeque<String>>>>,
    listen_handle: Arc<Mutex<Option<JoinHandle<()>>>>,
    ping_interval: Option<Duration>,
    ping_message: Option<String>,
    binary_handler: Option<BinaryMessageHandler>, // ← новый параметр
    connection_handler: Option<ConnectionHandler>,
}

impl WebSocketClient {
    pub fn new(url: &str) -> Self {
        WebSocketClient {
            connection_url: url.to_string(),
            state: Arc::new(RwLock::new(None)),
            state_is_streamed: false,
            streamed_state: Arc::new(RwLock::new(None)),
            listen_handle: Arc::new(Mutex::new(None)),
            ping_interval: None,
            ping_message: None,
            binary_handler: None,
            connection_handler: None,
        }
    }
    // Метод для установки ping интервала
    pub fn with_ping_interval(mut self, interval: Duration, ping_message: String) -> Self {
        self.ping_interval = Some(interval);
        self.ping_message = Some(ping_message);
        self
    }
    pub fn with_binary_handler(mut self, handler: BinaryMessageHandler) -> Self {
        self.binary_handler = Some(handler);
        self
    }
    pub fn with_connection_handler(mut self, handler: ConnectionHandler) -> Self {
        self.connection_handler = Some(handler);
        self
    }

    pub fn with_streamed_state(mut self) -> Self {
        self.state_is_streamed = true;
        self
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

        let (ws_stream, _) = match &self.connection_handler {
            Some(handler) => handler().await?,
            None => {
                let con = connect_async(&self.connection_url).await?;
                println!("WebSocket соединение c {} установлено", self.connection_url);
                con
            }
        };

        let (mut write, mut read) = ws_stream.split();

        // Подписываемся если нужно
        if let Some(msg) = subscribe_message {
            write
                .send(Message::Text(msg.into()))
                .await
                .map_err(|e| format!("Failed to send subscribe message: {}", e))?;
        }

        let state = self.state.clone();
        let streamed_state = self.streamed_state.clone();
        let ping_interval = self.ping_interval;
        let ping_message = self.ping_message.clone();

        let state_is_streamed = self.state_is_streamed;
        let con_url = self.connection_url.clone();

        let binary_handler = self.binary_handler.clone();
        // Запускаем задачу слушателя
        let handle = tokio::spawn(async move {
            // Используем Arc и Mutex для разделения write между задачами
            let write = Arc::new(Mutex::new(write));

            // Задача для отправки ping
            let ping_handle =
                if let (Some(interval), Some(ping_message)) = (ping_interval, ping_message) {
                    let write_for_ping = Arc::clone(&write);

                    Some(tokio::spawn(async move {
                        let mut interval_timer = tokio::time::interval(interval);
                        loop {
                            interval_timer.tick().await;
                            let mut write_guard = write_for_ping.lock().await;
                            // println!("Sending ping..");
                            if let Err(e) = write_guard
                                .send(Message::Text(ping_message.clone().into()))
                                .await
                            {
                                eprintln!("Failed to send ping: {}", e);
                                break;
                            }
                        }
                    }))
                } else {
                    None
                };

            // Основной цикл обработки сообщений
            while let Some(message_result) = read.next().await {
                match message_result {
                    Ok(Message::Text(text)) => {
                        if state_is_streamed {
                            let mut state_guard = streamed_state.write().await;
                            let vec = state_guard.get_or_insert_with(VecDeque::new);
                            vec.push_back(text.to_string());
                            if vec.len() > 500 {
                                vec.clear();
                            }
                        } else {
                            let mut state_guard = state.write().await;
                            *state_guard = Some(text.to_string());
                        }
                    }
                    Ok(Message::Ping(data)) => {
                        let mut write_guard = write.lock().await;
                        if let Err(e) = write_guard.send(Message::Pong(data)).await {
                            eprintln!("Failed to send pong: {}", e);
                            break;
                        }
                    }
                    Ok(Message::Close(frame)) => {
                        println!("Сервер {} закрыл соединение: {:?}", con_url, frame);
                        break;
                    }
                    Ok(Message::Binary(data)) => {
                        // Используем обработчик если он есть
                        if let Some(handler) = &binary_handler {
                            if let Some(processed_data) = handler(data) {
                                let mut state_guard = state.write().await;
                                *state_guard = Some(processed_data);
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("Ошибка WebSocket: {}", e);
                        break;
                    }
                    _ => {}
                }
            }

            // Останавливаем задачу ping при разрыве соединения
            if let Some(ping_handle) = ping_handle {
                ping_handle.abort();
            }

            println!("Задача слушателя завершена");
        });

        Ok(handle)
    }
    pub async fn get_state(&self) -> Option<String> {
        self.state.read().await.as_ref().cloned()
    }

    pub async fn get_streamed_state(&self) -> Option<VecDeque<String>> {
        self.streamed_state.read().await.as_ref().cloned()
    }
}
