use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Transaction {
    pub amount: f64,
    pub wallet_from: String,
    pub wallet_to: String,
    pub hash_tx: String,
    pub direction: String,
    pub wallet_from_id: String,
    pub wallet_to_id: String,
}
