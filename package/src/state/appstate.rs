#[derive(Clone, Debug)]
pub struct AppState {
    pub opened_file: Option<String>,
    pub text: String,
    pub search_results: Vec<crate::editor::search::SearchResult>,
}

impl AppState {
    pub fn new() -> Self {
        AppState {
            opened_file: None,
            text: "Hello World".to_string(),
            search_results: Vec::new(),
        }
    }
}
