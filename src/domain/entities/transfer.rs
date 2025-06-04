use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transfer {
    pub ts: u64,
    pub from: String,
    pub to: String,
    pub amount: f64,
    pub usd_price: f64,
}
