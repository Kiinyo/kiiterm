use crate::graphics::Glyph;

/// Where the widget is in coordinate space.
pub struct Position {
    // Where the ui piece actually is
    current_x: usize,
    current_y: usize,
    // Where it wants to be
    target_x: usize,
    target_y: usize,
    // How it'll get there
    animation: Animation<Reposition>
}
/// The possible ways to animate a repositing widget.
pub enum Reposition {
    None
}
/// The amount of space the widget takes up.
pub struct Size {
    pub current_width: usize,
    pub current_height: usize,

    pub target_width: usize,
    pub target_height: usize,

    pub animation: Animation<Resize>
}
/// The possible ways to animate resizing a widget.
pub enum Resize {
    None
}
pub struct Appearance {
    body: crate::graphics::Glyph,
    text: Text,
    // The shape of the widget itself
    shape: crate::grid::Shape,
    // Animation of the appearance
    animation: crate::ui::Animation<Shader>,
    // The actual grid of (pixels), saves time
    // when calling up draw functions.
    buffer: Vec<Vec<Glyph>>
}
pub struct Text {
    string: Vec<crate::graphics::Glyph>,
    // Alignment of the text
    AlignX: AlignX,
    AlignY: AlignY,
    // How far from the edge of the widget text can be displayed
    padding_x: usize,
    padding_y: usize,
    // The actual grid of (pixels), saves time
    // when calling up draw functions.
    buffer: Vec<Vec<Glyph>>
}
pub enum AlignX {
    Left,
    Center,
    Right
}
pub enum AlignY {
    Up,
    Center,
    Down
}
pub enum Shader {
    None
}
/// Animation container
pub struct Animation<AnimationType> {
    // Details about what frame the animation is in
    current_frame: usize,
    target_frame: usize,
    // Info for actually parsing the animation.
    animation_type: AnimationType,
}
pub mod element;
pub mod container;