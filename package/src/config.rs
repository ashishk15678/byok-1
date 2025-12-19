use crate::utils::hex_to_rgba;
use gpui::Rgba;

pub const APP_NAME: &str = "Zed";

pub const CONFIG_FILE_NAME: &str = "config.toml";

// colours
pub const PRIMARY_COLOR: Rgba = hex_to_rgba(0x3498db);
pub const SECONDARY_COLOR: Rgba = hex_to_rgba(0x2ecc71);
pub const ACCENT_COLOR: Rgba = hex_to_rgba(0xe74c3c);
pub const BACKGROUND_COLOR: Rgba = hex_to_rgba(0x292929);

// fonts
pub const FONT_FAMILY: &str = "Roboto";
pub const BASE_FONT_SIZE: f32 = 16.0;

// other config options
pub const DEFAULT_WINDOW_WIDTH: f32 = 800.0;
pub const DEFAULT_WINDOW_HEIGHT: f32 = 600.0;

pub const DIR_ICON: &str = "📁";
pub const FILE_ICON: &str = "📄";

pub const INITIAL_LOAD_LINES: usize = 100;
pub const CHUNK_LOAD_LINES: usize = 500;
