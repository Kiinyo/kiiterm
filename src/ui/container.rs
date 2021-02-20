pub struct Container {
    // Identifiers
    pub name: String,
    pub desc: String,
    pub cont_id: usize,
    // Things the element can do
    pub cont_type: ContainerType,
    // Physical properties
    pub position: crate::ui::Position,
    pub size: crate::ui::Size,
    pub appearance: crate::ui::Appearance,
    // Keeping track of elements
    pub elem_id: usize,
    pub elements: Vec<crate::ui::element::Element>,
    pub element_buffer: crate::grid::Grid
}
pub enum ContainerType {
    Default
}

