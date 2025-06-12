use clickhouse::Row;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Row)]
pub struct UserStats {
    pub address: String,
    pub total_volume: f64,
    pub avg_buy_price: f64,
    pub avg_sell_price: f64,
    pub max_balance: f64,
}

impl UserStats {
    pub fn new(
        address: String,
        total_volume: f64,
        avg_buy_price: f64,
        avg_sell_price: f64,
        max_balance: f64,
    ) -> Self {
        Self {
            address,
            total_volume,
            avg_buy_price,
            avg_sell_price,
            max_balance,
        }
    }
}
