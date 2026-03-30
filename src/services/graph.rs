use crate::models::transaction::Transaction;
use std::collections::HashMap;

// ---- Nodo destino + referencia a transacción ----
#[derive(Debug, Clone)]
pub struct Edge {
    pub to: String,
    pub tx_index: usize, // índice en ds.transactions
}

// ---- Grafo completo ----
#[derive(Debug)]
pub struct Graph {
    // wallet_from -> edges
    pub forward: HashMap<String, Vec<Edge>>,

    // wallet_to -> edges (inverso)
    pub reverse: HashMap<String, Vec<Edge>>,
}

impl Graph {
    pub fn new() -> Self {
        Self {
            forward: HashMap::new(),
            reverse: HashMap::new(),
        }
    }

    // ---- construir desde datastore ----
    pub fn from_transactions(transactions: &[Transaction]) -> Self {
        let mut graph = Graph::new();

        for (i, tx) in transactions.iter().enumerate() {
            // forward: from -> to
            graph
                .forward
                .entry(tx.wallet_from.clone())
                .or_default()
                .push(Edge {
                    to: tx.wallet_to.clone(),
                    tx_index: i,
                });

            // reverse: to -> from
            graph
                .reverse
                .entry(tx.wallet_to.clone())
                .or_default()
                .push(Edge {
                    to: tx.wallet_from.clone(),
                    tx_index: i,
                });
        }

        graph
    }
}
