pub struct Container {
    // Identifiers
    name: String,
    desc: String,
    cont_id: usize,
    // Things the element can do
    cont_type: Container_Type,
    // Physical properties
    position: crate::ui::Position,
    size: crate::ui::Size,
    appearance: crate::ui::Appearance,
    // Keeping track of elements
    elem_id: usize,
    elements: Vec<crate::ui::element::Element>,
    element_buffer: crate::grid::Grid
}
pub enum Container_Type {
    Default
}