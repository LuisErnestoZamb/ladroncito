use crate::services::{csv_loader::load_datastore, graph::Graph, load_config::load_config};

pub fn process_graph(mode: &str) {
    let config = load_config("config.yaml").expect("config error");
    let ds = load_datastore(&config.accounts_path, &config.transactions_path).unwrap();

    let start = &config.initial_wallet;
    let destination = &config.final_wallet;
    let depth = config.depth;

    let graph = Graph::from_transactions(&ds.transactions);

    let results_graph = match mode {
        "core" => graph.find_all_paths_extreme(start, destination, depth),
        "parallel" => graph.find_all_paths_parallel(start, destination, depth),
        "all" => graph.find_all_paths(start, destination, depth),
        _ => graph.find_all_paths_extreme(start, destination, depth),
    };

    if let Some(path) = graph.find_one_path(start, destination, depth) {
        println!("One path found:");
        println!("{:?}", path);

        graph
            .save_paths_to_file(&vec![path], &config.result_one_path)
            .expect("Error saving one path");
    } else {
        println!("No path found (single search)");
    }

    let all_paths = graph.find_all_paths(start, destination, depth);

    println!("Total paths found: {}", all_paths.len());

    graph
        .save_paths_to_file(&results_graph, &config.result_all_paths)
        .expect("Error saving all paths");
}

#[test]
fn test_full_flow() {
    let ds = load_datastore(
        "data/accounts_fake_test.csv",
        "data/transacciones_complejas.csv",
    )
    .unwrap();

    assert!(ds.accounts.len() > 0);
    assert!(ds.transactions.len() > 0);

    assert!(ds.transactions.len() > 0);
}

#[test]
fn test_graph() {
    let ds = load_datastore(
        "data/accounts_fake_test.csv",
        "data/transacciones_complejas.csv",
    )
    .unwrap();

    let start = &"TL0_Node7_x92Jk";
    let destination = &"TL4_Node5_wQ11";

    let graph = Graph::from_transactions(&ds.transactions);
    let all_paths = graph.find_all_paths(start, destination, 5);
    assert!(all_paths.len() == 1);
}
