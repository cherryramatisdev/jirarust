use colors_transform::{Color, Rgb};

pub enum ColorState {
    Brighter,
    Darker,
}

pub fn call(hex_color: &str) -> ColorState {
    let result = Rgb::from_hex_str(hex_color).unwrap();

    let (red, green, blue) = (
        result.get_red() as usize,
        result.get_green() as usize,
        result.get_blue() as usize,
    );

    return match is_color_lighter_or_darker(red, green, blue) {
        true => ColorState::Brighter,
        _ => ColorState::Darker,
    };
}

fn is_color_lighter_or_darker(r: usize, g: usize, b: usize) -> bool {
    let brightness = (r * 299 + g * 587 + b * 114) / 1000;

    brightness > 127
}
