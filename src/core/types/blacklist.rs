use dashmap::DashMap;
use std::collections::HashSet;

use crate::core::types::TradingPair;

pub struct Blacklist {
    pub buy_blacklist: DashMap<TradingPair, HashSet<String>>,
    pub sell_blacklist: DashMap<TradingPair, HashSet<String>>,
}

impl Blacklist {
    pub fn new() -> Self {
        Self {
            buy_blacklist: DashMap::<TradingPair, HashSet<String>>::new(),
            sell_blacklist: DashMap::<TradingPair, HashSet<String>>::new(),
        }
    }
    pub fn is_buy_blacklisted(&self, pair: &TradingPair, exchange: &str) -> bool {
        self.buy_blacklist
            .get(pair)
            .map(|set| set.contains(exchange))
            .unwrap_or(false)
    }
    pub fn is_sell_blacklisted(&self, pair: &TradingPair, exchange: &str) -> bool {
        self.sell_blacklist
            .get(pair)
            .map(|set| set.contains(exchange))
            .unwrap_or(false)
    }
    pub fn blacklist_buy(&self, pair: &TradingPair, exchange: String) {
        self.buy_blacklist
            .entry(pair.clone())
            .or_insert_with(HashSet::new)
            .insert(exchange);
    }
    pub fn blacklist_sell(&self, pair: &TradingPair, exchange: String) {
        self.sell_blacklist
            .entry(pair.clone())
            .or_insert_with(HashSet::new)
            .insert(exchange);
    }
}
