use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub accounts_path: String,
    pub transactions_path: String,
    pub initial_wallet: String,
    pub final_wallet: String,
}
