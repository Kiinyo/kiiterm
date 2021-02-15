use std::io::Write;

use crate::terminal;
use crate::Input;
use crate::Input_Modifier;
/// The structure that holds everything needed to handle inputs and draw to thet terminal
pub struct Screen {
    pub width: u16,
    pub height: u16,
    context: termion::input::MouseTerminal<termion::raw::RawTerminal<std::io::Stdout>>,
    inputs: termion::AsyncReader
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

/// Hide the screen's cursor
pub fn hide_cursor(screen: &mut Screen) {
    write!(screen.context,"\u{001B}[?25l").unwrap();
}
/// Show the screen's cursor
pub fn show_cursor(screen: &mut Screen) {
    write!(screen.context,"\u{001B}[?25h").unwrap();
}

/// Get a Vec of player Inputs, this returns every interaction
/// with the window since the last time this function was run.
pub fn get_inputs (screen: &mut Screen) -> Vec<Input> {
    use std::io::Read;

    let mut buffer: Vec<u8> = Vec::new();
    screen.inputs.read_to_end(&mut buffer).unwrap();

    parse_buffer(&buffer)


}
/// Used for troubleshooting difference between the buffer and
/// the inputs being parsed.
pub fn debug_inputs (screen: &mut Screen) -> (Vec<Input>, Vec<u8>) {
    use std::io::Read;

    let mut buffer: Vec<u8> = Vec::new();
    screen.inputs.read_to_end(&mut buffer).unwrap();

    (parse_buffer(&buffer), buffer)


}

/// A function for drawing to the screen's buffer.
pub fn draw_to_buffer(screen: &mut Screen, string: String) {
    write!(screen.context, "{}", string).unwrap();
}
/// Draw the buffer to the screen!
pub fn display_buffer(screen: &mut Screen) {
    screen.context.flush().unwrap();
}

/// Helper function for parsing numbers that appear in the buffer.
fn parse_numbers (buffer: &Vec<u8>, mut start: usize) -> (u16, usize) {
    let mut number: u16 = 0;
    let mut place: u16 = 1;
    let mut inc: usize = 0;
    let length: usize = buffer.len();

    loop {
       if start < length {
           match buffer[start] {

               48..=57 => {
                   number += (buffer[start] as u16 - 48u16) * place;
               },
               _ => {
                   break;
               }
            }
        } else {
            break;
        }
       start += 1;
       inc += 1;
       place *= 10;
    }

    let number: String = format!("{:?}", number);
    let number: String = number.chars().rev().collect::<String>();
    let number: u16 = number.parse().unwrap();

    (number, inc)
}
/// Helper function to converting buffer into inputs.
fn parse_buffer (buffer: &Vec<u8>) -> Vec<Input> {
    let mut inputs = Vec::new();
    let mut index: usize = 0;
    let length: usize = buffer.len();

    'new_input: loop {
        if index < length {
            match buffer[index] {
               
                // It's Ctrl + A..Z
                1..=12 | 14..=26 => {
                    inputs.push(Input::Ctrl_Char(parse_char(buffer[index] + 31)));
                },
                13 => {
                    inputs.push(Input::Enter);
                }
                // We've hit an escape code
                27 => {
                    index += 1;
                    if index < length {

                        // Let's figure out what the escape code says!
                        match buffer[index] {
                            // It's Ctrl + Alt + Char
                            1..=12 | 14..=26 => {
                                inputs.push(Input::Ctrl_Alt_Char(parse_char(buffer[index] + 31)));
                            },
                            // Enter
                            13 => {
                                inputs.push(Input::Alt_Enter)
                            },
                            // Another escape code
                            27 => {
                                inputs.push(Input::Escape);
                                continue 'new_input;
                            },
                            // A command!
                            91 => {
                                index += 1;
                                if index < length {

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
                                                            inputs.push(Input::Shift_Up);
                                                        } else if buffer[index] == 66 {
                                                            inputs.push(Input::Shift_Down);
                                                        } else if buffer[index] == 67 {
                                                            inputs.push(Input::Shift_Right);
                                                        } else if buffer[index] == 68 {
                                                            inputs.push(Input::Shift_Left);
                                                        } else {
                                                            inputs.push(Input::Null);
                                                        }
                                                    } else if buffer[index] == 51 {
                                                        index += 1;
                                                        if buffer[index] == 65 {
                                                            inputs.push(Input::Alt_Up);
                                                        } else if buffer[index] == 66 {
                                                            inputs.push(Input::Alt_Down);
                                                        } else if buffer[index] == 67 {
                                                            inputs.push(Input::Alt_Right);
                                                        } else if buffer[index] == 68 {
                                                            inputs.push(Input::Alt_Left);
                                                        } else {
                                                            inputs.push(Input::Null);
                                                        }
                                                    } else if buffer[index] == 53 {
                                                        index += 1;
                                                        if buffer[index] == 65 {
                                                            inputs.push(Input::Ctrl_Up);
                                                        } else if buffer[index] == 66 {
                                                            inputs.push(Input::Ctrl_Down);
                                                        } else if buffer[index] == 67 {
                                                            inputs.push(Input::Ctrl_Right);
                                                        } else if buffer[index] == 68 {
                                                            inputs.push(Input::Ctrl_Left);
                                                        } else {
                                                            inputs.push(Input::Null);
                                                        }
                                                    }
                                                } else {
                                                    inputs.push(Input::Null);
                                                }
                                            } else {
                                                inputs.push(Input::Null);
                                            }
                                        },

                                        // Insert
                                        50 => {
                                            index += 1;
                                            if index < length {

                                                if buffer[index] == 59 {
                                                    inputs.push(Input::Alt_Insert);
                                                    // Skipping over [51, 126]
                                                    index += 2;
                                                } else if buffer[index] == 126 {
                                                    inputs.push(Input::Insert)
                                                } else {
                                                    inputs.push(Input::Null)
                                                }

                                            } else {
                                                inputs.push(Input::Null)
                                            }
                                        },

                                        // Delete command
                                        51 => {
                                            index += 1;
                                            if index < length {
                                                
                                                match buffer[index] {
                                                    59 => {
                                                        if index < length {
                                                            index += 1;

                                                            match buffer[index] {
                                                                50 => {
                                                                    inputs.push(Input::Shift_Delete);
                                                                    // Skip the 126 ending
                                                                    index += 1;
                                                                },
                                                                51 => {
                                                                    inputs.push(Input::Alt_Delete);
                                                                    // Skip the 126 ending
                                                                    index += 1;
                                                                },
                                                                53 =>{
                                                                    inputs.push(Input::Ctrl_Delete);
                                                                    // Skip the 126 ending
                                                                    index += 1;
                                                                },
                                                                _ => inputs.push(Input::Null)
                                                            }
                                                        } else {
                                                            inputs.push(Input::Null);
                                                        }
                                                    },
                                                    126 => inputs.push(Input::Delete),
                                                    _ => inputs.push(Input::Null)
                                                }
                                            } else {
                                                inputs.push(Input::Null);
                                                break 'new_input;
                                            }
                                        },

                                        // Page Up
                                        53 => {
                                            index += 1;
                                            if index < length {
                                                if buffer[index] == 126 {
                                                    inputs.push(Input::Page_Down)
                                                } else if buffer[index] == 59 {
                                                    index += 1;
                                                    if index + 1 < length {
                                                        if buffer[index] == 50 {
                                                            index += 1;
                                                            if buffer[index] == 126 {
                                                                inputs.push(Input::Shift_Page_Up);
                                                            } else {
                                                                inputs.push(Input::Null);
                                                            }
                                                        } else if buffer[index] == 51 {
                                                            index += 1;
                                                            if buffer[index] == 126 {
                                                                inputs.push(Input::Alt_Page_Up);
                                                            } else {
                                                                inputs.push(Input::Null);
                                                            }
                                                        } else if buffer[index] == 53 {
                                                            index += 1;
                                                            if buffer[index] == 126 {
                                                                inputs.push(Input::Ctrl_Page_Up);
                                                            } else {
                                                                inputs.push(Input::Null);
                                                            }
                                                        } else if buffer[index] == 55 {
                                                            index += 1;
                                                            if buffer[index] == 126 {
                                                                inputs.push(Input::Ctrl_Alt_Page_Up);
                                                            } else {
                                                                inputs.push(Input::Null);
                                                            }
                                                        } else {
                                                            inputs.push(Input::Null)
                                                        }
                                                    } else {
                                                        inputs.push(Input::Null)
                                                    }
                                                } else {
                                                    inputs.push(Input::Null)
                                                }
                                            } else {
                                                inputs.push(Input::Null);
                                            }
                                        },
                                        // Page Down
                                        54 => {
                                            index += 1;
                                            if index < length {
                                                if buffer[index] == 126 {
                                                    inputs.push(Input::Page_Down)
                                                } else if buffer[index] == 59 {
                                                    index += 1;
                                                    if index + 1 < length {
                                                        if buffer[index] == 50 {
                                                            index += 1;
                                                            if buffer[index] == 126 {
                                                                inputs.push(Input::Shift_Page_Down);
                                                            } else {
                                                                inputs.push(Input::Null);
                                                            }
                                                        } else if buffer[index] == 51 {
                                                            index += 1;
                                                            if buffer[index] == 126 {
                                                                inputs.push(Input::Alt_Page_Down);
                                                            } else {
                                                                inputs.push(Input::Null);
                                                            }
                                                        } else if buffer[index] == 53 {
                                                            index += 1;
                                                            if buffer[index] == 126 {
                                                                inputs.push(Input::Ctrl_Page_Down);
                                                            } else {
                                                                inputs.push(Input::Null);
                                                            }
                                                        } else if buffer[index] == 55 {
                                                            index += 1;
                                                            if buffer[index] == 126 {
                                                                inputs.push(Input::Ctrl_Alt_Page_Down);
                                                            } else {
                                                                inputs.push(Input::Null);
                                                            }
                                                        } else {
                                                            inputs.push(Input::Null)
                                                        }
                                                    } else {
                                                        inputs.push(Input::Null)
                                                    }
                                                } else {
                                                    inputs.push(Input::Null)
                                                }
                                            } else {
                                                inputs.push(Input::Null);
                                            }
                                        },

                                        // Mouse Event
                                        60 => {
                                            index += 1;
                                            if index + 5 < length {
                                                match buffer[index] {
                                                    // Vanilla Clicks
                                                    48..=50 => {
                                                        let remember_me = index;
                                                        index += 1;

                                                        if buffer[index] == 59 {
                                                            index += 1;
                                                            let (x, inc ) = parse_numbers(buffer, index);
                                                            index += 1 + inc;
                                                            let (y, inc) = parse_numbers(buffer, index); 
                                                            index += inc;
                                                            if buffer[index] == 77 {
                                                                //  It's being Pressed
                                                                if buffer[remember_me] == 48 {
                                                                    inputs.push(Input::LMB_Press{x, y, modifier: Input_Modifier::Null});
                                                                } else if buffer[remember_me] == 49 {
                                                                    inputs.push(Input::MMB_Press{x, y, modifier: Input_Modifier::Null});
                                                                } else if buffer[remember_me] == 50 {
                                                                    inputs.push(Input::RMB_Press{x, y, modifier: Input_Modifier::Null});
                                                                } else {
                                                                    inputs.push(Input::Null);
                                                                }
                                                            } else if buffer[index] == 109 {
                                                                // It's being Released
                                                                if buffer[remember_me] == 48 {
                                                                    inputs.push(Input::LMB_Release{x, y, modifier: Input_Modifier::Null});
                                                                } else if buffer[remember_me] == 49 {
                                                                    inputs.push(Input::MMB_Release{x, y, modifier: Input_Modifier::Null});
                                                                } else if buffer[remember_me] == 50 {
                                                                    inputs.push(Input::RMB_Release{x, y, modifier: Input_Modifier::Null});
                                                                } else {
                                                                    inputs.push(Input::Null);
                                                                }
                                                            }
                                                        } else if buffer[index] >= 54 && buffer[index] <= 56 && buffer[index - 1] == 49 {
                                                            let remember_me = index;
                                                            index += 2;
                                                            let (x, inc ) = parse_numbers(buffer, index);
                                                            index += 1 + inc;
                                                            let (y, inc) = parse_numbers(buffer, index); 
                                                            index += inc;
                                                            if buffer[index] == 77 {
                                                                //  It's being Pressed
                                                                if buffer[remember_me] == 54 {
                                                                    inputs.push(Input::LMB_Press{x, y, modifier: Input_Modifier::Ctrl});
                                                                } else if buffer[remember_me] == 55 {
                                                                    inputs.push(Input::MMB_Press{x, y, modifier: Input_Modifier::Ctrl});
                                                                } else if buffer[remember_me] == 56 {
                                                                    inputs.push(Input::RMB_Press{x, y, modifier: Input_Modifier::Ctrl});
                                                                } else {
                                                                    inputs.push(Input::Null);
                                                                }
                                                            } else if buffer[index] == 109 {
                                                                // It's being Released
                                                                if buffer[remember_me] == 54 {
                                                                    inputs.push(Input::LMB_Release{x, y, modifier: Input_Modifier::Ctrl});
                                                                } else if buffer[remember_me] == 55 {
                                                                    inputs.push(Input::MMB_Release{x, y, modifier: Input_Modifier::Ctrl});
                                                                } else if buffer[remember_me] == 56 {
                                                                    inputs.push(Input::RMB_Release{x, y, modifier: Input_Modifier::Ctrl});
                                                                } else {
                                                                    inputs.push(Input::Null);
                                                                }
                                                            }

                                                        } else if buffer[index] == 48 {
                                                            index += 2;
                                                            let (x, inc ) = parse_numbers(buffer, index);
                                                            index += 1 + inc;
                                                            let (y, inc) = parse_numbers(buffer, index); 
                                                            index += inc;
                                                            if buffer[index] == 77 {
                                                                inputs.push(Input::RMB_Press{x, y, modifier: Input_Modifier::Alt});
                                                            } else if buffer[index] == 109 {
                                                                inputs.push(Input::RMB_Release{x, y, modifier: Input_Modifier::Alt});
                                                            }
                                                        } else {
                                                            inputs.push(Input::Null);
                                                        }
                                                    }
                                                    // Vanilla Drag
                                                    51 => {
                                                        index += 1;

                                                        if buffer[index] > 49 && buffer[index] < 53 {
                                                            let remember_me = index;
                                                            index += 2;
                                                            let (x, inc ) = parse_numbers(buffer, index);
                                                            index += 1 + inc;
                                                            let (y, inc) = parse_numbers(buffer, index); 
                                                            index += inc;
                                                            if buffer[index] == 77 {
                                                                //  It's being Pressed
                                                                if buffer[remember_me] == 50 {
                                                                    inputs.push(Input::LMB_Move{x, y, modifier: Input_Modifier::Null});
                                                                } else if buffer[remember_me] == 51 {
                                                                    inputs.push(Input::MMB_Move{x, y, modifier: Input_Modifier::Null});
                                                                } else if buffer[remember_me] == 52 {
                                                                    inputs.push(Input::RMB_Move{x, y, modifier: Input_Modifier::Null});
                                                                } else {
                                                                    inputs.push(Input::Null);
                                                                }
                                                            }
                                                        }
                                                    },
                                                    // (Vanilla or Shift) + Scroll
                                                    54 => {
                                                        index += 1;

                                                        if buffer[index] == 52 || buffer[index] == 53 {
                                                            let remember_me = index;
                                                            index += 2;
                                                            let (x, inc ) = parse_numbers(buffer, index);
                                                            index += 1 + inc;
                                                            let (y, inc) = parse_numbers(buffer, index); 
                                                            index += inc;
                                                            if buffer[index] == 77 {
                                                                //  It's being Pressed
                                                                if buffer[remember_me] == 52 {
                                                                    inputs.push(Input::Scroll_Up{x, y, modifier: Input_Modifier::Null});
                                                                } else if buffer[remember_me] == 53 {
                                                                    inputs.push(Input::Scroll_Down{x, y, modifier: Input_Modifier::Null});
                                                                } else {
                                                                    inputs.push(Input::Null);
                                                                }
                                                            }

                                                        } else if buffer[index] == 56 || buffer[index] == 57 {
                                                            let remember_me = index;
                                                            index += 2;
                                                            let (x, inc ) = parse_numbers(buffer, index);
                                                            index += 1 + inc;
                                                            let (y, inc) = parse_numbers(buffer, index); 
                                                            index += inc;
                                                            if buffer[index] == 77 {
                                                                //  It's being Pressed
                                                                if buffer[remember_me] == 56 {
                                                                    inputs.push(Input::Scroll_Up{x, y, modifier: Input_Modifier::Shift});
                                                                } else if buffer[remember_me] == 57 {
                                                                    inputs.push(Input::Scroll_Down{x, y, modifier: Input_Modifier::Shift});
                                                                } else {
                                                                    inputs.push(Input::Null);
                                                                }
                                                            }

                                                        }
                                                    }
                                                    // Alt + Scroll
                                                    55 => {
                                                        index += 1;

                                                        if buffer[index] == 50 || buffer[index] == 51 {
                                                            let remember_me = index;
                                                            index += 2;
                                                            let (x, inc ) = parse_numbers(buffer, index);
                                                            index += 1 + inc;
                                                            let (y, inc) = parse_numbers(buffer, index); 
                                                            index += inc;
                                                            if buffer[index] == 77 {
                                                                //  It's being Pressed
                                                                if buffer[remember_me] == 50 {
                                                                    inputs.push(Input::Scroll_Up{x, y, modifier: Input_Modifier::Alt});
                                                                } else if buffer[remember_me] == 51 {
                                                                    inputs.push(Input::Scroll_Down{x, y, modifier: Input_Modifier::Alt});
                                                                } else {
                                                                    inputs.push(Input::Null);
                                                                }
                                                            }

                                                        }
                                                    }
                                                    // Ctrl + Scroll
                                                    56 => {
                                                        index += 1;

                                                        if buffer[index] == 48 || buffer[index] == 49 {
                                                            let remember_me = index;
                                                            index += 2;
                                                            let (x, inc ) = parse_numbers(buffer, index);
                                                            index += 1 + inc;
                                                            let (y, inc) = parse_numbers(buffer, index); 
                                                            index += inc;
                                                            if buffer[index] == 77 {
                                                                //  It's being Pressed
                                                                if buffer[remember_me] == 48 {
                                                                    inputs.push(Input::Scroll_Up{x, y, modifier: Input_Modifier::Ctrl});
                                                                } else if buffer[remember_me] == 49 {
                                                                    inputs.push(Input::Scroll_Down{x, y, modifier: Input_Modifier::Ctrl});
                                                                } else {
                                                                    inputs.push(Input::Null);
                                                                }
                                                            }

                                                        } else if buffer[index] == 59 {
                                                            index += 1;
                                                            let (x, inc ) = parse_numbers(buffer, index);
                                                            index += 1 + inc;
                                                            let (y, inc) = parse_numbers(buffer, index); 
                                                            index += inc;
                                                            if buffer[index] == 77 {
                                                                inputs.push(Input::LMB_Press{x, y, modifier: Input_Modifier::Alt});
                                                            } else if buffer[index] == 109 {
                                                                inputs.push(Input::LMB_Release{x, y, modifier: Input_Modifier::Alt});
                                                            } else {
                                                                inputs.push(Input::Null);
                                                            }

                                                        }
                                                        
                                                    }
                                                    57 => {
                                                        index += 1;
                                                        if buffer[index] == 59 {
                                                            index += 1;
                                                            let (x, inc ) = parse_numbers(buffer, index);
                                                            index += 1 + inc;
                                                            let (y, inc) = parse_numbers(buffer, index); 
                                                            index += inc;
                                                            if buffer[index] == 77 {
                                                                inputs.push(Input::MMB_Press{x, y, modifier: Input_Modifier::Alt});
                                                            } else if buffer[index] == 109 {
                                                                inputs.push(Input::MMB_Release{x, y, modifier: Input_Modifier::Alt});
                                                            } else {
                                                                inputs.push(Input::Null);
                                                            }
                                                        }
                                                    }
                                                    _ => {inputs.push(Input::Null);}
                                                }
                                            } else {
                                                inputs.push(Input::Null);
                                                break 'new_input;
                                            }
                                        },

                                        // Arrow Inputs
                                        65..=68 => {
                                            if buffer[index] == 65 {
                                                inputs.push(Input::Up);
                                            } else if buffer[index] == 66 {
                                                inputs.push(Input::Down);
                                            } else if buffer[index] == 67 {
                                                inputs.push(Input::Right);
                                            } else if buffer[index] == 68 {
                                                inputs.push(Input::Left);
                                            }
                                        },
                                        _ => {
                                            inputs.push(Input::Null);
                                        }
                                    }

                                } else {
                                    inputs.push(Input::Char('['));
                                    break 'new_input;
                                }
                            },
                            // It's Alt + Char
                            33..=90 | 92..=126 => {
                                inputs.push(Input::Alt_Char(parse_char(buffer[index] - 33)));
                            },
                            127 => {
                                inputs.push(Input::Alt_Backspace);
                            },
                            _ => {
                                inputs.push(Input::Null);
                            }
                        }

                    } else {
                        inputs.push(Input::Escape);
                        break 'new_input;
                    };

                },
                // It's just a character!
                33..=126 => {
                    inputs.push(Input::Char(parse_char(buffer[index] - 33)));
                },
                127 => {
                    inputs.push(Input::Backspace);
                }
                _ => {
                    inputs.push(Input::Null);
                }
            }
            index += 1;
        } else { break; }
    }

    return inputs
}
/// Helper function for parsing chars that appear in the buffer.
fn parse_char (character: u8) -> char {
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