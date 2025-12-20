// pub mod implementations;
use gpui::{InteractiveElement, ParentElement, Render, Styled, actions, div, rgb};

// use crate::config::{CHUNK_LOAD_LINES, INITIAL_LOAD_LINES};
use gpui::{
    App, Bounds as GpuiBounds, Context, Element, ElementId, FocusHandle, GlobalElementId,
    InputHandler, InspectorElementId, IntoElement, KeyContext, KeyDownEvent, LayoutId, Pixels,
    Point, ScrollWheelEvent, Style, UTF16Selection, Window, px,
};
use std::cell::RefCell;
use std::fs;
use std::ops::Range;
use std::panic;
use std::path::PathBuf;
use std::rc::Rc;
use crate::state::appstate::AppState;
use gpui::Entity;

use crate::DS::tree::UndoTree;

#[derive(Clone)]
pub struct TextEditor {
    model: Entity<AppState>,
    cursor_position: usize,
    selection: Option<Range<usize>>,
    pub focus_handle: FocusHandle,
    file_path: Option<PathBuf>,
    undo_tree: UndoTree,
    scroll_x: f32,
    scroll_y: f32,
    line_height: f32,
}

actions!(
    TextEditor,
    [
        NewFile, OpenFile, SaveFile, SaveFileAs, Undo, Redo, Cut, Copy, Paste
    ]
);

impl Render for TextEditor {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let text = self.get_text(cx);
        let mut lines: Vec<&str> = text.lines().collect();
        if lines.is_empty() {
            lines.push("");
        }
        let line_count = lines.len();
        let viewport_height = 800.0;
        let start_line = (self.scroll_y / self.line_height).floor() as usize;
        let visible_lines = (viewport_height / self.line_height).ceil() as usize;
        let end_line = std::cmp::min(line_count, start_line + visible_lines);

        let start_line = std::cmp::min(start_line, line_count.saturating_sub(1));

        // Calculate cursor position once
        let cursor_row = self.cursor_row(cx);
        let cursor_col = self.cursor_col(cx);

        // Calculate offset for the content container
        let content_y = -(self.scroll_y % self.line_height);
        let line_height = self.line_height;

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
            .on_key_down(cx.listener(TextEditor::handle_key_down))
            .on_scroll_wheel(cx.listener(TextEditor::handle_scroll_wheel))
            .on_mouse_down(
                gpui::MouseButton::Left,
                cx.listener(|this, _: &gpui::MouseDownEvent, window, cx| {
                    window.focus(&this.focus_handle);
                }),
            )
            .key_context("Editor")
            .size_full()
            .flex()
            .flex_row()
            .child(
                div()
                    .w_16()
                    .h_full()
                    // .bg(rgb(0x1e1e1e))
                    // .border_r_1()
                    .border_color(rgb(0x404040))
                    .p_1()
                    .text_color(rgb(0x888888))
                    .text_sm()
                    // Gutter should also scroll
                    // Implementation simplification: just rendering numbers matching visible content
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            // Offset gutter to match content
                            .mt(px(content_y))
                            .children(
                                (start_line..end_line)
                                    .map(|i| div().text_right().child((i + 1).to_string())),
                            ),
                    ),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .flex_1()
                    .p_1()
                    // Clip content that scrolls out
                    .overflow_hidden()
                    .text_sm()
                    .text_color(rgb(0xcccccc))
                    .bg(rgb(0x252525))
                    .child(
                        div()
                            // Offset content
                            .mt(px(content_y))
                            .ml(px(-self.scroll_x))
                            .flex()
                            .flex_col()
                            .gap_1() // Match gutter gap
                            .children(lines[start_line..end_line].iter().enumerate().map(
                                move |(rel_idx, line)| {
                                    let line_idx = start_line + rel_idx;
                                    // Ensure empty lines have height
                                    let content = if line.is_empty() { " " } else { *line };

                                    // Calculate cursor
                                    let cursor_element = if line_idx == cursor_row {
                                        let col = cursor_col;
                                        let col = std::cmp::min(col, line.len());

                                        // Split line for cursor
                                        let (pre, post) = if line.is_empty() {
                                            ("", "")
                                        } else if col >= line.len() {
                                            (*line, "")
                                        } else {
                                            line.split_at(col)
                                        };

                                        div()
                                            .flex()
                                            .flex_row()
                                            .child(pre.to_string())
                                            .child(
                                                div().w(px(2.0)).h_full().bg(rgb(0x5cabf5)), // Cursor color
                                            )
                                            .child(post.to_string())
                                    } else {
                                        div().child(content.to_string())
                                    };
                                    
                                    div().h(px(line_height)).child(cursor_element)
                                },
                            )),
                    )
                    .child(self.clone()), // Render the Element for input handling
            )
    }
}

