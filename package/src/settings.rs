use gpui::{
    div, px, rgb, IntoElement, Render, Styled, Window, Context, InteractiveElement, ParentElement,
    EventEmitter,
};

pub struct SettingsView;

impl SettingsView {
    pub fn new() -> Self {
        Self
    }
}

impl Render for SettingsView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(rgb(0x1e1e1e))
            .text_color(rgb(0xcccccc))
            .p_4()
            .gap_4()
            .child(
                div()
                    .text_xl()
                    .font_weight(gpui::FontWeight::BOLD)
                    .child("Settings")
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_2()
                    .child(
                         div().child("Font Size: 16 (Not editable yet)")
                    )
                    .child(
                         div().child("Theme: Dark (Not editable yet)")
                    )
            )
    }
}
