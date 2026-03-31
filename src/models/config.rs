use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub accounts_path: String,
    pub transactions_path: String,
    pub initial_wallet: String,
    pub final_wallet: String,
    pub depth: usize,
    pub result_one_path: String,
    pub result_all_paths: String,
}
