use crate::screen;
/// Command used to reset the style of the screen.
const RESET: &str = "\u{001B}[0m";
#[allow(non_camel_case_types)]
/// All the different styles possible for glyphs.
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
/// Parsing the style enum into a command for the terminal.
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
/// All the different colors possible for glyphs.
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
/// What I use to determine if something's foreground or background.
/// A helper enum for parse_color.
enum Depth {
    Fg,
    Bg
}
/// Parsing the color enum into a command for the terminal.
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
/// Parsing (x, y) coordinates into a command for the terminal.
fn move_cursor (x: u16, y: u16) -> String {
    format!("\u{001B}[{};{}H", y + 1, x + 1)
}
/// The most basic unit that can be drawn to the terminal,
/// can be seen as the equivalent to a pixel.
pub struct Glyph {
    // The actual String that will be drawn on the screen
    pub symbol: String,
    // The corresponding colors
    pub fg_color: Color,
    pub bg_color: Color,
    // Any styles the glyph might have
    pub styles: Vec<Style>
}
/// Draw function to draw glyphs to the screen.
pub fn draw_glyph(screen: &mut screen::Screen, glyph: &Glyph, x: u16, y: u16) {
    // Start the code variable we'll be drawing to the buffer
    let mut code  = move_cursor(x, y);
    // Add the style(s) to it
    for style in glyph.styles.iter() {
        code = format!("{}{}", code, parse_style(style));
    }
    // Append the rest of the formatting from the glyph
    code = format!("{}{}{}{}{}", code,
        // Foreground color
        parse_color(&glyph.fg_color, &Depth::Fg),
        // Background color
        parse_color(&glyph.bg_color, &Depth::Bg),
        // The actual symbol (or text) to be drawn
        glyph.symbol,
        // Setting the terminal back to its default state
        RESET
    );
    // Finally draw the code to the buffer
    screen::buffer_write(screen, code);
}