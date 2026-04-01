use crate::services::{csv_loader::load_datastore, graph::Graph, load_config::load_config};

pub fn process_one_graph() {
    let config = load_config("config.yaml").expect("config error");
    let ds = load_datastore(&config.accounts_path, &config.transactions_path).unwrap();

    let start = &config.initial_wallet;
    let destination = &config.final_wallet;
    let depth = config.depth;

    let graph = Graph::from_transactions(&ds.transactions);

    println!("Forward nodes: {}", graph.forward.len());
    println!("Reverse nodes: {}", graph.reverse.len());

    if let Some(path) = graph.find_one_path(start, destination, depth) {
        println!("One path found:");
        println!("{:?}", path);

        graph
            .save_paths_to_file(&vec![path], &config.result_one_path)
            .expect("Error saving one path");
    } else {
        println!("No path found (single search)");
    }
}
