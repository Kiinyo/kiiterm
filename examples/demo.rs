extern crate kiiterm;
use std::io::*;

use kiiterm::screen::input::*;
fn main() {
    let mut screen = kiiterm::screen::init(100, 5);

    loop {

        let mut buffer: Vec<u8> = Vec::new();

        screen.inputs.read_to_end(&mut buffer).unwrap();

        let inputs: Vec<Key> = parse_input(&buffer);

        writeln!(screen.context,
        "{}{}Buffer: {:?} {}Interpreted Inputs: {:?}",
        termion::clear::All,
        termion::cursor::Goto(1, 1),
        buffer,
        termion::cursor::Goto(1, 2),
        inputs)
       .unwrap();

        if inputs.len() > 0 {
            match inputs[0] {
                Key::Ctrl_Char(c) => {
                    if c == 'C' {break;}
                },
                _ => {}
            }
       }


        screen.context.flush().unwrap();
        std::thread::sleep(std::time::Duration::from_millis(2000));
    }
}