use crate::screen;
/// Used to reset the style of the screen
pub const RESET: &str = "\u{001B}[0m";
/// Styles
#[allow(non_camel_case_types)]
/// All the different styles possible!
pub enum Style {
    Default,

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
    // Blink
    Blink,
    // Invert
    Invert_Colors,
    // Invisible
    Invisible,
    // Strike through
    Strike_Through
}
fn parse_style (style: &Style) -> String {
    let code: String;

    match style {
        Style::Default => {code = String::new();},

        Style::Bright => {code = String::from("\u{001B}[0m");}
        Style::Bold => {code = String::from("\u{001B}[1m");},
        Style::Dim => {code = String::from("\u{001B}[2m");},
        Style::Italics => {code = String::from("\u{001B}[3m");},
        Style::Underline => {code = String::from("\u{001B}[4m");},
        Style::Blink => {code = String::from("\u{001B}[5m");},
        Style::Invert_Colors => {code = String::from("\u{001B}[7m");},
        Style::Invisible => {code = String::from("\u{001B}[8m");},
        Style::Strike_Through => {code = String::from("\u{001B}[9m");}
    }

    code
}
/// All the different colors possible!
pub enum Color {
    Default,
    
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
enum Depth {
    Fg,
    Bg
}
fn parse_color (color: &Color, depth: &Depth) -> String {
    let mut value: u8;
    let code: String;
    let mut color_tuple: (u8, u8, u8) = (0, 0, 0);

    match color {
        Color::Default => {return String::new()},

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
            color_tuple = (*r, *g, *b);
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
pub fn move_cursor (x: u16, y: u16) -> String {
    format!("\u{001B}[{};{}H", y + 1, x + 1)
}
/// The most basic unit that can be drawn to the terminal,
/// can be seen as the equivalent to a pixel
pub struct Glyph {
    pub symbol: String,

    pub fg_color: Color,
    pub bg_color: Color,

    pub styles: Vec<Style>
}
/// Draw function to draw glyphs to the screen.
pub fn draw_glyph(screen: &mut screen::Screen, glyph: &Glyph, x: u16, y: u16) {
    let mut code  = move_cursor(x, y);

    for style in glyph.styles.iter() {
        code = format!("{}{}", code, parse_style(style));
    }

    

    code = format!("{}{}{}{}{}", code,
        parse_color(&glyph.fg_color, &Depth::Fg),
        parse_color(&glyph.bg_color, &Depth::Bg),
        glyph.symbol, 
        RESET
    );

    screen::draw_to_buffer(screen, code);
}