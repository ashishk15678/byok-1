#[derive(Clone, Debug)]
pub struct AppState {
    pub opened_file: Option<String>,
    pub text: String,
}

impl AppState {
    pub fn new() -> Self {
        AppState {
            opened_file: None,
            text: "Hello World".to_string(),
        }
    }
}
