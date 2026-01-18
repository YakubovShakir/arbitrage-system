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

use crate::config::parameters::{ERROR_CODE, INFO_CODE, RESET_CODE, SUCCESS_CODE};

pub type BinaryMessageHandler = Arc<dyn Fn(prost::bytes::Bytes) -> Option<String> + Send + Sync>;
pub type PingPongHandler = Arc<dyn Fn(&str) -> Option<String> + Send + Sync>;

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
    binary_handler: Option<BinaryMessageHandler>,
    connection_handler: Option<ConnectionHandler>,
    ping_pong_handler: Option<PingPongHandler>, // ← новый обработчик ping/pong
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
            ping_pong_handler: None,
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

    // Новый метод для установки обработчика ping/pong
    pub fn with_ping_pong_handler(mut self, handler: PingPongHandler) -> Self {
        self.ping_pong_handler = Some(handler);
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
                            println!("{INFO_CODE}[INFO] Принудительное переподключение через 1 час{RESET_CODE}");
                            // Продолжаем цикл
                        }
                        _ = Self::wait_for_task_completion(listen_handle_clone) => {
                            println!("{INFO_CODE}[INFO] Соединение разорвано, переподключаемся...{RESET_CODE}");
                            // Продолжаем цикл переподключения
                        }
                    }
                }
                Err(e) => {
                    println!(
                        "{ERROR_CODE}[ERROR] Ошибка подключения: {}. Повтор через 5 сек.{RESET_CODE}",
                        e
                    );
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
        println!(
            "{INFO_CODE}[INFO] 📡 Подключаемся к {}{RESET_CODE}",
            self.connection_url
        );

        let (ws_stream, _) = match &self.connection_handler {
            Some(handler) => handler().await?,
            None => {
                let con = connect_async(&self.connection_url).await?;
                println!(
                    "{SUCCESS_CODE}[INFO] 🔗 WebSocket соединение c {} установлено{RESET_CODE}",
                    self.connection_url
                );
                con
            }
        };

        let (mut write, mut read) = ws_stream.split();

        // Подписываемся если нужно
        if let Some(msg) = subscribe_message {
            write.send(Message::Text(msg.into())).await.map_err(|e| {
                format!(
                    "{ERROR_CODE}[ERROR] Failed to send subscribe message: {}{RESET_CODE}",
                    e
                )
            })?;
        }

        let state = self.state.clone();
        let streamed_state = self.streamed_state.clone();
        let ping_interval = self.ping_interval;
        let ping_message = self.ping_message.clone();

        let state_is_streamed = self.state_is_streamed;
        let con_url = self.connection_url.clone();

        let binary_handler = self.binary_handler.clone();
        let ping_pong_handler = self.ping_pong_handler.clone();

        // Запускаем задачу слушателя
        let handle = tokio::spawn(async move {
            // Используем Arc и Mutex для разделения write между задачами
            let write = Arc::new(Mutex::new(write));

            // Задача для отправки ping
            let ping_handle = if let (Some(interval), Some(ping_message)) =
                (ping_interval, ping_message)
            {
                let write_for_ping = Arc::clone(&write);

                Some(tokio::spawn(async move {
                    let mut interval_timer = tokio::time::interval(interval);
                    loop {
                        interval_timer.tick().await;
                        let mut write_guard = write_for_ping.lock().await;
                        if let Err(e) = write_guard
                            .send(Message::Text(ping_message.clone().into()))
                            .await
                        {
                            eprintln!("{ERROR_CODE}[ERROR] Failed to send ping: {}{RESET_CODE}", e);
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
                        // Проверяем, не является ли это ping-сообщением
                        let mut is_ping_pong = false;

                        if let Some(handler) = &ping_pong_handler {
                            if let Some(response) = handler(&text) {
                                // Отправляем ответный pong
                                let mut write_guard = write.lock().await;
                                println!(
                                    "{INFO_CODE}[INFO] Получили ping от {} отпраляем ответку{RESET_CODE}",
                                    con_url
                                );
                                if let Err(e) =
                                    write_guard.send(Message::Text(response.into())).await
                                {
                                    eprintln!(
                                        "{ERROR_CODE}[ERROR]Failed to send pong response: {}{RESET_CODE}",
                                        e
                                    );
                                    break;
                                }
                                is_ping_pong = true;
                            }
                        }

                        // Если это было ping/pong сообщение, не сохраняем его в state
                        if !is_ping_pong {
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
                    }
                    Ok(Message::Ping(data)) => {
                        let mut write_guard = write.lock().await;
                        if let Err(e) = write_guard.send(Message::Pong(data)).await {
                            eprintln!("{ERROR_CODE}[ERROR] Failed to send pong: {}{RESET_CODE}", e);
                            break;
                        }
                    }
                    Ok(Message::Close(frame)) => {
                        println!(
                            "{INFO_CODE}[INFO] Сервер {} закрыл соединение: {:?}{RESET_CODE}",
                            con_url, frame
                        );
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
                        eprintln!("{ERROR_CODE}[ERROR] Ошибка WebSocket: {}{RESET_CODE}", e);
                        break;
                    }
                    _ => {}
                }
            }

            // Останавливаем задачу ping при разрыве соединения
            if let Some(ping_handle) = ping_handle {
                ping_handle.abort();
            }

            println!("{INFO_CODE}[INFO] Задача слушателя завершена{RESET_CODE}");
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
