use gpui::*;
use gpui::prelude::*;

pub struct CommandPalette {
    pub focus_handle: FocusHandle,
}

impl CommandPalette {
    pub fn new(cx: &mut Context<Self>) -> Self {
        Self {
            focus_handle: cx.focus_handle(),
        }
    }
}

impl Render for CommandPalette {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .track_focus(&self.focus_handle)
            .shadow_lg()
            .absolute()
            .top(px(40.0))
            .left_1_2()
            .ml(px(-250.0))
            .w(px(500.0))
            .bg(rgb(0x252526))
            .border_1()
            .border_color(rgb(0x454545))
            .child(
                div()
                    .p_2()
                    .child("Type a command...")
            )
    }
}
