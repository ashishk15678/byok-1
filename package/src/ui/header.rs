use gpui::{
    Context, InteractiveElement, IntoElement, MouseButton, ParentElement, Render, Styled, Window,
    div, px, rgb,
};

pub struct Header {}

impl Header {
    pub fn new() -> Self {
        Self {}
    }

    pub fn render(
        &self,
        _window: &mut Window,
        _cx: &mut Context<crate::MainScreen>,
    ) -> impl IntoElement {
        div()
            .h_10()
            .w_full()
            .bg(rgb(0x252526))
            .border_b_1()
            .border_color(rgb(0x1e1e1e))
            .flex()
            .flex_row()
            .items_center()
            .px_2()
            .child(
                div()
                    .px_2()
                    .py_1()
                    .text_sm()
                    .rounded_md()
                    .cursor_pointer()
                    .child("Settings")
                    .on_mouse_down(
                        MouseButton::Left,
                        _cx.listener(|_, _, window, cx| {
                            window.dispatch_action(Box::new(crate::OpenSettings), cx);
                        }),
                    ),
            )
    }
}
