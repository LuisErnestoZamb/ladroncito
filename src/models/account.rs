use crate::models::booleans::de_bool;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Account {
    pub id: String,
    pub wallet: String,
    #[serde(deserialize_with = "de_bool")]
    pub is_exchange: bool,
    #[serde(deserialize_with = "de_bool")]
    pub is_exchange_arkm: bool,
    #[serde(deserialize_with = "de_bool")]
    pub is_receiver: bool,
    pub total_usd_amount: f64,
}
