// pub mod implementations;
use gpui::actions;
use serde::Deserialize;
pub mod DS;
pub mod config;
pub mod editor;
pub mod log;
pub mod settings;
pub mod state;
pub mod tests;
pub mod utils;
pub mod workspace;
pub mod ui {
    pub mod header;
}

use crate::config::{APP_NAME, BACKGROUND_COLOR, PRIMARY_COLOR};
use crate::editor::filebrowser::FileBrowser;
use crate::editor::texteditor::{
    Copy, Cut, NewFile, OpenFile, Paste, Redo, SaveFile, SaveFileAs, TextEditor, Undo,
};
use crate::state::appstate::AppState;
use crate::utils::{bind_editor_action, bind_global_action};
use gpui::{
    App, Application, Bounds, Context, Entity, FocusHandle, IntoElement, KeyBinding, KeyContext,
    LayoutId, Menu, MenuItem, MouseButton, Pixels, Point, Style, UTF16Selection, Window,
    WindowBounds, WindowOptions, div, prelude::*, px, rgb, size,
};
use std::env;
use std::path::PathBuf;
// use std::rc::Rc;

use crate::settings::SettingsView;
use crate::ui::header::Header;
use crate::workspace::WorkspaceItem;

pub struct MainScreen {
    items: Vec<WorkspaceItem>,
    active_item_index: usize,
    file_browser: Entity<FileBrowser>,
    show_browser: bool,
    show_info_panel: bool,
    focus_handle: FocusHandle,
    header: Header,
    state: Entity<AppState>,
}

actions!(
    MainScreen,
    MainScreen,
    [Quit, ToggleBrowser, ToggleInfoPanel, OpenSettings, CloseTab]
);

#[derive(Clone, PartialEq, Debug, Deserialize)]
pub struct SwitchTab {
    pub index: usize,
}

impl gpui::Action for SwitchTab {
    fn name(&self) -> &str { "SwitchTab" }
    fn name_for_type() -> &'static str { "SwitchTab" }
    fn build(value: serde_json::Value) -> gpui::Result<Box<dyn gpui::Action>> {
        let action: SwitchTab = serde_json::from_value(value)?;
        Ok(Box::new(action))
    }
    fn boxed_clone(&self) -> Box<dyn gpui::Action> {
        Box::new(self.clone())
    }
    fn partial_eq(&self, action: &dyn gpui::Action) -> bool {
        action.as_any().downcast_ref::<Self>().map_or(false, |a| self == a)
    }
}

// Ensure payload struct is compatible if using actions! with payload?
// actually actions! macro defines unit structs or structs that derive Deserialize.
// For `OpenPath`, we need data.
// GPUI `actions!` macro supports data payload if we implement serde::Deserialize.
// Let's implement Deserialize for OpenPath.
use serde::Deserialize;

#[derive(Clone, PartialEq, Debug, Deserialize)]
pub struct OpenPath {
    pub path: PathBuf,
}

impl gpui::Action for OpenPath {
    fn name(&self) -> &'static str { "OpenPath" }

    fn name_for_type() -> &'static str { "OpenPath" }
    
    fn build(value: serde_json::Value) -> gpui::Result<Box<dyn gpui::Action>> {
        let action: OpenPath = serde_json::from_value(value)?;
        Ok(Box::new(action))
    }
    
    fn boxed_clone(&self) -> Box<dyn gpui::Action> {
        Box::new(self.clone())
    }
    
    fn partial_eq(&self, action: &dyn gpui::Action) -> bool {
        action.as_any().downcast_ref::<Self>().map_or(false, |a| self == a)
    }
}

impl MainScreen {
    fn new(editor: Entity<TextEditor>, state: Entity<AppState>, cx: &mut Context<Self>) -> Self {
        let file_browser = cx.new(|cx| FileBrowser::new(cx));

        let initial_items = vec![WorkspaceItem::Editor(editor)];

        Self {
            items: initial_items,
            active_item_index: 0,
            file_browser,
            show_browser: true, // Show file browser by default
            show_info_panel: true,
            focus_handle: cx.focus_handle(),
            header: Header::new(),
            state,
        }
    }

