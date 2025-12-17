use gpui::{Context, Entity, IntoElement, MouseButton, Window, div, prelude::*, rgb};
use std::fs;
use std::path::PathBuf;

use crate::config::{DIR_ICON, FILE_ICON};
use crate::editor::texteditor::TextEditor;

pub struct FileBrowser {
    current_path: PathBuf,
    selected_file: Option<PathBuf>,
    editor: Entity<TextEditor>,
}

impl FileBrowser {
    pub fn new(editor: Entity<TextEditor>, cx: &mut Context<Self>) -> Self {
        Self {
            current_path: PathBuf::from("."),
            selected_file: None,
            editor,
        }
    }

    fn get_files(&self) -> Vec<PathBuf> {
        match fs::read_dir(&self.current_path) {
            Ok(entries) => {
                let mut files: Vec<PathBuf> =
                    entries.filter_map(|e| e.ok()).map(|e| e.path()).collect();
                files.sort();
                files
            }
            Err(_) => vec![],
        }
    }

    fn open_file(&mut self, path: PathBuf, cx: &mut Context<Self>) {
        if path.is_file() {
            self.selected_file = Some(path.clone());
            self.editor.update(cx, |editor, cx| {
                editor.open_file_from_path(path, cx);
            });
        } else if path.is_dir() {
            self.current_path = path;
            cx.notify();
        }
    }
}

impl Render for FileBrowser {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let files = self.get_files();
        let current_path_str = self.current_path.to_string_lossy().to_string();

        div()
            .flex()
            .flex_col()
            .w_64()
            .h_full()
            .bg(rgb(0x1e1e1e))
            .border_r_1()
            .border_color(rgb(0x404040))
            .px_2()
            .child(
                div()
                    .text_color(rgb(0x888888))
                    .text_xs()
                    .mb_2()
                    .child(format!("{} {}", DIR_ICON, current_path_str)),
            )
            .child(div().flex().flex_col().children(files.iter().map(|file| {
                let file_name = file
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("?")
                    .to_string();
                let is_dir = file.is_dir();
                let icon = if is_dir { DIR_ICON } else { FILE_ICON };
                let file_path = file.clone();

                div()
                    .px_2()
                    .py_1()
                    .rounded_md()
                    .text_color(rgb(0xcccccc))
                    .text_xs()
                    .cursor_pointer()
                    .hover(|d| d.bg(rgb(0x303030)))
                    .child(format!("{} {}", icon, file_name))
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener(move |this, _evt, _window, cx| {
                            this.open_file(file_path.clone(), cx);
                        }),
                    )
            })))
    }
}
