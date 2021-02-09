/// Gets the current dimensions of the terminal in characters displayed.
pub fn dimensions() -> (u16, u16) {
    termion::terminal_size().unwrap()
}
/// Resizes the terminal by the amount of characters it can hold. Returns the new window's dimensions (width, height)
pub fn resize (width: u16, height: u16) -> (u16, u16) {    
    // The actual resizing of the window
    println!("\u{001B}[8;{};{}t",height, width);
    // Apparently it goes too fast and returns the wrong dimensions
    // without a quick pause here!
    std::thread::sleep(std::time::Duration::from_millis(50));

    dimensions()
}