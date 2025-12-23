use crate::config::{BACKGROUND_COLOR, PRIMARY_COLOR};
use crate::editor::filebrowser::FileBrowser;
use crate::editor::texteditor::TextEditor;
use crate::state::appstate::AppState;
use crate::ui::header::Header;
use crate::workspace::WorkspaceItem;
use gpui::prelude::*;
use gpui::*;
use serde::Deserialize;
use std::path::PathBuf;

// Imports at top needed: StatusBar
use crate::ui::statusbar::StatusBar;

pub struct MainScreen {
    pub items: Vec<WorkspaceItem>,
    pub active_item_index: usize,
    pub file_browser: Entity<FileBrowser>,
    pub show_browser: bool,
    pub show_info_panel: bool,
    pub focus_handle: FocusHandle,
    pub header: Header,
    pub status_bar: StatusBar, // Add Status Bar
    pub state: Entity<AppState>,
    pub show_file_switcher: bool,
}

#[derive(Clone, PartialEq, Debug, Deserialize)]
pub struct SwitchTab {
    pub index: usize,
}

impl gpui::Action for SwitchTab {
    fn name(&self) -> &'static str {
        "SwitchTab"
    }
    fn name_for_type() -> &'static str {
        "SwitchTab"
    }
    fn build(value: serde_json::Value) -> gpui::Result<Box<dyn gpui::Action>> {
        let action: SwitchTab = serde_json::from_value(value)?;
        Ok(Box::new(action))
    }
    fn boxed_clone(&self) -> Box<dyn gpui::Action> {
        Box::new(self.clone())
    }
    fn partial_eq(&self, action: &dyn gpui::Action) -> bool {
        action
            .as_any()
            .downcast_ref::<Self>()
            .map_or(false, |a| self == a)
    }
}

#[derive(Clone, PartialEq, Debug, Deserialize)]
pub struct OpenPath {
    pub path: PathBuf,
}

impl gpui::Action for OpenPath {
    fn name(&self) -> &'static str {
        "OpenPath"
    }

    fn name_for_type() -> &'static str {
        "OpenPath"
    }

    fn build(value: serde_json::Value) -> gpui::Result<Box<dyn gpui::Action>> {
        let action: OpenPath = serde_json::from_value(value)?;
        Ok(Box::new(action))
    }

    fn boxed_clone(&self) -> Box<dyn gpui::Action> {
        Box::new(self.clone())
    }

    fn partial_eq(&self, action: &dyn gpui::Action) -> bool {
        action
            .as_any()
            .downcast_ref::<Self>()
            .map_or(false, |a| self == a)
    }
}

#[derive(Clone, PartialEq, Debug, Deserialize)]
pub struct TriggerSearch;

impl gpui::Action for TriggerSearch {
    fn name(&self) -> &'static str {
        "TriggerSearch"
    }
    fn name_for_type() -> &'static str {
        "TriggerSearch"
    }
    fn build(_: serde_json::Value) -> gpui::Result<Box<dyn gpui::Action>> {
        Ok(Box::new(TriggerSearch))
    }
    fn boxed_clone(&self) -> Box<dyn gpui::Action> {
        Box::new(self.clone())
    }
    fn partial_eq(&self, action: &dyn gpui::Action) -> bool {
        action
            .as_any()
            .downcast_ref::<Self>()
            .map_or(false, |a| self == a)
    }
}

actions!(
    MainScreen,
    [
        Quit,
        ToggleBrowser,
        ToggleInfoPanel,
        OpenSettings,
        CloseTab,
        ToggleFileSwitcher
    ]
);

use crate::settings::SettingsView;

impl MainScreen {
    pub fn new(
        editor: Entity<TextEditor>,
        state: Entity<AppState>,
        cx: &mut Context<Self>,
    ) -> Self {
        let file_browser = cx.new(|cx| FileBrowser::new(cx));

        let initial_items = vec![WorkspaceItem::Editor(editor)];

        Self {
            items: initial_items,
            active_item_index: 0,
            file_browser,
            show_browser: true,
            show_info_panel: true,
            focus_handle: cx.focus_handle(),
            header: Header::new(),
            status_bar: StatusBar::new(),
            state,
            show_file_switcher: false,
        }
    }

