use std::collections::HashSet;

use crate::services::graph::Graph;
use rayon::prelude::*;

impl Graph {
    pub fn find_all_paths_parallel(
        &self,
        start: &str,
        destination: &str,
        max_depth: usize,
    ) -> Vec<Vec<String>> {
        let neighbors = match self.reverse.get(destination) {
            Some(n) => n,
            None => return vec![],
        };

        let results: Vec<Vec<Vec<String>>> = neighbors
            .par_iter()
            .map(|edge| {
                let mut path = vec![destination.to_string()];
                let mut visited = HashSet::new();
                visited.insert(destination.to_string());
                self.dfs_collect(&edge.to, start, &mut visited, &mut path, max_depth)
            })
            .collect();

        results.into_iter().flatten().collect()
    }

    fn dfs_collect(
        &self,
        current: &str,
        start: &str,
        visited: &mut HashSet<String>,
        path: &mut Vec<String>,
        max_depth: usize,
    ) -> Vec<Vec<String>> {
        if path.len() > max_depth {
            return vec![];
        }

        visited.insert(current.to_string());
        path.push(current.to_string());

        let mut results = Vec::new();

        if current == start {
            results.push(path.clone());
        } else if let Some(edges) = self.reverse.get(current) {
            for edge in edges {
                if !visited.contains(&edge.to) {
                    results.extend(self.dfs_collect(&edge.to, start, visited, path, max_depth));
                }
            }
        }

        path.pop();
        visited.remove(current);

        results
    }
}
