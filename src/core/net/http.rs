use json::JsonValue;
use reqwest::{
    Client,
    header::{HeaderMap, HeaderName, HeaderValue},
};
use std::{error::Error, str::FromStr, sync::Arc};
use tokio::sync::OnceCell;

use crate::core::types::KeyValue;

static GLOBAL_CLIENT: OnceCell<Arc<Client>> = OnceCell::const_new();

pub async fn get_global_client() -> &'static Arc<Client> {
    GLOBAL_CLIENT
        .get_or_init(|| async {
            Arc::new(
                Client::builder()
                    .timeout(std::time::Duration::from_secs(30))
                    .pool_max_idle_per_host(20) // Переиспользует соединения!
                    .build()
                    .unwrap(),
            )
        })
        .await
}
#[derive(Debug, Clone)]
pub struct HttpClient {
    _base_url: String,
}

impl HttpClient {
    pub fn new(base_url: &str) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            _base_url: base_url.to_string(),
        })
    }
    pub async fn get(
        &self,
        endpoint: &str,
        query: Option<&[(&str, &str)]>,
        headers: Option<&[(&str, &str)]>,
    ) -> Result<JsonValue, Box<dyn std::error::Error>> {
        let query = query.unwrap_or(&[]);
        let mut formated_headers = HeaderMap::new();

        match headers {
            Some(pairs) => {
                for pair in pairs {
                    formated_headers.insert(
                        HeaderName::from_str(pair.0)?,
                        HeaderValue::from_str(pair.1)?,
                    );
                }
            }
            None => (),
        };

        let req_builder = get_global_client()
            .await
            .get(format!("{}{}", self._base_url, endpoint))
            .headers(formated_headers)
            .query(query);

        // Отправляем запрос с полной обработкой ошибок
        let response = match req_builder.send().await {
            Ok(resp) => resp,
            Err(e) => {
                if e.is_timeout() {
                    eprintln!(
                        "Timeout error - увеличьте таймаут - {}{}",
                        self._base_url, endpoint
                    );
                }
                return Err(Box::new(e));
            }
        };

        if !response.status().is_success() {
            let status = response.status();
            // Пытаемся прочитать тело ошибки
            let error_body = response.text().await.unwrap_or_default();

            return Err(format!("Error status code HTTP {}: {}", status, error_body).into());
        }

        // Читаем ответ
        let text = response.text().await?;

        // Парсим JSON
        match json::parse(&text) {
            Ok(parsed) => Ok(parsed),
            Err(e) => Err(Box::new(e)),
        }
    }

    pub async fn post(
        &self,
        endpoint: &str,
        query: Option<&[KeyValue]>,
        headers: Option<&[KeyValue]>,
        body: Option<&[KeyValue]>, // Добавляем параметр body
    ) -> Result<JsonValue, Box<dyn std::error::Error>> {
        let query = query.unwrap_or(&[]);
        let mut formated_headers = HeaderMap::new();

        // Добавляем заголовки
        if let Some(pairs) = headers {
            for pair in pairs {
                formated_headers.insert(
                    HeaderName::from_str(pair.0.as_str())?,
                    HeaderValue::from_str(pair.1.as_str())?,
                );
            }
        }

        let mut req_builder = get_global_client()
            .await
            .post(format!("{}{}", self._base_url, endpoint))
            .headers(formated_headers)
            .query(query);

        // Добавляем body, если оно есть
        if let Some(body_params) = body {
            let form_body = body_params
                .iter()
                .map(|(k, v)| format!("{}={}", k, v))
                .collect::<Vec<_>>()
                .join("&");

            req_builder = req_builder.body(form_body);
        }

        // Отправляем запрос
        let response = req_builder.send().await?;

        if !response.status().is_success() {
            let status = response.status();
            let error_body = response.text().await.unwrap_or_default();
            return Err(format!("Error status code HTTP {}: {}", status, error_body).into());
        }

        let text = response.text().await?;
        match json::parse(&text) {
            Ok(parsed) => Ok(parsed),
            Err(e) => Err(Box::new(e)),
        }
    }
}
