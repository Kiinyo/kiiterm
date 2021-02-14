extern crate kiiterm;
use kiiterm::screen::*;
use kiiterm::Input;
use kiiterm::graphics;
fn main() {
    let mut screen = kiiterm::screen::init(100, 5);

    loop {

        let debug = debug_inputs(&mut screen);

        let color1 = graphics::parse_color(
            graphics::Color::Blue, 
            graphics::Depth::Fg
        );

        let style1 = graphics::parse_style(graphics::Style::Bold);

        let color2 = graphics::parse_color(
            graphics::Color::Red,
            graphics::Depth::Bg
        );

        let style2 = graphics::parse_style(graphics::Style::Blink);


        let buffer = format!("{}{}{}{}Buffer: {:?}{}{}{}{}Interpreted Inputs: {:?}{}",
            "\u{001B}[2J",
            "\u{001B}[1;1H",
            color1,
            style1,
            debug.1,
            graphics::RESET,
            "\u{001B}[2;1H",
            color2,
            style2,
            debug.0,
            graphics::RESET
        );

        let buffer2 = format!("{}{}{}er",
        "\u{001B}[2;4H",
        color1,
        style1
        );
        

        draw_to_buffer(&mut screen, buffer);
        draw_to_buffer(&mut screen, buffer2);

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