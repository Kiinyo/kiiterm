use crate::terminal;
/// The structure that holds everything needed to handle inputs and draw to thet terminal
pub struct Screen {
    pub width: u16,
    pub height: u16,
    pub context: termion::input::MouseTerminal<termion::raw::RawTerminal<std::io::Stdout>>,
    pub inputs: termion::AsyncReader
}
/// Create the Screen to be used by nearly everything
pub fn init (width: u16, height: u16) -> Screen {
    // Let's first clear the terminal
    terminal::clear();
    // Then we'll get the terminal's widith and height after the resize
    let (width, height) = terminal::resize(width, height);

    // Unfortunately I seem to have to need this to run into_raw_mode()
    use termion::raw::IntoRawMode;
    // Create the context we'll be drawing things to
    let context= termion::input::MouseTerminal::from(std::io::stdout().into_raw_mode().unwrap());
    // Get a way to funnel player inputs
    let inputs = termion::async_stdin();

    // Finally return the screen to be used
    Screen {width, height, context, inputs}
}

pub mod input {
    #[allow(non_camel_case_types)]
    #[derive(Debug)]
    pub enum Key {
        // a..Z
        Char(char),
        Alt_Char(char),
        // A..Z
        // The terminal doe not recognize cae with Ctrl
        Ctrl_Char(char),
        Ctrl_Alt_Char(char),

        // Any combination not listed is due to a limitation
        // of the terminal (...usually)

        // Backspace

        Backspace,

        Alt_Backspace,

        // Delete

        Delete,

        Shift_Delete,
        Alt_Delete,
        Ctrl_Delete,

        // Arrow Keys

        Up,
        Down,
        Right,
        Left,

        Shift_Up,
        Shift_Down,
        Shift_Right,
        Shift_Left,

        Alt_Up,
        Alt_Down,
        Alt_Right,
        Alt_Left,

        Ctrl_Up,
        Ctrl_Down,
        Ctrl_Right,
        Ctrl_Left,

        // Escape - No other variations work it also
        // shares 27 with escape codes so if escape
        // is pressed with another key in the same 
        // frame it will not register so be careful!
        Escape,

        // Insert

        Insert,

        Alt_Insert,

        // Home

        Home,

        // Page
        
        Page_Up,
        Page_Down,

        // End

        End,

        // Enter

        Enter,

        Alt_Enter,

        // Null, junk/invalid/no inputs

        Null
    }
    #[derive(Debug)]
    pub enum Modifier {
        Alt,
        Shift,
        Ctrl,

        None
    }
    /// Follows Enum(x, y, Modifier)    
    #[allow(non_camel_case_types)]
    #[derive(Debug)]
    pub enum Mouse {
        // Left Mouse Button
        LMB_Press {x: u16, y: u16, modifier: Modifier},
        LMB_Release {x: u16, y: u16, modifier: Modifier},
        LMB_Move {x: u16, y: u16, modifier: Modifier},

        // Middle Mouse Button
        MMB_Press {x: u16, y: u16, modifier: Modifier},
        MMB_Release {x: u16, y: u16, modifier: Modifier},
        MMB_Move {x: u16, y: u16, modifier: Modifier},

        // Right Mouse Button
        RMB_Press {x: u16, y: u16, modifier: Modifier},
        RMB_Release {x: u16, y: u16, modifier: Modifier},
        RMB_Move {x: u16, y: u16, modifier: Modifier},

        // Scrolling
        Scroll_Up {x: u16, y: u16, modifier: Modifier},
        Scroll_Down {x: u16, y: u16, modifier: Modifier}
    }

