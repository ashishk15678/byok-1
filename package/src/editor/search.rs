use gpui::*;
use std::path::PathBuf;
use std::sync::Arc;

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
) -> Vec<SearchResult> {
    // Simulate heavy work / actual search
    let mut results = Vec::new();
    if query.is_empty() {
        return results;
    }

    if let Ok(entries) = std::fs::read_dir(&root_path) {
        for entry in entries.flatten() {
             let path = entry.path();
             if path.is_file() {
                 if let Ok(content) = std::fs::read_to_string(&path) {
                     for (i, line) in content.lines().enumerate() {
                         if line.contains(&query) {
                             results.push(SearchResult {
                                 path: path.clone(),
                                 line: i + 1,
                                 line_content: line.trim().to_string(),
                             });
                             if results.len() > 10 { break; } // limit
                         }
                     }
                 }
             }
        }
    }

    results
}
