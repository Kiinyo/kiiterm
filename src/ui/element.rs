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
    appearance: crate::ui::Appearance
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