    // This takes the buffer from Screen
    pub fn parse_input (buffer: &Vec<u8>) -> Vec<Key> {
        let mut inputs = Vec::new();
        let mut index: usize = 0;
        let length: usize = buffer.len();

        'new_input: loop {
            if index < length {
                match buffer[index] {
                    // It's Ctrl + A..Z
                    1..=12 | 14..=26 => {
                        inputs.push(Key::Ctrl_Char(parse_char(buffer[index] + 31)));
                    },
                    13 => {
                        inputs.push(Key::Enter);
                    }
                    // We've hit an escape code
                    27 => {
                        if index < length {
                            index += 1;

                            // Let's figure out what the escape code says!
                            match buffer[index] {
                                // It's Ctrl + Alt + Char
                                1..=12 | 14..=26 => {
                                    inputs.push(Key::Ctrl_Alt_Char(parse_char(buffer[index] + 31)));
                                },
                                13 => {
                                    inputs.push(Key::Alt_Enter)
                                },
                                // Another escape code
                                27 => {
                                    inputs.push(Key::Escape);
                                    continue 'new_input;
                                },
                                // A command!
                                91 => {
                                    if index < length {
                                        index += 1;

                                        match buffer[index] {

                                            // Modified arrow keys
                                            49 => {
                                                if index + 3 < length {
                                                    index += 1;
                                                    if buffer[index] == 59 {
                                                        index += 1;
                                                        if buffer[index] == 50 {
                                                            index += 1;
                                                            if buffer[index] == 65 {
                                                                inputs.push(Key::Shift_Up);
                                                            } else if buffer[index] == 66 {
                                                                inputs.push(Key::Shift_Down);
                                                            } else if buffer[index] == 67 {
                                                                inputs.push(Key::Shift_Right);
                                                            } else if buffer[index] == 68 {
                                                                inputs.push(Key::Shift_Left);
                                                            } else {
                                                                inputs.push(Key::Null);
                                                            }
                                                        } else if buffer[index] == 51 {
                                                            index += 1;
                                                            if buffer[index] == 65 {
                                                                inputs.push(Key::Alt_Up);
                                                            } else if buffer[index] == 66 {
                                                                inputs.push(Key::Alt_Down);
                                                            } else if buffer[index] == 67 {
                                                                inputs.push(Key::Alt_Right);
                                                            } else if buffer[index] == 68 {
                                                                inputs.push(Key::Alt_Left);
                                                            } else {
                                                                inputs.push(Key::Null);
                                                            }
                                                        } else if buffer[index] == 53 {
                                                            index += 1;
                                                            if buffer[index] == 65 {
                                                                inputs.push(Key::Ctrl_Up);
                                                            } else if buffer[index] == 66 {
                                                                inputs.push(Key::Ctrl_Down);
                                                            } else if buffer[index] == 67 {
                                                                inputs.push(Key::Ctrl_Right);
                                                            } else if buffer[index] == 68 {
                                                                inputs.push(Key::Ctrl_Left);
                                                            } else {
                                                                inputs.push(Key::Null);
                                                            }
                                                        }
                                                    } else {
                                                        inputs.push(Key::Null);
                                                    }
                                                } else {
                                                    inputs.push(Key::Null);
                                                }
                                            },
                                            // Insert
                                            50 => {
                                                if index < length {
                                                    index += 1;

                                                    if buffer[index] == 59 {
                                                        inputs.push(Key::Alt_Insert);
                                                        // Skipping over [51, 126]
                                                        index += 2;
                                                    } else if buffer[index] == 126 {
                                                        inputs.push(Key::Insert)
                                                    } else {
                                                        inputs.push(Key::Null)
                                                    }

                                                } else {
                                                    inputs.push(Key::Null)
                                                }
                                            },
                                            // Delete command
                                            51 => {
                                                if index < length {
                                                    index += 1;
                                                    
                                                    match buffer[index] {
                                                        59 => {
                                                            if index < length {
                                                                index += 1;

                                                                match buffer[index] {
                                                                    50 => {
                                                                        inputs.push(Key::Shift_Delete);
                                                                        // Skip the 126 ending
                                                                        index += 1;
                                                                    },
                                                                    51 => {
                                                                        inputs.push(Key::Alt_Delete);
                                                                        // Skip the 126 ending
                                                                        index += 1;
                                                                    },
                                                                    53 =>{
                                                                        inputs.push(Key::Ctrl_Delete);
                                                                        // Skip the 126 ending
                                                                        index += 1;
                                                                    },
                                                                    _ => inputs.push(Key::Null)
                                                                }
                                                            } else {
                                                                inputs.push(Key::Null);
                                                            }
                                                        },
                                                        126 => inputs.push(Key::Delete),
                                                        _ => inputs.push(Key::Null)
                                                    }
                                                } else {
                                                    inputs.push(Key::Null);
                                                    break 'new_input;
                                                }
                                            },
                                            53 => {},
                                            54 => {},
                                            // Mouse Event
                                            60 => {},
                                            65..=68 => {
                                                if buffer[index] == 65 {
                                                    inputs.push(Key::Up);
                                                } else if buffer[index] == 66 {
                                                    inputs.push(Key::Down);
                                                } else if buffer[index] == 67 {
                                                    inputs.push(Key::Right);
                                                } else if buffer[index] == 68 {
                                                    inputs.push(Key::Left);
                                                }
                                            },
                                            _ => {
                                                inputs.push(Key::Null);
                                            }
                                        }

                                    } else {
                                        inputs.push(Key::Char('['));
                                        break 'new_input;
                                    }
                                }
                                // It's Alt + Char
                                33..=90 | 92..=126 => {
                                    inputs.push(Key::Alt_Char(parse_char(buffer[index] - 33)));
                                }
                                _ => {
                                    inputs.push(Key::Null);
                                }
                            }

                        } else {
                            inputs.push(Key::Escape);
                            break;
                        };

                    },
                    // It's just a character!
                    33..=126 => {
                        inputs.push(Key::Char(parse_char(buffer[index] - 33)));
                    }
                    _ => {
                        inputs.push(Key::Null);
                    }
                }
                index += 1;
            } else { break; }
        }

