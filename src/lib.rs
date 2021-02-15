/// A list of a commands relating to the ANSI terminal window.
pub mod terminal;
/// A list of commands relating to manupulating the screen.
pub mod screen;
/// Handling drawing to the screen.
pub mod graphics;

#[allow(non_camel_case_types)]
#[derive(Debug)]
/// A list of all the Modifiers that can be applied to Inputs.
pub enum Input_Modifier {
    Alt,
    Shift,
    Ctrl,

    Null
}
#[allow(non_camel_case_types)]
#[derive(Debug)]
/// A list of all the possible Inputs the terminal can recognize.
pub enum Input {
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

        // Space

        Spacebar,

        Ctrl_Spacebar,

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

        Shift_Page_Up,
        Alt_Page_Up,
        Ctrl_Page_Up,
        Ctrl_Alt_Page_Up,

        Shift_Page_Down,
        Alt_Page_Down,
        Ctrl_Page_Down,
        Ctrl_Alt_Page_Down,

        // End

        End,

        // Enter

        Enter,

        Alt_Enter,

        // Left Mouse Button
        LMB_Press {x: u16, y: u16, modifier: Input_Modifier},
        LMB_Release {x: u16, y: u16, modifier: Input_Modifier},
        LMB_Move {x: u16, y: u16, modifier: Input_Modifier},

        // Middle Mouse Button
        MMB_Press {x: u16, y: u16, modifier: Input_Modifier},
        MMB_Release {x: u16, y: u16, modifier: Input_Modifier},
        MMB_Move {x: u16, y: u16, modifier: Input_Modifier},

        // Right Mouse Button
        RMB_Press {x: u16, y: u16, modifier: Input_Modifier},
        RMB_Release {x: u16, y: u16, modifier: Input_Modifier},
        RMB_Move {x: u16, y: u16, modifier: Input_Modifier},

        // Scrolling
        Scroll_Up {x: u16, y: u16, modifier: Input_Modifier},
        Scroll_Down {x: u16, y: u16, modifier: Input_Modifier},

        // Null, junk/invalid/no inputs

        Null
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
