// pub mod implementations;
use gpui::actions;
pub mod config;
pub mod state;
pub mod utils;

use crate::config::{APP_NAME, BACKGROUND_COLOR, PRIMARY_COLOR};
use gpui::{
    App, Application, Bounds, Context, Element, ElementId, Entity, FocusHandle, GlobalElementId,
    InspectorElementId, InputHandler, IntoElement, KeyBinding, KeyContext, LayoutId, Menu, MenuItem,
    MouseButton, Pixels, Point, Style, UTF16Selection, Window, WindowBounds, WindowOptions, div, prelude::*, px,
    rgb, size, Bounds as GpuiBounds,
};
use std::cell::RefCell;
use std::env;
use std::fs;
use std::ops::Range;
use std::panic;
use std::path::PathBuf;
use std::rc::Rc;

#[derive(Clone)]
struct TextEditor {
    content: Rc<RefCell<String>>,
    cursor_position: usize,
    selection: Option<Range<usize>>,
    focus_handle: FocusHandle,
    file_path: Option<PathBuf>,
}

actions!(
    TextEditor,
    [
        NewFile, OpenFile, SaveFile, SaveFileAs, Undo, Redo, Cut, Copy, Paste
    ]
);

impl TextEditor {
    fn new(cx: &mut Context<Self>) -> Self {
        Self {
            content: Rc::new(RefCell::new(String::new())),
            cursor_position: 0,
            selection: None,
            focus_handle: cx.focus_handle(),
            file_path: None,
        }
    }

    fn get_text(&self) -> String {
        self.content.borrow().clone()
    }

    fn set_text(&mut self, text: String) {
        *self.content.borrow_mut() = text;
        self.cursor_position = self.content.borrow().len();
        self.selection = None;
    }

    fn insert_text_at_cursor(&mut self, text: &str) {
        let mut content = self.content.borrow_mut();
        if let Some(selection) = &self.selection {
            content.replace_range(selection.clone(), text);
            self.cursor_position = selection.start + text.len();
        } else {
            content.insert_str(self.cursor_position, text);
            self.cursor_position += text.len();
        }
        self.selection = None;
    }

    fn open_file_from_path(&mut self, path: PathBuf, cx: &mut Context<Self>) {
        match fs::read_to_string(&path) {
            Ok(content) => {
                self.set_text(content);
                self.file_path = Some(path);
                cx.notify();
            }
            Err(e) => {
                eprintln!("Failed to open file: {}", e);
            }
        }
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
        _: &mut App,
    ) -> Option<String> {
        let content = self.content.borrow();
        if range.end <= content.len() {
            Some(content[range.clone()].to_string())
        } else {
            None
        }
    }

    fn replace_text_in_range(
        &mut self,
        replacement_range: Option<Range<usize>>,
        text: &str,
        _: &mut Window,
        _: &mut App,
    ) {
        if let Some(range) = replacement_range {
            let mut content = self.content.borrow_mut();
            if range.end <= content.len() {
                content.replace_range(range.clone(), text);
                self.cursor_position = range.start + text.len();
            }
        } else {
            self.insert_text_at_cursor(text);
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
    fn new_file(&mut self, _: &NewFile, _window: &mut Window, cx: &mut Context<Self>) {
        self.set_text(String::new());
        self.file_path = None;
        cx.notify();
    }

    fn open_file(&mut self, _: &OpenFile, _window: &mut Window, cx: &mut Context<Self>) {
        // For now, open a sample file - in real implementation would show file dialog
        let sample_path = PathBuf::from(".");
        if sample_path.exists() {
            // Try to open a test file if it exists
            let test_file = sample_path.join("test.txt");
            if test_file.exists() {
                self.open_file_from_path(test_file, cx);
            } else {
                self.set_text("// No file selected. Use file browser to open files.\n".to_string());
                cx.notify();
            }
        }
    }

    fn save_file(&mut self, _: &SaveFile, _window: &mut Window, cx: &mut Context<Self>) {
        if let Some(ref path) = self.file_path {
            if let Err(e) = fs::write(path, self.get_text()) {
                eprintln!("Failed to save file: {}", e);
            } else {
                cx.notify();
            }
        }
    }

    fn save_file_as(&mut self, _: &SaveFileAs, _window: &mut Window, cx: &mut Context<Self>) {
        // Placeholder - would show save dialog
        cx.notify();
    }

    fn undo(&mut self, _: &Undo, _window: &mut Window, cx: &mut Context<Self>) {
        cx.notify();
    }

    fn redo(&mut self, _: &Redo, _window: &mut Window, cx: &mut Context<Self>) {
        cx.notify();
    }

    fn cut(&mut self, _: &Cut, _window: &mut Window, cx: &mut Context<Self>) {
        cx.notify();
    }

    fn copy(&mut self, _: &Copy, _window: &mut Window, cx: &mut Context<Self>) {
        cx.notify();
    }

    fn paste(&mut self, _: &Paste, _window: &mut Window, cx: &mut Context<Self>) {
        cx.notify();
    }
}

struct FileBrowser {
    current_path: PathBuf,
    selected_file: Option<PathBuf>,
    editor: Entity<TextEditor>,
}

impl FileBrowser {
    fn new(editor: Entity<TextEditor>, cx: &mut Context<Self>) -> Self {
        Self {
            current_path: PathBuf::from("."),
            selected_file: None,
            editor,
        }
    }

    fn get_files(&self) -> Vec<PathBuf> {
        match fs::read_dir(&self.current_path) {
            Ok(entries) => {
                let mut files: Vec<PathBuf> = entries
                    .filter_map(|e| e.ok())
                    .map(|e| e.path())
                    .collect();
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
            .p_2()
            .child(
                div()
                    .text_color(rgb(0x888888))
                    .text_sm()
                    .mb_2()
                    .child(format!("📁 {}", current_path_str)),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_1()
                    .children(files.iter().map(|file| {
                        let file_name = file.file_name()
                            .and_then(|n| n.to_str())
                            .unwrap_or("?")
                            .to_string();
                        let is_dir = file.is_dir();
                        let icon = if is_dir { "📁" } else { "📄" };
                        let file_path = file.clone();
                        
                        div()
                            .p_2()
                            .rounded_md()
                            .bg(rgb(0x252525))
                            .text_color(rgb(0xcccccc))
                            .text_sm()
                            .cursor_pointer()
                            .hover(|d| d.bg(rgb(0x303030)))
                            .child(format!("{} {}", icon, file_name))
                            .on_mouse_down(MouseButton::Left, cx.listener(move |this, _evt, _window, cx| {
                                this.open_file(file_path.clone(), cx);
                            }))
                    }))
            )
    }
}

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
                // Line numbers
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
                // Editor content area - this is where the Element will be rendered
                div()
                    .flex()
                    .flex_col()
                    .flex_1()
                    .p_4()
                    .text_sm()
                    .text_color(rgb(0xcccccc))
                    .bg(rgb(0x252525))
                    .child(
                        if text.is_empty() {
                            div().h_4().child("")
                        } else {
                            div().child(text)
                        }
                    )
                    .child(self.clone()) // Render the Element for input handling
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
                        .child(self.file_browser.clone())
                )
            })
            .child(
                // Editor view
                div()
                    .flex()
                    .flex_1()
                    .size_full()
                    .child(self.editor.clone()),
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
    let _env: Vec<String> = env::args().collect();
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
