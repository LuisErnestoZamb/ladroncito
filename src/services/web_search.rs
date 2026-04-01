use crate::services::{csv_loader::load_only_transactions, graph::Graph};

pub async fn sort_search(
    initial_wallet: &str,
    final_wallet: &str,
    job_id: &str,
) -> Result<Vec<String>, String> {
    let file_path = format!("/tmp/analysis_{}.csv", job_id);

    let ds =
        load_only_transactions(&file_path).map_err(|e| format!("Error cargando datos: {}", e))?;

    let graph = Graph::from_transactions(&ds.transactions);

    graph
        .find_one_path(initial_wallet, final_wallet, 5)
        .ok_or_else(|| {
            "No route was found between the wallets within the depth limit (5).".to_string()
        })
}
