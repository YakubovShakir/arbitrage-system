pub type Price = f64;
pub type Quantity = f64;
pub type OrderBookLevel = (Price, Quantity);
pub type Glass = Vec<OrderBookLevel>;
pub type Asks = Glass;
pub type Bids = Glass;
pub type OrderBook = (Asks, Bids);