impl TextEditor {
    pub fn new(cx: &mut Context<Self>, model: Entity<AppState>) -> Self {
        // Observe the model to trigger updates when state changes
        cx.observe(&model, |_, _window, cx| cx.notify()).detach();

        Self {
            model,
            cursor_position: 0,
            selection: None,
            focus_handle: cx.focus_handle(),
            file_path: None,
            scroll_x: 0.0,
            scroll_y: 0.0,
            undo_tree: UndoTree::new(""),
            line_height: 20.0, // Approximation, should ideally be measured
        }
    }

    pub fn get_text(&self, cx: &Context<Self>) -> String {
        self.model.read(cx).text.clone()
    }

    pub fn set_text(&mut self, text: String, cx: &mut Context<Self>) {
        let text_clone = text.clone();
        self.model.update(cx, |model, _| model.text = text); 
        self.cursor_position = text_clone.len();
        self.selection = None;
        self.scroll_x = 0.0;
        self.scroll_y = 0.0;
        self.undo_tree.commit(text_clone);
    }

    pub fn insert_text_at_cursor(&mut self, text: &str, cx: &mut Context<Self>) {
        let selection = self.selection.clone();
        let old_cursor_pos = self.cursor_position;
        let text_owned = text.to_string();
        
        let new_cursor_pos = self.model.update(cx, move |model, _| {
            let content = &mut model.text;
            if let Some(selection) = selection {
                content.replace_range(selection.clone(), &text_owned);
                selection.start + text_owned.len()
            } else {
                let mut pos = old_cursor_pos;
                if pos > content.len() {
                    pos = content.len();
                }
                content.insert_str(pos, &text_owned);
                pos + text_owned.len()
            }
        });
        
        self.cursor_position = new_cursor_pos;
        self.selection = None;
    }

    pub fn open_file_from_path(&mut self, path: PathBuf, cx: &mut Context<Self>) {
        match fs::read_to_string(&path) {
            Ok(content) => {
                self.set_text(content, cx);
                self.file_path = Some(path);
                cx.notify();
            }
            Err(e) => {
                eprintln!("Failed to open file: {}: {}", path.display(), e);
            }
        }
    }

    fn handle_scroll_wheel(
        &mut self,
        event: &ScrollWheelEvent,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let delta = event.delta.pixel_delta(px(self.line_height));

        // Update scroll_y
        self.scroll_y -= f32::from(delta.y);
        if self.scroll_y < 0.0 {
            self.scroll_y = 0.0;
        }

        // Update scroll_x
        self.scroll_x -= f32::from(delta.x);
        if self.scroll_x < 0.0 {
            self.scroll_x = 0.0;
        }

        cx.notify();
    }

    fn handle_key_down(
        &mut self,
        event: &KeyDownEvent,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let keystroke = &event.keystroke.key;

        match keystroke.as_str() {
            "left" => self.move_cursor_left(cx),
            "right" => self.move_cursor_right(cx),
            "up" => self.move_cursor_up(cx),
            "down" => self.move_cursor_down(cx),
            "backspace" => self.handle_backspace(cx),
            "delete" => self.handle_delete(cx),
            "enter" => self.handle_enter(cx),
            _ => {
               // Do nothing for default keys, rely on InputHandler
            }
        }
        cx.notify();
    }

    fn move_cursor_left(&mut self, cx: &Context<Self>) {
        if self.cursor_position > 0 {
            self.cursor_position -= 1;
            // Handle utf8 char boundaries if necessary, simplified for now
            let text = &self.model.read(cx).text;
            while !text.is_char_boundary(self.cursor_position)
                && self.cursor_position > 0
            {
                self.cursor_position -= 1;
            }
        }
    }

    fn move_cursor_right(&mut self, cx: &Context<Self>) {
        let text = &self.model.read(cx).text;
        let len = text.len();
        if self.cursor_position < len {
            self.cursor_position += 1;
            while !text.is_char_boundary(self.cursor_position)
                && self.cursor_position < len
            {
                self.cursor_position += 1;
            }
        }
    }

