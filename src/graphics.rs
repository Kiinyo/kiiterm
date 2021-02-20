use crate::screen;
/// Command used to reset the style of the screen.
const RESET: &str = "\u{001B}[0m";
#[allow(non_camel_case_types)]
#[derive(Copy, Clone)]
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
#[derive(Copy, Clone)]
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
fn move_cursor (x: usize, y: usize) -> String {
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

impl Clone for Glyph {
    fn clone(&self) -> Glyph {
        let glyph = Glyph {
            symbol: self.symbol.clone(),
            fg_color: self.fg_color.clone(),
            bg_color: self.bg_color.clone(),
            styles: self.styles.clone(),
        };
        return glyph
    }
}

/// Draw function to draw glyphs to the screen.
pub fn draw_glyph(screen: &mut screen::Screen, glyph: &Glyph, x: usize, y: usize) {
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
enum ParseFlag {
    None,

    Waiting,

    FgColor,
    BgColor,
    Style
}
// If you end a string with "\" it will be dropped so be careful!
pub fn parse_string_to_glyphs(string: String) -> Vec<Glyph> {
    // What we'll be returning
    let mut glyph_vec: Vec<Glyph> = Vec::new();
    // How to interpret the current char
    let mut flag = ParseFlag::None;
    // Colors
    let mut fg_color = Color::Default;
    let mut bg_color = Color::Default;
    // Styles
    let mut styles = vec![Style::Default];

    // Iterate through the string and add Glyphs to
    // the vector formatted with the above enums
    for c in string.chars() {
        // This is our escape code
        match flag {
            // We have a vanilla character
            ParseFlag::None => {
                if c == '\\' {
                    flag = ParseFlag::Waiting;
                } else {
                    glyph_vec.push(Glyph {
                        symbol: c.to_string(),
                        fg_color,
                        bg_color,
                        styles: styles.clone()
                    });
                }
            }
            // We are waiting for an instruction
            ParseFlag::Waiting => {
                match c {
                    'f' => {
                        flag = ParseFlag::FgColor
                    }
                    'b' => {
                        flag = ParseFlag::BgColor
                    }
                    's' => {
                        flag = ParseFlag::Style
                    }
                    _ => {
                        flag = ParseFlag::None;
                        glyph_vec.push(Glyph {
                            symbol: "\\".to_string(),
                            fg_color,
                            bg_color,
                            styles: styles.clone()
                        });
                        glyph_vec.push(Glyph {
                            symbol: c.to_string(),
                            fg_color,
                            bg_color,
                            styles: styles.clone()
                        });
                    }
                }
            }
            // We have our instruction and are waiting to carry it out
            ParseFlag::FgColor => {
                match c {
                    '0' => fg_color = Color::Black,
                    '1' => fg_color = Color::Red,
                    '2' => fg_color = Color::Green,
                    '3' => fg_color = Color::Yellow,
                    '4' => fg_color = Color::Blue,
                    '5' => fg_color = Color::Magenta,
                    '6' => fg_color = Color::Cyan,
                    '7' => fg_color = Color::White,
                    _ => fg_color = Color::Default,
                };
                flag = ParseFlag::None;
            }
            ParseFlag::BgColor => {
                match c {
                    '0' => bg_color = Color::Black,
                    '1' => bg_color = Color::Red,
                    '2' => bg_color = Color::Green,
                    '3' => bg_color = Color::Yellow,
                    '4' => bg_color = Color::Blue,
                    '5' => bg_color = Color::Magenta,
                    '6' => bg_color = Color::Cyan,
                    '7' => bg_color = Color::White,
                    _ => bg_color = Color::Default,
                };
                flag = ParseFlag::None;
            }
            ParseFlag::Style => {
                match c {
                    '0' => styles.push(Style::Bright),
                    '1' => styles.push(Style::Bold),
                    '2' => styles.push(Style::Dim),
                    '3' => styles.push(Style::Italics),
                    '4' => styles.push(Style::Underline),
                    '5' | '6' => styles.push(Style::Blink),
                    '7' => styles.push(Style::Invert_Colors),
                    '8' => styles.push(Style::Invisible),
                    '9' => styles.push(Style::Strike_Through),

                    ';' => flag = ParseFlag::None,

                    _ => styles = vec![Style::Default]
                }
            }
        }
    }
    
    return glyph_vec;
}
pub fn draw_buffer(screen: &mut screen::Screen, buffer: Vec<Vec<Glyph>>, x: usize, y: usize) {
    for y1 in 0..buffer.len() {
        for x1 in 0..buffer[y].len() {
            draw_glyph(screen, &buffer[y][x], x + x1, y + y1)
        }
    }
}