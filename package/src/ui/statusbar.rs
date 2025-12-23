use gpui::*;

pub struct StatusBar {
}

impl StatusBar {
    pub fn new() -> Self {
        Self {}
    }

    pub fn render(&self, _cx: &App) -> impl IntoElement {
        div()
            .h_6()
            .w_full()
            .bg(rgb(0x007acc)) // Status bar blue
            .text_color(rgb(0xffffff))
            .flex()
            .items_center()
            .px_2()
            .text_xs()
            .child("Ready")
    }
}
