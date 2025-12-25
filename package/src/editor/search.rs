use gpui::*;
use std::path::PathBuf;
use std::sync::Arc;
use crate::pools::Pools;

#[derive(Clone, Debug)]
pub struct SearchResult {
    pub path: PathBuf,
    pub line: usize,
    pub line_content: String,
}

// Simplified: just a helper to be called in a background task
pub fn perform_search(
    query: String,
    root_path: PathBuf,
    pools: Arc<Pools>,
) -> Vec<SearchResult> {
    // Simulate heavy work / actual search
    let mut results = Vec::new();
    if query.is_empty() {
        return results;
    }

    let mut stack = vec![root_path];

    while let Some(path) = stack.pop() {
        if path.is_dir() {
            if let Ok(entries) = pools.resources.list_dir(&path) {
                for entry in entries.flatten() {
                    stack.push(entry.path());
                }
            }
        } else if path.is_file() {
            if let Ok(content) = pools.resources.open_file(&path) {
                for (i, line) in content.lines().enumerate() {
                    if line.contains(&query) {
                        results.push(SearchResult {
                            path: path.clone(),
                            line: i + 1,
                            line_content: line.trim().to_string(),
                        });
                        if results.len() > 50 { return results; }
                    }
                }
            }
        }
    }

    results
}
