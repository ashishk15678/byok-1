// pub mod implementations;
use gpui::actions;
pub mod config;
pub mod editor;
pub mod state;
pub mod utils;

use crate::config::{APP_NAME, BACKGROUND_COLOR, PRIMARY_COLOR};
use crate::editor::filebrowser::FileBrowser;
use crate::editor::texteditor::{
    Copy, Cut, NewFile, OpenFile, Paste, Redo, SaveFile, SaveFileAs, TextEditor, Undo,
};
use gpui::{
    App, Application, Bounds, Bounds as GpuiBounds, Context, Element, ElementId, Entity,
    FocusHandle, GlobalElementId, InputHandler, InspectorElementId, IntoElement, KeyBinding,
    KeyContext, LayoutId, Menu, MenuItem, MouseButton, Pixels, Point, Style, UTF16Selection,
    Window, WindowBounds, WindowOptions, div, prelude::*, px, rgb, size,
};
use std::cell::RefCell;
use std::env;
use std::fs;
use std::ops::Range;
use std::panic;
use std::path::PathBuf;
use std::rc::Rc;

struct MainScreen {
    editor: Entity<TextEditor>,
    file_browser: Entity<FileBrowser>,
    show_browser: bool,
    focus_handle: FocusHandle,
}

actions!(MainScreen, [Quit, ToggleBrowser]);

impl MainScreen {
    fn new(editor: Entity<TextEditor>, cx: &mut Context<Self>) -> Self {
        let editor_clone = editor.clone();
        let file_browser = cx.new(|cx| FileBrowser::new(editor_clone, cx));
        Self {
            editor,
            file_browser,
            show_browser: true, // Show file browser by default
            focus_handle: cx.focus_handle(),
        }
    }

    fn toggle_browser(&mut self, _: &ToggleBrowser, _window: &mut Window, cx: &mut Context<Self>) {
        self.show_browser = !self.show_browser;
        cx.notify();
    }
}

impl Render for TextEditor {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let text = self.get_text();
        let line_count = text.lines().count().max(1);

        div()
            .track_focus(&self.focus_handle)
            .on_action(cx.listener(TextEditor::new_file))
            .on_action(cx.listener(TextEditor::open_file))
            .on_action(cx.listener(TextEditor::save_file))
            .on_action(cx.listener(TextEditor::save_file_as))
            .on_action(cx.listener(TextEditor::undo))
            .on_action(cx.listener(TextEditor::redo))
            .on_action(cx.listener(TextEditor::cut))
            .on_action(cx.listener(TextEditor::copy))
            .on_action(cx.listener(TextEditor::paste))
            .key_context("Editor")
            .size_full()
            .flex()
            .flex_row()
            .child(
                div()
                    .w_16()
                    .h_full()
                    .bg(rgb(0x1e1e1e))
                    .border_r_1()
                    .border_color(rgb(0x404040))
                    .p_2()
                    .text_color(rgb(0x888888))
                    .text_sm()
                    .child(div().flex().flex_col().gap_1().children(
                        (1..=line_count).map(|i| div().text_right().child(i.to_string())),
                    )),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .flex_1()
                    .p_4()
                    .text_sm()
                    .text_color(rgb(0xcccccc))
                    .bg(rgb(0x252525))
                    .child(if text.is_empty() {
                        div().h_4().child("")
                    } else {
                        div().child(text)
                    })
                    .child(self.clone()), // Render the Element for input handling
            )
    }
}

impl Render for MainScreen {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .track_focus(&self.focus_handle)
            .on_action(cx.listener(MainScreen::toggle_browser))
            .on_action(cx.listener(|_, _: &Quit, _, cx| cx.quit()))
            .bg(BACKGROUND_COLOR)
            .text_color(PRIMARY_COLOR)
            .size_full()
            .flex()
            .flex_row()
            .when(self.show_browser, |this| {
                this.child(
                    div()
                        .flex()
                        .flex_1()
                        .size_full()
                        .child(self.file_browser.clone()),
                )
            })
            .child(
                // Editor view
                div().flex().flex_1().size_full().child(self.editor.clone()),
            )
            .when(self.show_browser, |this| {
                this.child(
                    div()
                        .w_1_4()
                        .h_full()
                        .bg(rgb(0x1e1e1e))
                        .border_l_1()
                        .border_color(rgb(0x404040))
                        .flex()
                        .flex_col()
                        .p_4()
                        .child(
                            div()
                                .text_color(rgb(0x888888))
                                .text_sm()
                                .child("Info Panel"),
                        ),
                )
            })
    }
}

fn main() {
    let env: Vec<String> = env::args().collect();
    Application::new().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(1000.), px(800.0)), cx);
        cx.activate(true);

        let menu = Menu {
            name: APP_NAME.into(),
            items: vec![
                MenuItem::Submenu(Menu {
                    name: "File".into(),
                    items: vec![
                        MenuItem::action("New", NewFile),
                        MenuItem::action("Open...", OpenFile),
                        MenuItem::action("Save", SaveFile),
                        MenuItem::action("Save As...", SaveFileAs),
                        MenuItem::Separator,
                        MenuItem::action("Quit", Quit),
                    ],
                }),
                MenuItem::Submenu(Menu {
                    name: "Edit".into(),
                    items: vec![
                        MenuItem::action("Undo", Undo),
                        MenuItem::action("Redo", Redo),
                        MenuItem::Separator,
                        MenuItem::action("Cut", Cut),
                        MenuItem::action("Copy", Copy),
                        MenuItem::action("Paste", Paste),
                    ],
                }),
                MenuItem::Submenu(Menu {
                    name: "View".into(),
                    items: vec![MenuItem::action("Toggle Browser", ToggleBrowser)],
                }),
            ],
        };

        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Maximized(bounds)),
                ..Default::default()
            },
            |window, cx| {
                cx.set_menus(vec![menu]);
                cx.bind_keys([
                    KeyBinding::new("ctrl-l", ToggleBrowser, None),
                    KeyBinding::new("ctrl-n", NewFile, Some("Editor")),
                    KeyBinding::new("ctrl-o", OpenFile, Some("Editor")),
                    KeyBinding::new("ctrl-s", SaveFile, Some("Editor")),
                    KeyBinding::new("ctrl-shift-s", SaveFileAs, Some("Editor")),
                    KeyBinding::new("ctrl-z", Undo, Some("Editor")),
                    KeyBinding::new("ctrl-shift-z", Redo, Some("Editor")),
                    KeyBinding::new("ctrl-x", Cut, Some("Editor")),
                    KeyBinding::new("ctrl-c", Copy, Some("Editor")),
                    KeyBinding::new("ctrl-v", Paste, Some("Editor")),
                ]);

                let editor = cx.new(|cx| TextEditor::new(cx));
                let editor_focus = editor.read(cx).focus_handle.clone();
                window.focus(&editor_focus);

                let main_screen = cx.new(|cx| MainScreen::new(editor, cx));
                main_screen
            },
        )
        .unwrap();
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::KeyBinding;

    #[test]
    fn test_keybinding_creation() {
        let _toggle_browser = KeyBinding::new("ctrl-l", ToggleBrowser, None);
        let _new_file = KeyBinding::new("ctrl-n", NewFile, Some("Editor"));
        assert!(true);
    }
}
