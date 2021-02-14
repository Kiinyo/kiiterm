extern crate kiiterm;
use std::io::*;

use kiiterm::screen::*;
use kiiterm::Input;
fn main() {
    let mut screen = kiiterm::screen::init(100, 5);

    loop {

        let debug = debug_inputs(&mut screen);
        let glyph = Glyph {symbol: 'a'};

        writeln!(
            screen.context,
            "{}{}Buffer: {:?}{}Interpreted Inputs: {:?}",
            termion::clear::All,
            termion::cursor::Goto(1, 1),
            debug.1,
            termion::cursor::Goto(1, 2),
            debug.0
        ).unwrap();

        draw(&mut screen, glyph, 1, 3);

        if debug.0.len() > 0 {
            match debug.0[0] {
                Input::Ctrl_Char(c) => {
                    if c == 'C' {break;}
                },
                _ => {}
            }
       }


        screen.context.flush().unwrap();
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}