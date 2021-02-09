extern crate kiiterm;
fn main() {
    println!("New window size: {:?}",kiiterm::terminal::resize(1000, 205));
}