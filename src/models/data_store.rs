use crate::models::{account::Account, transaction::Transaction};

#[derive(Debug)]
pub struct DataStore {
    pub accounts: Vec<Account>,
    pub transactions: Vec<Transaction>,
}

pub struct TransactionStore {
    pub transactions: Vec<Transaction>,
}
