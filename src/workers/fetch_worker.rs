use std::sync::Arc;

use crate::{
    config::USDT_LIMIT, core::calc_average_price, exchanges::TradingPair, traits::ExchangeAPI,
};

pub struct FetchWorker {
    id: i16,
    pairs: Vec<Arc<TradingPair>>,
    exchanges: Vec<Arc<dyn ExchangeAPI>>,
}
impl FetchWorker {
    pub fn new(
        id: i16,
        exchanges: Vec<Arc<dyn ExchangeAPI>>,
        pairs: Vec<Arc<TradingPair>>,
    ) -> Self {
        Self {
            id,
            pairs,
            exchanges,
        }
    }
    pub async fn run(&self) {
        loop {
            for pair in &self.pairs {
                for exchange in &self.exchanges {
                    if exchange.is_pair_available(pair.base().to_string() + pair.quote()) {
                        // println!(
                        //     // "{} : Пара {}/{} -> fetching orderbook from {}",
                        //     self.id,
                        //     pair.base(),
                        //     pair.quote(),
                        //     exchange.name()
                        // );
                        let orderbook =
                            match exchange.fetch_order_book(pair.base(), pair.quote()).await {
                                Ok(book) => book,
                                Err(e) => {
                                    eprintln!(
                                        "Ошибка получения стакана с {} для {}/{} - {}",
                                        exchange.name(),
                                        pair.base(),
                                        pair.quote(),
                                        e
                                    );
                                    continue;
                                }
                            };
                        let buy_price = match calc_average_price(&USDT_LIMIT, &orderbook.0) {
                            Some(price) => price,
                            None => {
                                eprintln!(
                                    "Ошибка вычисления средней цены покупки с {} для {}/{}",
                                    exchange.name(),
                                    pair.base(),
                                    pair.quote()
                                );
                                continue;
                            }
                        };
                        let sell_price = match calc_average_price(&USDT_LIMIT, &orderbook.1) {
                            Some(price) => price,
                            None => {
                                eprintln!(
                                    "Ошибка вычисления средней цены покупки с {} для {}/{}",
                                    exchange.name(),
                                    pair.base(),
                                    pair.quote()
                                );
                                continue;
                            }
                        };
                        pair.update_buy_price(exchange.name().to_string(), buy_price);
                        pair.update_sell_price(exchange.name().to_string(), sell_price);
                    };
                }
            }
            tokio::time::sleep(std::time::Duration::from_millis(5000)).await;
        }
    }
}
