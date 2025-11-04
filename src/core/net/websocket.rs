use std::sync::{Arc, RwLock};

use futures_util::{
    SinkExt, StreamExt,
    stream::{SplitSink, SplitStream},
};
use tokio::net::TcpStream;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream, connect_async, tungstenite::Message};

#[derive(Debug)]
pub struct WebSocketClient {
    connection_url: String,
    state: Arc<RwLock<String>>,
    // stream: Option<WebSocketStream<MaybeTlsStream<TcpStream>>>,
    write: Option<SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>>,
    read: Option<SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>>,
}

impl WebSocketClient {
    pub fn new(url: &str) -> Self {
        WebSocketClient {
            connection_url: url.to_string(),
            state: Arc::new(RwLock::new(String::new())),
            write: None,
            read: None,
        }
    }

    pub async fn connect(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("Начинаю устанавливать соедиение с {}", &self.connection_url);
        match connect_async(&self.connection_url).await {
            Ok((stream, _)) => {
                println!(
                    "WebSocket соединение c {} установлено успешно",
                    &self.connection_url
                ); // Разделяем поток на чтение и запись
                let (write, read) = stream.split();
                self.write = Some(write);
                self.read = Some(read);

                Ok(())
            }
            Err(e) => {
                eprintln!("Ошибка подключения к {}: {}", self.connection_url, e);
                Err(e.into())
            }
        }
    }

    pub async fn send_message(&mut self, message: &str) -> Option<()> {
        self.write
            .as_mut()?
            .send(Message::Text(message.into()))
            .await
            .ok()?;
        Some(())
    }

    pub async fn read_message(&mut self) -> Option<String> {
        let message = self.read.as_mut()?.next().await?;

        match message {
            Ok(Message::Text(text)) => Some(text.to_string()),
            Ok(Message::Close(_)) => None,
            Err(_) => None,
            _ => None, // Пропускаем не-текстовые сообщения
        }
    }
}
