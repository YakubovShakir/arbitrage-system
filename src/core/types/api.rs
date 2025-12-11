use super::exchanges::Exchange;

pub enum API {
    GetTickers,
    GetNetworks,
    GetMarginInfo,
    GetOrderBook,
}

impl API {
    pub fn endpoint(&self, exchange: &Exchange) -> Option<&str> {
        match self {
            API::GetTickers => match exchange {
                Exchange::Binance(_) => None,
                Exchange::Bybit(_) => Some("/v5/market/tickers"),
                Exchange::Bitget(_) => Some("/api/v2/spot/market/tickers"),
                Exchange::Gate(_) => Some("/api/v4/spot/tickers"),
                Exchange::Kucoin(_) => None,
                Exchange::Mexc(_) => None,
                Exchange::Huobi(_) => Some("/market/tickers"),
            },
            API::GetMarginInfo => match exchange {
                Exchange::Binance(_) => Some("/sapi/v1/margin/allAssets"),
                Exchange::Bybit(_) => Some("/v5/spot-margin-trade/data"),
                Exchange::Bitget(_) => Some("/api/v2/margin/currencies"),
                Exchange::Gate(_) => Some("/api/v4/margin/uni/currency_pairs"),
                Exchange::Kucoin(_) => Some("/api/v3/currencies/"),
                Exchange::Mexc(_) => None,
                Exchange::Huobi(_) => Some("/v1/margin/loan-info"),
            },
            API::GetNetworks => match exchange {
                Exchange::Binance(_) => Some("/sapi/v1/capital/config/getall"),
                Exchange::Bybit(_) => Some("/v5/asset/coin/query-info"),
                Exchange::Bitget(_) => Some("/api/v2/spot/public/coins"),
                Exchange::Gate(_) => Some("/api/v4/wallet/currency_chains"),
                Exchange::Kucoin(_) => Some("/api/v3/currencies"),
                Exchange::Mexc(_) => Some("/api/v3/capital/config/getall"),
                Exchange::Huobi(_) => Some("/v2/reference/currencies"),
            },
            API::GetOrderBook => match exchange {
                Exchange::Binance(_) => Some("/api/v3/depth"),
                Exchange::Bybit(_) => Some("/v5/market/orderbook"),
                Exchange::Bitget(_) => Some("/api/v2/spot/market/orderbook"),
                Exchange::Gate(_) => Some("/api/v4/spot/order_book"),
                Exchange::Kucoin(_) => Some("/api/v3/market/orderbook/level2"),
                Exchange::Mexc(_) => Some("/api/v3/depth"),
                Exchange::Huobi(_) => Some("/market/depth"),
            },
        }
    }
}