    fn open_path(&mut self, action: &OpenPath, _window: &mut Window, cx: &mut Context<Self>) {
        if let Some(existing_index) = self.items.iter().position(|item| {
            match item {
                WorkspaceItem::Editor(editor) => {
                    // In a real app we'd check if editor has this path open.
                    // For now, simpler: always open new tab or just assume for now we don't check
                    false
                }
                _ => false,
            }
        }) {
            self.active_item_index = existing_index;
        } else {
            // Create new editor
            let path = action.path.clone(); // Clone path to use in closure
            // Create a NEW state for the new tab/file
            let state = cx.new(|_| AppState::new());
            let editor = cx.new(|cx| {
                let mut editor = TextEditor::new(cx, state);
                editor.open_file_from_path(path, cx);
                editor
            });
            self.items.push(WorkspaceItem::Editor(editor));
            self.active_item_index = self.items.len() - 1;
        }
        cx.notify();
    }

    fn open_settings(&mut self, _: &OpenSettings, _window: &mut Window, cx: &mut Context<Self>) {
        // Check if settings already open
        if let Some(index) = self
            .items
            .iter()
            .position(|item| matches!(item, WorkspaceItem::Settings(_)))
        {
            self.active_item_index = index;
        } else {
            let settings_view = cx.new(|_cx| SettingsView::new());
            self.items.push(WorkspaceItem::Settings(settings_view));
            self.active_item_index = self.items.len() - 1;
        }
        cx.notify();
    }

    fn toggle_browser(&mut self, _: &ToggleBrowser, _window: &mut Window, cx: &mut Context<Self>) {
        self.show_browser = !self.show_browser;
        cx.notify();
    }

    fn toggle_info_panel(
        &mut self,
        _: &ToggleInfoPanel,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.show_info_panel = !self.show_info_panel;
        cx.notify();
    }
    
    fn switch_tab(&mut self, action: &SwitchTab, _: &mut Window, cx: &mut Context<Self>) {
        if action.index < self.items.len() {
            self.active_item_index = action.index;
            cx.notify();
        }
    }
    
    fn close_tab(&mut self, _: &CloseTab, _: &mut Window, cx: &mut Context<Self>) {
        if !self.items.is_empty() {
             self.items.remove(self.active_item_index);
             if self.active_item_index >= self.items.len() && !self.items.is_empty() {
                 self.active_item_index = self.items.len() - 1;
             } else if self.items.is_empty() {
                 self.active_item_index = 0;
             }
             cx.notify();
        }
    }
}

impl Render for MainScreen {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let active_item = self.items.get(self.active_item_index);

