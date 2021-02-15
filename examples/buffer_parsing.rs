extern crate kiiterm;
use graphics::Glyph;
use kiiterm::screen;
use kiiterm::Input;
use kiiterm::graphics;

fn main() {
    // First thing's first, let's create a screen
    let mut screen = screen::init(100, 5);

    // Now let's create two glyphs that'll hold the debug info
    let mut glyph1 = Glyph {
        // Let's create an empty string to start with
        symbol: String::new(),
        // And assign the foreground and background colors
        fg_color: graphics::Color::Default,
        bg_color: graphics::Color::Default,
        // Finally we add any styles we want
        styles: vec![graphics::Style::Bold, graphics::Style::Bright]
    };
    let mut glyph2 = Glyph {
        symbol: String::new(),
        fg_color: graphics::Color::Green,
        bg_color: graphics::Color::Default,
        styles: vec![graphics::Style::Bold, graphics::Style::Underline]
    };

    // Now let's create a simple loop
    loop {
        // Let's create a tuple to hold the two results of the
        // inputs_debug function!
        let debug = screen::inputs_debug(&mut screen);
        // Now that we've done that we can safely clear the screen
        // without having to worry about clearing the buffer holding
        // any inputs!
        screen::clear(&mut screen);
        // Then we manually assign the debug tuple's outputs to
        // a corresponding glyph's symbol to be displayed in a sec!
        glyph1.symbol = format!("Buffer: {:?}", debug.1);
        glyph2.symbol = format!("Inputs: {:?}", debug.0);
        // Once that's done we can display the glyphs!
        graphics::draw_glyph(&mut screen, &glyph1, 0, 0);
        graphics::draw_glyph(&mut screen, &glyph2, 0, 1);

        // Now we've draw to the buffer but we haven't buffer_rendered
        // it yet! This function lets us do that!
        screen::buffer_render(&mut screen);


        // Now we neeed a way to break out of the loop
        // So we'll create a simple input parser that
        // checks of Ctrl + C is the first input pressed.
        // If it is, we'll end the loop!
        match debug.0[0] {
            // First let's see if it's Ctrl + Char.
            Input::Ctrl_Char(c) => {
                // If it is, then check if the input is 'C'.
                // Remember terminals don't make a distinction
                // between Ctrl + C and Ctrl + c so the char in
                // Ctrl_Char is always uppercase!
                if c == 'C' {
                    // Finally if it is 'C' we break the loop
                    break;
                }
            },
            // If it's anything else we do nothing
            _ => {}
        }
        
        // Finally let's put the thread to sleep for a second before
        // looping again so we have time to see the buffer and input!
        std::thread::sleep(std::time::Duration::from_millis(1000));
    }
}