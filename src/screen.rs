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
    pub enum Key {
        // a..Z
        Char(char),
        Alt_Char(char),
        // A..Z
        // The terminal doe not recognize cae with Ctrl
        Ctrl_Char(char),

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

        // Home

        Home,

        // Page
        
        Page_Up,
        Page_Down,

        // End

        End,

        // Null, junk/invalid/no inputs

        Null
    }
    pub enum Modifier {
        Alt,
        Shift,
        Ctrl,

        None
    }
    /// Follows Enum(x, y, Modifier)
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

    
}