        div()
            .track_focus(&self.focus_handle)
            .on_action(cx.listener(MainScreen::toggle_browser))
            .on_action(cx.listener(MainScreen::toggle_info_panel))
            .on_action(cx.listener(MainScreen::open_path))
            .on_action(cx.listener(MainScreen::open_settings))
            .on_action(cx.listener(MainScreen::switch_tab))
            .on_action(cx.listener(MainScreen::close_tab))
            .on_action(cx.listener(|_, _: &Quit, _, cx| cx.quit()))
            .bg(BACKGROUND_COLOR)
            .text_color(PRIMARY_COLOR)
            .size_full()
            .flex()
            .flex_col() // Main layout is now Column (Header + Body)
            .child(self.header.render(window, cx))
            .child(
                div()
                    .flex()
                    .flex_row()
                    .flex_1()
                    .h_full() // Body takes remaining height
                    .when(self.show_browser, |this| {
                        this.child(
                            div()
                                .flex()
                                .h_full()
                                .flex_none()
                                .child(self.file_browser.clone()),
                        )
                    })
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .flex_1()
                            .size_full()
                            // Tab Bar
                            .child(
                                div()
                                    .flex()
                                    .flex_row()
                                    .h_8()
                                    .bg(rgb(0x252526))
                                    .text_color(rgb(0xcccccc))
                                    .children(
                                        self.items.iter().enumerate().map(|(i, item)| {
                                            let is_active = i == self.active_item_index;
                                            let title = match item {
                                                WorkspaceItem::Editor(e) => {
                                                    // This is tricky without read access to appstate or editor model easily here
                                                    // Assume "Editor" for now or try to get file path if we could.
                                                    // Actually we can't easily read entity state inside render without window context, 
                                                    // but we have window context.
                                                    // For now: "Editor"
                                                    "Editor"
                                                }
                                                WorkspaceItem::Settings(_) => "Settings",
                                            };
                                            
                                            div()
                                                .h_full()
                                                .px_3()
                                                .flex()
                                                .items_center()
                                                .bg(if is_active { rgb(0x1e1e1e) } else { rgb(0x2d2d2d) })
                                                .border_r_1()
                                                .border_color(rgb(0x1e1e1e))
                                                .cursor_pointer()
                                                .on_click(cx.listener(move |_, _, cx| {
                                                    cx.dispatch_action(SwitchTab { index: i });
                                                }))
                                                .child(title)
                                                .when(is_active, |this| {
                                                    this.child(
                                                        div()
                                                            .ml_2()
                                                            .child("x")
                                                            .hover(|s| s.text_color(rgb(0xffffff)))
                                                            .on_click(cx.listener(|_, _, cx| {
                                                                cx.dispatch_action(CloseTab);
                                                            }))
                                                    )
                                                })
                                        })
                                    )
                            )
                            // Content
                            .child(active_item.map(|item| item.render(cx))),
                    ),
            )
        // Info panel if needed... (omitted for brevity in this replace, can add back if important, user commented it out in their previous edit so I might leave it out or re-add)
    }
}

fn main() {
    let _env: Vec<String> = env::args().collect();
    Application::new().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(1000.), px(800.0)), cx);
        info!("ACTIVATING");
        cx.activate(true);
        debug!("ACTIVATED");
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
                    items: vec![
                        MenuItem::action("Toggle Browser", ToggleBrowser),
                        MenuItem::action("Toggle Info Panel", ToggleInfoPanel),
                    ],
                }),
            ],
        };

        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Maximized(bounds)),
                ..Default::default()
            },
            |window, cx| {
                // doesnot set menus for some reason ,
                // even though i followed their example at
                // https://gpui.rs
                cx.set_menus(vec![menu]);

                // keybindings
                bind_global_action(cx, "ctrl-b", ToggleBrowser);
                bind_global_action(cx, "ctrl-l", ToggleInfoPanel);
                bind_global_action(cx, "ctrl-shift-p", OpenSettings); // Temporary shortcut for settings requested by user
                bind_global_action(cx, "ctrl-,", OpenSettings); // Standard settings shortcut

                bind_editor_action(cx, "ctrl-n", NewFile);
                bind_editor_action(cx, "ctrl-o", OpenFile);
                bind_editor_action(cx, "ctrl-s", SaveFile);
                bind_editor_action(cx, "ctrl-shift-s", SaveFileAs);
                bind_editor_action(cx, "ctrl-z", Undo);
                bind_editor_action(cx, "ctrl-shift-z", Redo);
                bind_editor_action(cx, "ctrl-x", Cut);
                bind_editor_action(cx, "ctrl-c", Copy);
                bind_editor_action(cx, "ctrl-v", Paste);

                let app_state = cx.new(|_cx| AppState::new());
                let editor = cx.new(|cx| TextEditor::new(cx, app_state.clone()));
                let editor_focus = editor.read(cx).focus_handle.clone();
                window.focus(&editor_focus);

                let main_screen = cx.new(|cx| MainScreen::new(editor, app_state, cx));
                main_screen
            },
        )
        .unwrap();
    });
}