    fn move_cursor_up(&mut self, cx: &Context<Self>) {
        // Find previous line start and offset
        let text = self.model.read(cx).text.clone();
        let content = text.as_str();
        let current_line_start = content[..self.cursor_position]
            .rfind('\n')
            .map(|i| i + 1)
            .unwrap_or(0);
        let column = self.cursor_position - current_line_start;

        if current_line_start > 0 {
            let prev_line_end = current_line_start - 1;
            let prev_line_start = content[..prev_line_end]
                .rfind('\n')
                .map(|i| i + 1)
                .unwrap_or(0);
            let prev_line_len = prev_line_end - prev_line_start;

            self.cursor_position = prev_line_start + std::cmp::min(column, prev_line_len);
        } else {
            self.cursor_position = 0;
        }
    }

    fn move_cursor_down(&mut self, cx: &Context<Self>) {
        let text = self.model.read(cx).text.clone();
        let content = text.as_str();
        let current_line_start = content[..self.cursor_position]
            .rfind('\n')
            .map(|i| i + 1)
            .unwrap_or(0);
        let column = self.cursor_position - current_line_start;

        if let Some(next_line_start_offset) = content[self.cursor_position..].find('\n') {
            let next_line_start = self.cursor_position + next_line_start_offset + 1;
            if next_line_start < content.len() {
                let next_line_end = content[next_line_start..]
                    .find('\n')
                    .map(|i| next_line_start + i)
                    .unwrap_or(content.len());
                let next_line_len = next_line_end - next_line_start;
                self.cursor_position = next_line_start + std::cmp::min(column, next_line_len);
            } else {
                self.cursor_position = content.len();
            }
        } else {
            self.cursor_position = content.len();
        }
    }

    // Helper methods for cursor rendering
    fn cursor_row(&self, cx: &Context<Self>) -> usize {
        let text = self.model.read(cx).text.clone();
        text[..self.cursor_position].matches('\n').count()
    }

    fn cursor_col(&self, cx: &Context<Self>) -> usize {
        let text = self.model.read(cx).text.clone();
        let line_start = text[..self.cursor_position]
            .rfind('\n')
            .map(|i| i + 1)
            .unwrap_or(0);
        self.cursor_position - line_start
    }

    fn handle_backspace(&mut self, cx: &mut Context<Self>) {
        if self.cursor_position > 0 {
            let cursor_pos = self.cursor_position;
            let new_pos = self.model.update(cx, move |model, _| {
                let content = &mut model.text;
                let mut start = cursor_pos - 1;
                while !content.is_char_boundary(start) && start > 0 {
                    start -= 1;
                }
                content.remove(start);
                start
            });
            self.cursor_position = new_pos;
        }
    }

    fn handle_delete(&mut self, cx: &mut Context<Self>) {
        let cursor_pos = self.cursor_position;
        self.model.update(cx, move |model, _| {
            let content = &mut model.text;
            if cursor_pos < content.len() {
                content.remove(cursor_pos);
            }
        });
    }

    fn handle_enter(&mut self, cx: &mut Context<Self>) {
        self.insert_text_at_cursor("\n", cx);
    }
}

impl InputHandler for TextEditor {
    fn selected_text_range(
        &mut self,
        _: bool,
        _: &mut Window,
        _: &mut App,
    ) -> Option<UTF16Selection> {
        self.selection.as_ref().map(|range| UTF16Selection {
            range: range.clone(),
            reversed: false,
        })
    }

    fn marked_text_range(&mut self, _: &mut Window, _: &mut App) -> Option<Range<usize>> {
        None
    }

    fn text_for_range(
        &mut self,
        range: Range<usize>,
        _: &mut Option<Range<usize>>,
        _: &mut Window,
        app: &mut App,
    ) -> Option<String> {
        let text = self.model.read(app).text.clone();
        if range.end <= text.len() {
            Some(text[range.clone()].to_string())
        } else {
            None
        }
    }

    fn replace_text_in_range(
        &mut self,
        replacement_range: Option<Range<usize>>,
        text: &str,
        _: &mut Window,
        app: &mut App,
    ) {
        if let Some(range) = replacement_range {
            let text_owned = text.to_string();
            let new_pos = self.model.update(app, move |model, _| {
                let content = &mut model.text;
                if range.end <= content.len() {
                    content.replace_range(range.clone(), &text_owned);
                    Some(range.start + text_owned.len())
                } else {
                    None
                }
            });
            
            if let Some(pos) = new_pos {
                self.cursor_position = pos;
            }
        } else {
             // Inline insert logic here
             let selection = self.selection.clone();
             let old_cursor_pos = self.cursor_position;
             let text_owned = text.to_string();

             let new_cursor_pos = self.model.update(app, move |model, _| {
                 let content = &mut model.text;
                 if let Some(selection) = selection {
                     content.replace_range(selection.clone(), &text_owned);
                     selection.start + text_owned.len()
                 } else {
                     let mut pos = old_cursor_pos;
                     if pos > content.len() {
                         pos = content.len();
                     }
                     content.insert_str(pos, &text_owned);
                     pos + text_owned.len()
                 }
             });
             self.cursor_position = new_cursor_pos;
        }
        self.selection = None;
    }