    pub fn open_path(&mut self, action: &OpenPath, _window: &mut Window, cx: &mut Context<Self>) {
        if let Some(existing_index) = self.items.iter().position(|item| {
            if let WorkspaceItem::Editor(editor) = item {
                let editor_model = editor.read(cx);
                if let Some(opened_path) = &editor_model.file_path {
                    return opened_path == &action.path;
                }
            }
            false
        }) {
            self.active_item_index = existing_index;
        } else {
            let path = action.path.clone();
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

    pub fn open_settings(
        &mut self,
        _: &OpenSettings,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
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

    pub fn toggle_browser(
        &mut self,
        _: &ToggleBrowser,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.show_browser = !self.show_browser;
        cx.notify();
    }

    pub fn toggle_info_panel(
        &mut self,
        _: &ToggleInfoPanel,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.show_info_panel = !self.show_info_panel;
        cx.notify();
    }

    pub fn switch_tab(&mut self, action: &SwitchTab, _: &mut Window, cx: &mut Context<Self>) {
        if action.index < self.items.len() {
            self.active_item_index = action.index;
            cx.notify();
        }
    }

    pub fn close_tab(&mut self, _: &CloseTab, _: &mut Window, cx: &mut Context<Self>) {
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

    pub fn toggle_file_switcher(
        &mut self,
        _: &ToggleFileSwitcher,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.show_file_switcher = !self.show_file_switcher;
        cx.notify();
    }

    pub fn trigger_search(
        &mut self,
        _: &TriggerSearch,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let state = self.state.clone();
        let query = "struct".to_string(); // Hardcoded for demo
        let root_path = std::path::PathBuf::from("src"); // Search in src

        let async_cx = cx.to_async();
        let executor = cx.background_executor().clone();

        let background_task =
            executor.spawn(async move { crate::editor::search::perform_search(query, root_path) });

        // cx.spawn(|_, _| async move {
        //     let results = background_task.await;

        //     let _ = async_cx.update(|cx| {
        //         state.update(cx, |state, cx| {
        //             state.search_results = results;
        //             println!(
        //                 "Search complete. Found {} results.",
        //                 state.search_results.len()
        //             );
        //             cx.notify();
        //         });
        //     });
        // })
        // .detach();
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
            .on_action(cx.listener(MainScreen::toggle_file_switcher))
            .on_action(cx.listener(MainScreen::trigger_search))
            .on_action(cx.listener(|_, _: &Quit, _, cx| cx.quit()))
            .bg(rgb(0x252526))
            .text_color(PRIMARY_COLOR)
            .size_full()
            .flex()
            .flex_col()
            .child(self.header.render(window, cx))
            .child(
                div()
                    .flex()
                    .flex_row()
                    .flex_1()
                    .h_full()
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
                                    .bg(BACKGROUND_COLOR)
                                    .text_color(rgb(0xcccccc))
                                    .children(self.items.iter().enumerate().map(|(i, item)| {
                                        let is_active = i == self.active_item_index;
                                        let title = match item {
                                            WorkspaceItem::Editor(e) => {
                                                let editor = e.read(cx);
                                                editor
                                                    .file_path
                                                    .as_ref()
                                                    .and_then(|p| p.to_str())
                                                    .map(|s| s.to_owned())
                                                    .unwrap_or_else(|| "Editor".to_string())
                                            }
                                            WorkspaceItem::Settings(_) => "Settings".to_string(),
                                        };
                                        div()
                                            .h_full()
                                            .px_3()
                                            .flex()
                                            .items_center()
                                            .bg(if is_active {
                                                rgb(0x1e1e1e)
                                            } else {
                                                rgb(0x2d2d2d)
                                            })
                                            .border_r_1()
                                            .border_color(rgb(0x1e1e1e))
                                            .cursor_pointer()
                                            .on_mouse_down(
                                                MouseButton::Left,
                                                cx.listener(move |_, _, _, cx| {
                                                    cx.dispatch_action(&SwitchTab { index: i });
                                                }),
                                            )
                                            .child(div().text_xs().child(title))
                                            .when(is_active, |this| {
                                                this.child(
                                                    div()
                                                        .ml_2()
                                                        .child("x")
                                                        .hover(|s| s.text_color(rgb(0xffffff)))
                                                        .on_mouse_down(
                                                            MouseButton::Left,
                                                            cx.listener(|_, _, _, cx| {
                                                                cx.dispatch_action(&CloseTab);
                                                            }),
                                                        ),
                                                )
                                            })
                                    })),
                            )
                            // Content
                            .children(active_item.map(|item| item.render(cx))),
                    ),
            )
            .child(self.status_bar.render(cx)) // Bottom Bar
            .when(self.show_file_switcher, |this| {
                this.child(
                    div()
                        .absolute()
                        .top(px(40.0))
                        .left_1_2()
                        .w(px(400.0))
                        .ml(px(-200.0))
                        .bg(rgb(0x252526))
                        .border_1()
                        .border_color(rgb(0x454545))
                        .shadow_lg()
                        .flex()
                        .flex_col()
                        .children(self.items.iter().enumerate().map(|(i, item)| {
                            let is_active = i == self.active_item_index;
                            let title = match item {
                                WorkspaceItem::Editor(e) => {
                                    let editor = e.read(cx);
                                    editor
                                        .file_path
                                        .as_ref()
                                        .and_then(|p| p.to_str())
                                        .map(|s| s.to_owned())
                                        .unwrap_or_else(|| "Editor".to_string())
                                }
                                WorkspaceItem::Settings(_) => "Settings".to_string(),
                            };
                            div()
                                .px_4()
                                .py_2()
                                .text_color(if is_active {
                                    rgb(0xffffff)
                                } else {
                                    rgb(0xcccccc)
                                })
                                .bg(if is_active {
                                    rgb(0x04395e)
                                } else {
                                    rgb(0x252526)
                                })
                                .hover(|s| s.bg(rgb(0x2a2d2e)))
                                .cursor_pointer()
                                .child(div().text_xs().child(format!("{}", title)))
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(move |_, _, _, cx| {
                                        cx.dispatch_action(&SwitchTab { index: i });
                                        cx.dispatch_action(&ToggleFileSwitcher); // Close switcher
                                    }),
                                )
                        })),
                )
            })
    }
}
