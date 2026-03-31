use crate::services::{csv_loader::load_datastore, graph::Graph, load_config::load_config};

pub fn process_graph() {
    let config = load_config("config.yaml").expect("config error");
    let ds = load_datastore(&config.accounts_path, &config.transactions_path).unwrap();

    let start = &config.initial_wallet;
    let destination = &config.final_wallet;

    let graph = Graph::from_transactions(&ds.transactions);

    println!("Forward nodes: {}", graph.forward.len());
    println!("Reverse nodes: {}", graph.reverse.len());

    if let Some(path) = graph.find_one_path(start, destination, 5) {
        println!("One path found:");
        println!("{:?}", path);

        graph
            .save_paths_to_file(&vec![path], "data/one_path.txt")
            .expect("Error saving one path");
    } else {
        println!("No path found (single search)");
    }

    let all_paths = graph.find_all_paths(start, destination, 5);

    println!("Total paths found: {}", all_paths.len());

    graph
        .save_paths_to_file(&all_paths, "data/all_paths.txt")
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