    fn replace_and_mark_text_in_range(
        &mut self,
        replacement_range: Option<Range<usize>>,
        new_text: &str,
        _marked_range: Option<Range<usize>>,
        window: &mut Window,
        app: &mut App,
    ) {
        self.replace_text_in_range(replacement_range, new_text, window, app);
    }

    fn unmark_text(&mut self, _: &mut Window, _: &mut App) {}

    fn bounds_for_range(
        &mut self,
        _: Range<usize>,
        _: &mut Window,
        _: &mut App,
    ) -> Option<GpuiBounds<Pixels>> {
        None
    }

    fn character_index_for_point(
        &mut self,
        _: Point<Pixels>,
        _: &mut Window,
        _: &mut App,
    ) -> Option<usize> {
        Some(self.cursor_position)
    }
}

impl Element for TextEditor {
    type RequestLayoutState = ();
    type PrepaintState = ();

    fn id(&self) -> Option<ElementId> {
        Some("text_editor".into())
    }

    fn source_location(&self) -> Option<&'static panic::Location<'static>> {
        None
    }

    fn request_layout(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Option<&InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (LayoutId, Self::RequestLayoutState) {
        (window.request_layout(Style::default(), [], cx), ())
    }

    fn prepaint(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Option<&InspectorElementId>,
        _: GpuiBounds<Pixels>,
        _: &mut Self::RequestLayoutState,
        _window: &mut Window,
        _cx: &mut App,
    ) -> Self::PrepaintState {
        ()
    }

    fn paint(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Option<&InspectorElementId>,
        _: GpuiBounds<Pixels>,
        _: &mut Self::RequestLayoutState,
        _: &mut Self::PrepaintState,
        window: &mut Window,
        cx: &mut App,
    ) {
        let mut key_context = KeyContext::default();
        key_context.add("Editor");
        window.set_key_context(key_context);
        window.handle_input(&self.focus_handle, self.clone(), cx);
    }
}

impl IntoElement for TextEditor {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}

impl TextEditor {
    pub fn new_file(&mut self, _: &NewFile, _window: &mut Window, cx: &mut Context<Self>) {
        self.set_text(String::new(), cx);
        self.file_path = None;
        cx.notify();
    }

    pub fn open_file(&mut self, _: &OpenFile, _window: &mut Window, cx: &mut Context<Self>) {
        // For now, open a sample file - in real implementation would show file dialog
        let sample_path = PathBuf::from(".");
        if sample_path.exists() {
            // Try to open a test file if it exists
            let test_file = sample_path.join("test.txt");
            if test_file.exists() {
                self.open_file_from_path(test_file, cx);
            } else {
                self.set_text("// No file selected. Use file browser to open files.\n".to_string(), cx);
                cx.notify();
            }
        }
    }

    pub fn save_file(&mut self, _: &SaveFile, _window: &mut Window, cx: &mut Context<Self>) {
        if let Some(ref path) = self.file_path {
            if let Err(e) = fs::write(path, self.get_text(cx)) {
                eprintln!("Failed to save file: {}", e);
            } else {
                cx.notify();
            }
        }
    }

    pub fn save_file_as(&mut self, _: &SaveFileAs, _window: &mut Window, cx: &mut Context<Self>) {
        // Placeholder - would show save dialog
        cx.notify();
    }

    pub fn undo(&mut self, _: &Undo, _window: &mut Window, cx: &mut Context<Self>) {
        self.undo_tree.undo();
        cx.notify();
    }

    pub fn redo(&mut self, _: &Redo, _window: &mut Window, cx: &mut Context<Self>) {
        self.undo_tree.redo();
        cx.notify();
    }

    pub fn cut(&mut self, _: &Cut, _window: &mut Window, cx: &mut Context<Self>) {
        cx.notify();
    }

    pub fn copy(&mut self, _: &Copy, _window: &mut Window, cx: &mut Context<Self>) {
        cx.notify();
    }

    pub fn paste(&mut self, _: &Paste, _window: &mut Window, cx: &mut Context<Self>) {
        cx.notify();
    }
}
