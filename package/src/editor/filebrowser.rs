use gpui::{
    Context, Entity, IntoElement, MouseButton, MouseDownEvent, MouseUpEvent, Render, Window, div,
    prelude::*, px, rgb,
};
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

use crate::config::{DIR_ICON, FILE_ICON};
use crate::editor::texteditor::TextEditor;

pub struct FileBrowser {
    root_path: PathBuf,
    selected_file: Option<PathBuf>,
    expanded_paths: HashSet<PathBuf>,
    // State for resizing
    width: f32,
    is_resizing: bool,
}

impl FileBrowser {
    pub fn new(_cx: &mut Context<Self>) -> Self {
        let root = PathBuf::from(".");
        let mut expanded = HashSet::new();
        expanded.insert(root.clone()); // Expand root by default

        Self {
            root_path: root,
            selected_file: None,
            expanded_paths: expanded,
            width: 256.0,
            is_resizing: false,
        }
    }

    fn toggle_expand(&mut self, path: PathBuf, cx: &mut Context<Self>) {
        if self.expanded_paths.contains(&path) {
            self.expanded_paths.remove(&path);
        } else {
            self.expanded_paths.insert(path);
        }
        cx.notify();
    }

    fn open_file(&mut self, path: PathBuf, window: &mut Window, cx: &mut Context<Self>) {
        if path.is_file() {
            self.selected_file = Some(path.clone());
            window.dispatch_action(Box::new(crate::OpenPath { path }), cx);
        }
    }

    fn render_tree(
        &self,
        path: &Path,
        depth: usize,
        cx: &mut Context<Self>,
    ) -> Vec<impl IntoElement> {
        let mut elements = Vec::new();

        if let Ok(entries) = fs::read_dir(path) {
            let mut files: Vec<PathBuf> =
                entries.filter_map(|e| e.ok()).map(|e| e.path()).collect();

            // Sort directories first, then files
            files.sort_by(|a, b| {
                let a_is_dir = a.is_dir();
                let b_is_dir = b.is_dir();
                if a_is_dir == b_is_dir {
                    a.cmp(b)
                } else if a_is_dir {
                    std::cmp::Ordering::Less
                } else {
                    std::cmp::Ordering::Greater
                }
            });

            for file_path in files {
                let file_name = file_path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("?")
                    .to_string();

                let is_dir = file_path.is_dir();
                let is_expanded = self.expanded_paths.contains(&file_path);

                let icon = if is_dir {
                    if is_expanded { DIR_ICON } else { DIR_ICON }
                } else {
                    FILE_ICON
                };
                // let display_icon = if is_dir { DIR_ICON } else { FILE_ICON }; // Removed redundant variable

                let padding = 10.0 + (depth as f32 * 15.0);
                let path_clone = file_path.clone();
                let path_clone_2 = file_path.clone();

                let element = div()
                    .pl(px(padding))
                    .pr_2()
                    .py_1()
                    .text_color(rgb(0xcccccc))
                    .text_xs()
                    .hover(|d| d.bg(rgb(0x303030)))
                    .cursor_pointer()
                    .child(format!("{} {}", icon, file_name))
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener(move |this, _, window, cx| {
                            if is_dir {
                                this.toggle_expand(path_clone.clone(), cx);
                            } else {
                                this.open_file(path_clone.clone(), window, cx);
                            }
                        }),
                    );

                elements.push(element.into_any_element());

                if is_dir && is_expanded {
                    elements.extend(
                        self.render_tree(&path_clone_2, depth + 1, cx)
                            .into_iter()
                            .map(|e| e.into_any_element()),
                    );
                }
            }
        }
        elements
    }
}

impl Render for FileBrowser {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let root_path_str = self.root_path.to_string_lossy().to_string();

        div()
            .relative()
            .flex()
            .h_full()
            .on_mouse_up(
                MouseButton::Left,
                cx.listener(|this, _: &MouseUpEvent, _, cx| {
                    this.is_resizing = false;
                    cx.notify();
                }),
            )
            .child(
                // The Sidebar Content
                div()
                    .w(px(self.width))
                    .h_full()
                    .bg(rgb(0x1e1e1e))
                    .border_r_1()
                    .border_color(rgb(0x404040))
                    .flex()
                    .flex_col()
                    .overflow_hidden()
                    .child(
                        div()
                            .px_2()
                            .py_2()
                            .text_color(rgb(0x888888))
                            .text_xs()
                            .child(
                                root_path_str
                                    .split('/')
                                    .last()
                                    .unwrap_or(&root_path_str)
                                    .to_string()
                                    .to_uppercase(),
                            ),
                    )
                    .child(
                        div()
                            .flex_col()
                            // .overflow_y_scrollbar()
                            .children(self.render_tree(&self.root_path, 0, cx)),
                    ),
            )
            // The Resize Handle (invisible but wide enough to grab)
            .child(
                div()
                    .absolute()
                    .left(px(self.width - 2.0))
                    .w_2()
                    .h_full()
                    .cursor_col_resize()
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener(|this, _: &MouseDownEvent, _, cx| {
                            this.is_resizing = true;
                            cx.notify();
                        }),
                    ),
            )
    }
}
