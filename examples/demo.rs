extern crate kiiterm;

use kiiterm::screen::*;
use kiiterm::Input;
fn main() {
    let mut screen = kiiterm::screen::init(100, 5);

    loop {

        let debug = debug_inputs(&mut screen);

        let buffer = format!("{}{}Buffer: {:?}{}Interpreted Inputs: {:?}",
            "\u{001B}[2J",
            "\u{001B}[1;1H",
            debug.1,
            "\u{001B}[2;1H",
            debug.0
        );
        

        draw_to_buffer(&mut screen, buffer);

        if debug.0.len() > 0 {
            match debug.0[0] {
                Input::Ctrl_Char(c) => {
                    if c == 'C' {break;}
                },
                _ => {}
            }
       }

       display_buffer(&mut screen);

       std::thread::sleep(std::time::Duration::from_millis(100));
    }
}