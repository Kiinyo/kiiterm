pub struct Element {
    // Identifiers
    name: String,
    desc: String,
    elem_id: usize,
    // Things the element can do
    elem_type: Element_Type,
    // Physical properties
    position: crate::ui::Position,
    size: crate::ui::Size,
    appearance: crate::ui::Appearance,
    // Template for later use/reference
    template: Element_Type
}

pub enum Element_Type {
    Default,
    Interactive {
        press: Element_Action,
        release: Element_Action,

        click: Element_Action,
        abandon: Element_Action,

        drag: Element_Action,

        scroll: Element_Action,
    }
}
pub enum Element_Action {
    None,

    Delete_Element,
    Delete_Container
}

pub struct Element_Template {
    name: String,
    desc: String,
    elem_type: Element_Type,
    // Percentage of Container (<=1)
    align_x: crate::ui::Align_X,
    offset_x: f32,
    align_y: crate::ui::Align_Y,
    offset_y: f32,
    // Percentage of Container (<=1)
    scale_width: f32,
    scale_height: f32,
    // Shape
    shape: crate::grid::Shape,
    // Text handling
    text: String,
    text_align_x: crate::ui::Align_X,
    text_align_y: crate::ui::Align_Y,
    padding_x: usize,
    padding_y: usize
}