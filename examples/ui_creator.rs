use kiiterm::graphics::Glyph;
use kiiterm::graphics::Color;
use kiiterm::graphics::Style;

fn main() {
    let glyph1 = Glyph {
        symbol: "A".to_string(),
        fg_color: Color::Black,
        bg_color: Color::Default,
        styles: vec![Style::Strike_Through]
    };

    let mut glyph2 = glyph1.clone();

    glyph2.symbol = "B".to_string();

    println!("Does glyph1 work? {}", glyph1.symbol);
    println!("Does glyph1 work? {}", glyph2.symbol);
}