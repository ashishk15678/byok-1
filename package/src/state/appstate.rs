use crate::pools::Pools;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct AppState {
    pub opened_file: Option<String>,
    pub text: String,
    pub search_results: Vec<crate::editor::search::SearchResult>,
    pub pools: Arc<Pools>,
}

impl AppState {
    pub fn new() -> Self {
        AppState {
            opened_file: None,
            // later gets changed when file opens , but if say file doesnot open then default state
            text: "Could not open file".to_string(),
            search_results: Vec::new(),
            pools: Arc::new(Pools::new()),
        }
    }
}
