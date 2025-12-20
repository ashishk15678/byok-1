use gpui::{Entity, Render, AnyElement, IntoElement, Element, VisualContext, Window, Context};
use crate::editor::texteditor::TextEditor;
use crate::settings::SettingsView;

pub enum WorkspaceItem {
    Editor(Entity<TextEditor>),
    Settings(Entity<SettingsView>),
}

impl WorkspaceItem {
    pub fn title(&self, cx: &Window) -> String {
        match self {
            WorkspaceItem::Editor(_) => "Editor".to_string(), // Need access to filename
            WorkspaceItem::Settings(_) => "Settings".to_string(),
        }
    }
    
    pub fn render(&self, cx: &mut Context<crate::MainScreen>) -> AnyElement {
         match self {
            WorkspaceItem::Editor(editor) => editor.clone().into_any_element(),
            WorkspaceItem::Settings(settings) => settings.clone().into_any_element(),
        }
    }
}
