// src/config/mexc_protocol_buffers/mod.rs

use prost::Message;

// Импортируем сгенерированный код через корневой proto модуль
use crate::proto::generated::*;

// Реэкспортируем для удобства использования
pub use crate::proto::generated::{PublicMiniTickerV3Api, PublicMiniTickersV3Api};

pub fn decode_message(data: &[u8]) -> Result<PushDataV3ApiWrapper, prost::DecodeError> {
    PushDataV3ApiWrapper::decode(data)
}

pub fn handle_protobuf_message(message: PushDataV3ApiWrapper) -> Vec<PublicMiniTickerV3Api> {
    let mut tickers = Vec::new();

    if let Some(data) = message.body {
        match data {
            push_data_v3_api_wrapper::Body::PublicMiniTickers(tickers_data) => {
                // println!(
                //     "📊 Получены мини-тикеры, количество: {} ",
                //     tickers_data.items.len(),
                // );
                tickers = tickers_data.items;
            }
            _ => {
                println!("📨 Другой тип сообщения from Mexc binary");
            }
        }
    }

    tickers
}
