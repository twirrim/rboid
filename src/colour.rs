use colors_transform::{Color, Hsl};

pub fn get_colour_by_width(x: f32, width: u32) -> ggez::graphics::Color {
    let width = width as f32;
    let h_per = 360.0 / width;
    let hsl = Hsl::from(h_per * x, 100.0, 50.0);
    let rgb = hsl.to_rgb();
    ggez::graphics::Color::from_rgb(
        rgb.get_red() as u8,
        rgb.get_green() as u8,
        rgb.get_blue() as u8,
    )
}
