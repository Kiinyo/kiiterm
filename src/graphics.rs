/// Used to reset the style of the screen
pub const RESET: &str = "\u{001B}[0m";
/// Styles
#[allow(non_camel_case_types)]
pub enum Style {
    // Bright
    Bright,
    // Bold
    Bold,
    // Dim
    Dim,
    // Italics
    Italics,
    // Underline
    Underline,
    // Slow blinking
    Slow_Blink,
    // Fast blinking
    Fast_Blink,
    // Invert
    Invert_Colors,
    // Invisible
    Invisible,
    // Strike through
    Strike_Through
}
pub fn parse_style (style: Style) -> String {
    let code: String;

    match style {
        Style::Bright => {code = String::from("\u{001B}[0m");}
        Style::Bold => {code = String::from("\u{001B}[1m");},
        Style::Dim => {code = String::from("\u{001B}[2m");},
        Style::Italics => {code = String::from("\u{001B}[3m");},
        Style::Underline => {code = String::from("\u{001B}[4m");},
        Style::Slow_Blink => {code = String::from("\u{001B}[5m");},
        Style::Fast_Blink => {code = String::from("\u{001B}[6m");},
        Style::Invert_Colors => {code = String::from("\u{001B}[7m");},
        Style::Invisible => {code = String::from("\u{001B}[8m");},
        Style::Strike_Through => {code = String::from("\u{001B}[9m");}
    }

    code
}
/// Colors
pub enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,

    RGB(u8, u8, u8)
}
pub enum Depth {
    Fg,
    Bg
}
pub fn parse_color (color: Color, depth: Depth) -> String {
    let mut value: u8;
    let code: String;
    let mut color_tuple: (u8, u8, u8) = (0, 0, 0);

    match color {
        Color::Black => value = 30,
        Color::Red => value = 31,
        Color::Green => value = 32,
        Color::Yellow => value = 33,
        Color::Blue => value = 34,
        Color::Magenta => value = 35,
        Color::Cyan => value = 36,
        Color::White => value = 37,

        Color::RGB(r, g, b) => {
            value = 38;
            color_tuple = (r, g, b);
        }
    }

    match depth {
        Depth::Fg => value += 0,
        Depth::Bg => value += 10
    }

    if value != 38 && value != 48 {
        code = format!("\u{001B}[{:?}m", value);
    } else {
        code = format!("\u{001B}[{:?};{};{};{}m", value, color_tuple.0, color_tuple.1, color_tuple.2)
    }

    code
}