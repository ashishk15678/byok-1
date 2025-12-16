use gpui::Rgba;

pub const fn hex_to_rgba(hex: u32) -> Rgba {
    Rgba {
        r: (((hex >> 16) & 0xFF) as f32) / 255.0,
        g: (((hex >> 8) & 0xFF) as f32) / 255.0,
        b: ((hex & 0xFF) as f32) / 255.0,
        a: 1.0,
    }
}
