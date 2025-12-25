use crate::config::APP_NAME;
use crate::editor::texteditor::{
    Copy, Cut, NewFile, OpenFile, Paste, Redo, SaveFile, SaveFileAs, TextEditor, Undo,
};
use crate::state::appstate::AppState;
use crate::ui::workspace::{
    CloseTab, MainScreen, OpenSettings, Quit, ToggleBrowser, ToggleFileSwitcher, ToggleInfoPanel,
};
use crate::utils::{bind_editor_action, bind_global_action};
use gpui::{
    App, Application, Bounds, Menu, MenuItem, WindowBounds, WindowOptions, prelude::*, px, size, KeyBinding,
};
use std::env;

pub mod config;
pub mod editor;
pub mod log;
pub mod pools;
pub mod state;
pub mod structs;
pub mod tests;
pub mod ui;
pub mod utils;
pub mod workspace;
fn main() {
    let _env: Vec<String> = env::args().collect();
    Application::new().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(1000.), px(800.0)), cx);

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
                cx.set_menus(vec![menu]);

                // keybindings
                bind_global_action(cx, "ctrl-b", ToggleBrowser);
                bind_global_action(cx, "ctrl-l", ToggleInfoPanel);
                bind_global_action(cx, "ctrl-shift-p", OpenSettings);
                bind_global_action(cx, "ctrl-,", OpenSettings);
                bind_global_action(cx, "ctrl-p", ToggleFileSwitcher);
                cx.bind_keys([
                    KeyBinding::new("ctrl-f", crate::ui::workspace::ToggleSearch { global: false }, None),
                    KeyBinding::new("ctrl-shift-f", crate::ui::workspace::ToggleSearch { global: true }, None),
                ]);

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
