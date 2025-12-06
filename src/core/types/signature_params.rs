pub enum SignatureParams<'a> {
    Binance {
        query: &'a str,
    },
    Bybit {
        query: &'a str,
        timestamp: &'a str,
        recv_window: &'a str,
    },
    Bitget {
        query: &'a str,
        method: &'a str,
        endpoint: &'a str,
        timestamp: &'a str,
    },
    Gate {
        query: &'a str,
        method: &'a str,
        endpoint: &'a str,
        timestamp: &'a str,
        json_body: &'a str,
    },
    Kucoin {
        query: &'a str,
        method: &'a str,
        endpoint: &'a str,
        timestamp: &'a str,
    },
    Mexc {
        query: &'a str,
        string_body: &'a str,
    },
}