        return inputs
    }
    /// If coming from a UTF-8 character subtract 33!
    /// This is simply a backend for me because I'm
    /// not smart enough to figure out how Rust and
    /// Termion do it! 
    pub fn parse_char (character: u8) -> char {
        match character {
            0 => '!',
            1 => '"',
            2 => '#',
            3 => '$',
            4 => '%',
            5 => '&',
            6 => '\'',
            7 => '(',
            8 => ')',
            9 => '*',
            10 => '+',
            11 => ',',
            12 => '-',
            13 => '.',
            14 => '/',
            // Numbers
            15 => '0',
            16 => '1',
            17 => '2',
            18 => '3',
            19 => '4',
            20 => '5',
            21 => '6',
            22 => '7',
            23 => '8',
            24 => '9',
            // Special Characters
            25 => ':',
            26 => ';',
            27 => '<',
            28 => '=',
            29 => '>',
            30 => '?',
            31 => '@',
            // Capitalized Characters
            32 => 'A',
            33 => 'B',
            34 => 'C',
            35 => 'D',
            36 => 'E',
            37 => 'F',
            38 => 'G',
            39 => 'H',
            40 => 'I',
            41 => 'J',
            42 => 'K',
            43 => 'L',
            44 => 'M',
            45 => 'N',
            46 => 'O',
            47 => 'P',
            48 => 'Q',
            49 => 'R',
            50 => 'S',
            51 => 'T',
            52 => 'U',
            53 => 'V',
            54 => 'W',
            55 => 'X',
            56 => 'Y',
            57 => 'Z',
            // Special Characters
            58 => '[',
            59 => '\\',
            60 => ']',
            61 => '^',
            62 => '_',
            63 => '`',
            // Lowercase Characters
            64 => 'a',
            65 => 'b',
            66 => 'c',
            67 => 'd',
            68 => 'e',
            69 => 'f',
            70 => 'g',
            71 => 'h',
            72 => 'i',
            73 => 'j',
            74 => 'k',
            75 => 'l',
            76 => 'm',
            77 => 'n',
            78 => 'o',
            79 => 'p',
            80 => 'q',
            81 => 'r',
            82 => 's',
            83 => 't',
            84 => 'u',
            85 => 'v',
            86 => 'w',
            87 => 'x',
            88 => 'y',
            89 => 'z',
            // Special Characters
            90 => '{',
            91 => '|',
            92 => '}',
            93 => '~',
            _ => panic!("{} Is not a number that's covered!", character)

        }
    }
} 