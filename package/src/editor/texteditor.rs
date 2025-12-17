// pub mod implementations;
use gpui::actions;

use gpui::{
    App, Bounds as GpuiBounds, Context, Element, ElementId, FocusHandle, GlobalElementId,
    InputHandler, InspectorElementId, IntoElement, KeyContext, LayoutId, Pixels, Point, Style,
    UTF16Selection, Window,
};
use std::cell::RefCell;
use std::fs;
use std::ops::Range;
use std::panic;
use std::path::PathBuf;
use std::rc::Rc;

#[derive(Clone)]
pub struct TextEditor {
    content: Rc<RefCell<String>>,
    cursor_position: usize,
    selection: Option<Range<usize>>,
    pub focus_handle: FocusHandle,
    file_path: Option<PathBuf>,
}

actions!(
    TextEditor,
    [
        NewFile, OpenFile, SaveFile, SaveFileAs, Undo, Redo, Cut, Copy, Paste
    ]
);

impl TextEditor {
    pub fn new(cx: &mut Context<Self>) -> Self {
        Self {
            content: Rc::new(RefCell::new(String::new())),
            cursor_position: 0,
            selection: None,
            focus_handle: cx.focus_handle(),
            file_path: None,
        }
    }

    pub fn get_text(&self) -> String {
        self.content.borrow().clone()
    }

    pub fn set_text(&mut self, text: String) {
        *self.content.borrow_mut() = text;
        self.cursor_position = self.content.borrow().len();
        self.selection = None;
    }

    pub fn insert_text_at_cursor(&mut self, text: &str) {
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

    pub fn open_file_from_path(&mut self, path: PathBuf, cx: &mut Context<Self>) {
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
    pub fn new_file(&mut self, _: &NewFile, _window: &mut Window, cx: &mut Context<Self>) {
        self.set_text(String::new());
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
                self.set_text("// No file selected. Use file browser to open files.\n".to_string());
                cx.notify();
            }
        }
    }

    pub fn save_file(&mut self, _: &SaveFile, _window: &mut Window, cx: &mut Context<Self>) {
        if let Some(ref path) = self.file_path {
            if let Err(e) = fs::write(path, self.get_text()) {
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
        cx.notify();
    }

    pub fn redo(&mut self, _: &Redo, _window: &mut Window, cx: &mut Context<Self>) {
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
