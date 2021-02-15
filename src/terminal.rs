/// Returns the terminal's current dimensions: (width,height)
pub fn dimensions() -> (u16, u16) {
    // Leveraging termion for this because I can't find a simple
    // way to do it without this.
    termion::terminal_size().unwrap()
}
/// Equivalent to running "clear" in the terminal, wipes buffers. Not to be confused with screen::clear!
pub fn clear () {
    // ANSI escape code to run the clear command
    println!("\u{001B}c");
}
/// Resizes the terminal by the amount of characters it can hold. Returns terminal dimensions: (width, height)
pub fn resize (width: u16, height: u16) -> (u16, u16) {
    // The actual resizing of the window
    println!("\u{001B}[8;{};{}t",height, width);
    // Apparently it goes too fast and returns the wrong dimensions
    // without a quick pause here!
    std::thread::sleep(std::time::Duration::from_millis(50));
    // Return the terminal dimensions
    dimensions()
}