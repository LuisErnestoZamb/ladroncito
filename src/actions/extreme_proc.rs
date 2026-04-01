use std::collections::HashSet;

use crate::services::graph::Graph;
use rayon::prelude::*;
use std::sync::{Arc, Mutex};

impl Graph {
    pub fn find_all_paths_extreme(
        &self,
        start: &str,
        destination: &str,
        max_depth: usize,
    ) -> Vec<Vec<String>> {
        let neighbors = match self.reverse.get(destination) {
            Some(n) => n,
            None => return vec![],
        };

        let results = Arc::new(Mutex::new(Vec::new()));

        neighbors.par_iter().for_each(|edge| {
            let visited = Arc::new(Mutex::new(HashSet::new()));
            let mut path = vec![destination.to_string()];

            visited.lock().unwrap().insert(destination.to_string());

            self.dfs_all_parallel_extreme(&edge.to, start, visited, &mut path, &results, max_depth);
        });

        Arc::try_unwrap(results).unwrap().into_inner().unwrap()
    }

    fn dfs_all_parallel_extreme(
        &self,
        current: &str,
        start: &str,
        visited: Arc<Mutex<HashSet<String>>>,
        path: &mut Vec<String>,
        results: &Arc<Mutex<Vec<Vec<String>>>>,
        max_depth: usize,
    ) {
        if path.len() > max_depth {
            return;
        }

        {
            let mut visited_guard = visited.lock().unwrap();
            visited_guard.insert(current.to_string());
        }

        path.push(current.to_string());

        if current == start {
            results.lock().unwrap().push(path.clone());
        } else if let Some(edges) = self.reverse.get(current) {
            // Paralelizar la exploración de hijos
            let path_clone = path.clone();
            let visited_clone = visited.clone();
            let results_clone = results.clone();

            edges.par_iter().for_each(|edge| {
                let mut new_path = path_clone.clone();
                let visited_guard = visited_clone.lock().unwrap();

                if !visited_guard.contains(&edge.to) {
                    let new_visited = Arc::new(Mutex::new(visited_guard.clone()));
                    drop(visited_guard);

                    self.dfs_all_parallel_extreme(
                        &edge.to,
                        start,
                        new_visited,
                        &mut new_path,
                        &results_clone,
                        max_depth,
                    );
                }
            });
        }

        path.pop();
        {
            let mut visited_guard = visited.lock().unwrap();
            visited_guard.remove(current);
        }
    }
}
