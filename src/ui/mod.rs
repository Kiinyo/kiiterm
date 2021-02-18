/// Where the widget is in coordinate space.
pub struct Position {
    // Where the ui piece actually is
    current_x: usize,
    current_y: usize,
    // Where it wants to be
    target_x: usize,
    target_y: usize,
    // How it'll get there
    Animation: Animation<Reposition>
}
/// The possible ways to animate a repositing widget.
pub enum Reposition {
    None
}
/// The amount of space the widget takes up.
pub struct Size {
    current_width: usize,
    current_height: usize,

    target_width: usize,
    target_height: usize,

    Animation: Animation<Resize>
}
/// The possible ways to animate resizing a widget.
pub enum Resize {
    None
}
pub struct Appearance {
    // The color of the body of a text box
    primary_color: crate::graphics::Color,
    // The color of the bar along the top
    secondary_color: crate::graphics::Color,
    // The color of any text in the box
    tertiary_color: crate::graphics::Color,

    text: Text,
    
    // The shape of the widget itself
    shape: crate::grid::Shape,
    // Animation of the appearance
    animation: crate::ui::Animation<Shader>,
    // The actual grid of (pixels), saves time
    // when calling up draw functions.
    buffer: crate::grid::Grid
}
pub struct Text {
    string: Vec<crate::graphics::Glyph>,
    // Alignment of the text
    align_x: Align_X,
    align_y: Align_Y,
    // How far from the edge of the widget text can be displayed
    padding_x: usize,
    padding_y: usize,
    // The actual grid of (pixels), saves time
    // when calling up draw functions.
    buffer: crate::grid::Grid
}
pub enum Align_X {
    Left,
    Center,
    Right
}
pub enum Align_Y {
    Up,
    Center,
    Down
}
pub enum Shader {
    None
}
/// Animation container
pub struct Animation<Animation_Type> {
    // Details about what frame the animation is in
    current_frame: usize,
    target_frame: usize,
    // Info for actually parsing the animation.
    animation_type: Animation_Type,
}
pub mod element;
pub mod container;