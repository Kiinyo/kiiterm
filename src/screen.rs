use crate::terminal;
/// The structure that holds everything needed to handle inputs and draw to thet terminal
pub struct Screen {
    width: u16,
    height: u16,
    context: termion::input::MouseTerminal<termion::raw::RawTerminal<std::io::Stdout>>,
    inputs: std::io::Stdin
}
/// Create the Screen to be used by nearly everything
pub fn init (width: u16, height: u16) -> Screen {
    // Let's first clear the terminal
    terminal::clear();
    // Then we'll get the terminal's widith and height after the resize
    let (width, height) = terminal::resize(width, height);

    // Unfortunately I seem to have to need this to run into_raw_mode()
    use termion::raw::IntoRawMode;
    // Create the context we'll be drawing things to
    let context= termion::input::MouseTerminal::from(std::io::stdout().into_raw_mode().unwrap());
    // Get a way to funnel player inputs
    let inputs = std::io::stdin();

    // Finally return the screen to be used
    Screen {width, height, context, inputs}
}