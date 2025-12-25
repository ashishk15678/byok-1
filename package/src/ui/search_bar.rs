use gpui::*;
use gpui::prelude::*;
use serde::Deserialize;

#[derive(Clone, PartialEq, Debug, Deserialize)]
pub struct PerformSearch {
    pub query: String,
    pub global: bool,
}

impl gpui::Action for PerformSearch {
    fn name(&self) -> &'static str { "PerformSearch" }
    fn name_for_type() -> &'static str { "PerformSearch" }
    fn build(value: serde_json::Value) -> gpui::Result<Box<dyn gpui::Action>> {
        let action: PerformSearch = serde_json::from_value(value)?;
        Ok(Box::new(action))
    }
    fn boxed_clone(&self) -> Box<dyn gpui::Action> { Box::new(self.clone()) }
    fn partial_eq(&self, action: &dyn gpui::Action) -> bool {
        action.as_any().downcast_ref::<Self>().map_or(false, |a| self == a)
    }
}

#[derive(Clone)]
pub struct SearchBar {
    pub query: String,
    pub is_global: bool,
    pub is_visible: bool,
    pub focus_handle: FocusHandle,
}

impl SearchBar {
    pub fn new(cx: &mut Context<MainScreen>) -> Self {
        Self {
            query: String::new(),
            is_global: false, // Default to local
            is_visible: false,
            focus_handle: cx.focus_handle(),
        }
    }

    pub fn toggle(&mut self, global: bool, window: &mut Window, cx: &mut Context<MainScreen>) {
        if self.is_visible && self.is_global == global {
            self.is_visible = false;
            // logic to return focus to editor would go here
        } else {
            self.is_visible = true;
            self.is_global = global;
            self.query.clear();
            window.focus(&self.focus_handle);
        }
        cx.notify();
    }
}

use crate::ui::workspace::MainScreen;

impl SearchBar {
    pub fn render(&self, _window: &mut Window, cx: &mut Context<MainScreen>) -> impl IntoElement {
        if !self.is_visible {
             return div();
        }
 
        let query = self.query.clone();
        
        div()
            .track_focus(&self.focus_handle)
            .flex()
            .flex_row()
            .items_center()
            .bg(rgb(0x252526))
            .border_1()
            .border_color(rgb(0x454545))
            .p_2()
            .gap_2()
            .on_key_down(cx.listener(|this, event: &KeyDownEvent, _, cx| {
                let text = event.keystroke.key.as_str();
                match text {
                    "backspace" => {
                        this.search_bar.query.pop();
                    }
                    "escape" => {
                        this.search_bar.is_visible = false;
                    }
                    _ => {
                        if text.len() == 1 {
                             this.search_bar.query.push_str(text);
                             // Dispatch local search immediately
                             if !this.search_bar.is_global {
                                 cx.dispatch_action(&PerformSearch {
                                     query: this.search_bar.query.clone(),
                                     global: false,
                                 });
                             }
                        }
                    }
                }
                
                // If Enter, force search (useful for global which might be heavy)
                if text == "enter" {
                    cx.dispatch_action(&PerformSearch {
                        query: this.search_bar.query.clone(),
                        global: this.search_bar.is_global,
                    });
                }
                
                cx.notify();
            }))
            .child(
                div()
                    .text_xs()
                    .text_color(rgb(0xcccccc))
                    .child(if self.is_global { "Global Search:" } else { "Find:" })
            )
            .child(
                div()
                    .text_sm()
                    .text_color(rgb(0xffffff))
                    .bg(rgb(0x1e1e1e))
                    .border_1()
                    .border_color(rgb(0x555555))
                    .px_2()
                    .py_1()
                    .min_w(px(200.0))
                    .child(if query.is_empty() { "Type to search...".to_string() } else { query })
            )
    }
}
