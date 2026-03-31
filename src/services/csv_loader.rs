use crate::models::{
    account::Account,
    data_store::{DataStore, TransactionStore},
    transaction::Transaction,
};

use std::error::Error;

pub fn load_datastore(
    accounts_path: &str,
    transactions_path: &str,
) -> Result<DataStore, Box<dyn Error>> {
    let mut accounts_reader = csv::Reader::from_path(accounts_path)?;
    let mut transactions_reader = csv::Reader::from_path(transactions_path)?;

    let accounts: Vec<Account> = accounts_reader.deserialize().collect::<Result<_, _>>()?;
    let transactions: Vec<Transaction> = transactions_reader
        .deserialize()
        .collect::<Result<_, _>>()?;

    Ok(DataStore {
        accounts,
        transactions,
    })
}

pub fn load_only_transactions(transactions_path: &str) -> Result<TransactionStore, Box<dyn Error>> {
    let mut transactions_reader = csv::Reader::from_path(transactions_path)?;

    let transactions: Vec<Transaction> = transactions_reader
        .deserialize()
        .collect::<Result<_, _>>()?;

    Ok(TransactionStore { transactions })
}
