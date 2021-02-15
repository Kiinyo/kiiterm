extern crate kiiterm;
use graphics::Glyph;
use kiiterm::{screen::*, terminal};
use kiiterm::Input;
use kiiterm::graphics;
fn main() {
    let mut screen = kiiterm::screen::init(100, 5);

    let mut glyph1 = Glyph {
        symbol: "Debug".to_string(),

        fg_color: graphics::Color::Blue,
        bg_color: graphics::Color::Black,

        styles: vec![graphics::Style::Blink, graphics::Style::Strike_Through]
    };

    let mut glyph2 = Glyph {
        symbol: "Debug".to_string(),

        fg_color: graphics::Color::White,
        bg_color: graphics::Color::Red,

        styles: vec![graphics::Style::Bold]
    };

    loop {

        let debug = debug_inputs(&mut screen);

        glyph1.symbol = format!("Buffer: {:?}", debug.1);
        glyph2.symbol = format!("Inputs: {:?}", debug.0);
        

        graphics::draw_glyph(&mut screen, &glyph1, 0, 0);
        graphics::draw_glyph(&mut screen, &glyph2, 0, 1);

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