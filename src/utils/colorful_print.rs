#[derive(Debug)]
pub enum Color {
    White,
    Black,
}

pub fn get_colorful_text(text: &str, color: Color) -> String {
    let color_code = match color {
        Color::Black => 30,
        Color::White => 37,
    };

    format!("\x1b[{}m{}\x1b[0m", color_code, text)
}
