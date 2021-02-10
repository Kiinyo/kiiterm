extern crate kiiterm;
use std::io::{Write};
fn main() {
    let mut screen = kiiterm::screen::init(100, 5);

    loop {
        let mut buffer = Vec::new();
        use std::io::*;
        screen.inputs.read_to_end(&mut buffer).unwrap();
        writeln!(screen.context,
        "{}{}Buffer: {:?}",
        termion::clear::All,
        termion::cursor::Goto(1, 1),
        buffer)
       .unwrap();

        screen.context.flush().unwrap();
        std::thread::sleep(std::time::Duration::from_millis(2000));
    }
}