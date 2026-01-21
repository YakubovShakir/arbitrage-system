use json::JsonValue;
use reqwest::{
    Client, RequestBuilder, StatusCode,
    header::{HeaderMap, HeaderName, HeaderValue},
};
use std::{error::Error, str::FromStr, sync::Arc, time::Duration};
use tokio::sync::OnceCell;

use crate::{
    config::parameters::{
        ERROR_CODE, GLOBAL_HTTP_TIMEOUT_SECS, HTTP_MAX_POOL_IDLE_PER_HOST, HTTP_RETRY_AFTER_MILLIS,
        RESET_CODE,
    },
    core::types::KeyValue,
};

static GLOBAL_CLIENT: OnceCell<Arc<Client>> = OnceCell::const_new();

pub async fn get_global_client() -> &'static Arc<Client> {
    GLOBAL_CLIENT
        .get_or_init(|| async {
            Arc::new(
                Client::builder()
                    .timeout(std::time::Duration::from_secs(GLOBAL_HTTP_TIMEOUT_SECS))
                    .pool_max_idle_per_host(HTTP_MAX_POOL_IDLE_PER_HOST) // Переиспользует соединения!
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
    async fn build_get(
        &self,
        endpoint: &str,
        headers: HeaderMap,
        query: &[(&str, &str)],
        timeout: Option<Duration>,
    ) -> RequestBuilder {
        match timeout {
            Some(timeout) => get_global_client()
                .await
                .get(format!("{}{}", self._base_url, endpoint))
                .timeout(timeout)
                .headers(headers)
                .query(query),
            None => get_global_client()
                .await
                .get(format!("{}{}", self._base_url, endpoint))
                .headers(headers)
                .query(query),
        }
    }
    async fn is_success(
        status_code: StatusCode,
        text: &str,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        if !status_code.is_success() {
            return Err(format!("Error status code HTTP {} - {}", status_code, text).into());
        }
        Ok(true)
    }

    pub async fn get(
        &self,
        endpoint: &str,
        query: Option<&[(&str, &str)]>,
        headers: Option<&[(&str, &str)]>,
        timeout: Option<Duration>,
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

        let req_builder = self
            .build_get(endpoint, formated_headers.clone(), query, timeout)
            .await;

        // Отправляем запрос с полной обработкой ошибок
        let response = match req_builder.send().await {
            Ok(resp) => resp,
            Err(_) => {
                println!(
                    "{ERROR_CODE}[ERROR] Failed GET {}{}. Retry after {}ms..{RESET_CODE}",
                    self._base_url, endpoint, HTTP_RETRY_AFTER_MILLIS
                );
                tokio::time::sleep(std::time::Duration::from_millis(HTTP_RETRY_AFTER_MILLIS)).await;
                let req_builder = self
                    .build_get(endpoint, formated_headers.clone(), query, timeout)
                    .await;

                match req_builder.send().await {
                    Ok(res) => res,
                    Err(e) => {
                        println!(
                            "{ERROR_CODE}[ERROR] Failed GET after retry {}{} {RESET_CODE}",
                            self._base_url, endpoint
                        );
                        if e.is_timeout() {
                            eprintln!(
                                "{ERROR_CODE}[ERROR] Timeout error {}{} - {}{RESET_CODE}",
                                self._base_url, endpoint, e
                            );
                        }
                        return Err(Box::new(e));
                    }
                }
            }
        };
        let response_status = response.status();

        // Читаем ответ
        let text = if self._base_url.contains("huobi.pro") {
            // Для HTX читаем ответ побайтово
            self.read_chunked_response(response).await?
        } else {
            // Для других бирж - обычный способ
            response.text().await?
        };

        HttpClient::is_success(response_status, &text).await?;
        // Парсим JSON
        match json::parse(&text) {
            Ok(parsed) => Ok(parsed),
            Err(e) => Err(Box::new(e)),
        }
    }
    async fn read_chunked_response(
        &self,
        response: reqwest::Response,
    ) -> Result<String, Box<dyn std::error::Error>> {
        use flate2::read::GzDecoder;
        use std::io::Read;

        // 1. Собираем все байты
        let bytes = response.bytes().await?;

        // 2. Проверяем, сжат ли ответ
        let text = if bytes.len() > 2 && bytes[0] == 0x1f && bytes[1] == 0x8b {
            // GZIP compressed
            let mut decoder = GzDecoder::new(&bytes[..]);
            let mut decompressed = String::new();
            decoder.read_to_string(&mut decompressed)?;
            decompressed
        } else {
            // Not compressed
            String::from_utf8_lossy(&bytes).to_string()
        };

        // 3. Проверяем целостность JSON
        if !text.trim().ends_with("}]}") && text.contains("\"status\":\"ok\"") {
            // Ответ обрезан - логируем для отладки
            eprintln!("⚠️ HTX response may be truncated: {} chars", text.len());
            // Но продолжаем обработку - возможно, JSON валиден
        }

        Ok(text)
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
