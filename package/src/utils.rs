use gpui::{App, Action, KeyBinding, Rgba};

pub const fn hex_to_rgba(hex: u32) -> Rgba {
    Rgba {
        r: (((hex >> 16) & 0xFF) as f32) / 255.0,
        g: (((hex >> 8) & 0xFF) as f32) / 255.0,
        b: ((hex & 0xFF) as f32) / 255.0,
        a: 1.0,
    }
}

/// Registers a global keybinding for a specific action.
/// The action will be triggered regardless of the focused context,
/// unless a more specific binding intercepts it.
pub fn bind_global_action<A: Action + Clone>(cx: &mut App, keystroke: &str, action: A) {
    cx.bind_keys([
        KeyBinding::new(keystroke, action, None)
    ]);
}

/// Registers a keybinding that is active when the Editor is focused.
pub fn bind_editor_action<A: Action + Clone>(cx: &mut App, keystroke: &str, action: A) {
    cx.bind_keys([
        KeyBinding::new(keystroke, action, Some("Editor"))
    ]);
}
