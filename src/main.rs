use ladroncito::services::{csv_loader::load_datastore, load_config::load_config};

fn main() {
    let config = load_config("config.yaml").expect("config error");
    let ds = load_datastore(&config.accounts_path, &config.transactions_path).unwrap();
}

#[test]
fn test_full_flow() {
    let ds = load_datastore("data/test_accounts.csv", "data/test_transactions.csv").unwrap();

    assert!(ds.accounts.len() > 0);
    assert!(ds.transactions.len() > 0);
}
