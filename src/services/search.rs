use std::collections::HashSet;

use crate::services::graph::Graph;
use rayon::prelude::*;
use std::sync::{
    Arc, Mutex,
    atomic::{AtomicBool, Ordering},
};

impl Graph {
    pub fn find_one_path(
        &self,
        start: &str,
        destination: &str,
        max_depth: usize,
    ) -> Option<Vec<String>> {
        let neighbors = match self.reverse.get(destination) {
            Some(n) => n,
            None => return None,
        };

        let found = Arc::new(AtomicBool::new(false));
        let result = Arc::new(Mutex::new(None));

        neighbors.par_iter().for_each(|edge| {
            if found.load(Ordering::Relaxed) {
                return;
            }

            let mut visited = HashSet::new();
            let mut path = vec![destination.to_string()];
            visited.insert(destination.to_string());

            if let Some(p) = self.dfs_find_one_parallel(
                &edge.to,
                start,
                &mut visited,
                &mut path,
                max_depth,
                &found,
            ) {
                let mut res = result.lock().unwrap();
                *res = Some(p);
                found.store(true, Ordering::Relaxed);
            }
        });

        result.lock().unwrap().clone()
    }

    fn dfs_find_one_parallel(
        &self,
        current: &str,
        start: &str,
        visited: &mut HashSet<String>,
        path: &mut Vec<String>,
        max_depth: usize,
        found: &AtomicBool,
    ) -> Option<Vec<String>> {
        if found.load(Ordering::Relaxed) {
            return None;
        }

        if path.len() > max_depth {
            return None;
        }

        visited.insert(current.to_string());
        path.push(current.to_string());

        if current == start {
            return Some(path.clone());
        }

        if let Some(edges) = self.reverse.get(current) {
            for edge in edges {
                if !visited.contains(&edge.to) {
                    if let Some(p) =
                        self.dfs_find_one_parallel(&edge.to, start, visited, path, max_depth, found)
                    {
                        return Some(p);
                    }
                }
            }
        }

        path.pop();
        visited.remove(current);
        None
    }

    pub fn find_all_paths(
        &self,
        start: &str,
        destination: &str,
        max_depth: usize,
    ) -> Vec<Vec<String>> {
        let neighbors = match self.reverse.get(destination) {
            Some(n) => n,
            None => return vec![],
        };

        neighbors
            .par_iter()
            .map(|edge| {
                let mut results = Vec::new();
                let mut visited = HashSet::new();
                let mut path = vec![destination.to_string()];
                visited.insert(destination.to_string());

                self.dfs_all(
                    &edge.to,
                    start,
                    &mut visited,
                    &mut path,
                    &mut results,
                    max_depth,
                );

                results
            })
            .flatten()
            .collect()
    }

    fn dfs_all(
        &self,
        current: &str,
        start: &str,
        visited: &mut HashSet<String>,
        path: &mut Vec<String>,
        results: &mut Vec<Vec<String>>,
        max_depth: usize,
    ) {
        if path.len() > max_depth {
            return;
        }

        visited.insert(current.to_string());
        path.push(current.to_string());

        if current == start {
            results.push(path.clone());
        } else if let Some(edges) = self.reverse.get(current) {
            for edge in edges {
                if !visited.contains(&edge.to) {
                    self.dfs_all(&edge.to, start, visited, path, results, max_depth);
                }
            }
        }

        path.pop();
        visited.remove(current);
    }

    pub fn save_paths_to_file(
        &self,
        paths: &[Vec<String>],
        file_path: &str,
    ) -> std::io::Result<()> {
        use std::fs::File;
        use std::io::Write;

        let mut file = File::create(file_path)?;

        for path in paths {
            let line = path.join(" -> ");
            writeln!(file, "{}", line)?;
        }

        Ok(())
    }
}
