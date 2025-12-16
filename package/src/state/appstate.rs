use gpui::{Context, IntoElement, ParentElement, Render, Styled, Window, div, rgb};

struct AppState {
    opened_file: Option<String>,
    text: String,
}

impl AppState {
    fn new() -> Self {
        AppState {
            opened_file: None,
            text: "Hello World".to_string(),
        }
    }
}

impl Render for AppState {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .bg(rgb(0x2e7d32))
            .size_full()
            .justify_center()
            .items_center()
            .text_xl()
            .text_color(rgb(0xffffff))
            .child("Set Menus Example")
    }
}
