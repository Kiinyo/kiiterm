use crate::ui::*;
pub struct Element {
    // Identifiers
    name: String,
    desc: String,
    elem_id: usize,
    // Things the element can do
    elem_type: Element_Type,
    // Physical properties
    position: Position,
    size: Size,
    appearance: Appearance,
    // Template for later use/reference
    template: Element_Template
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
    align_x: Align_X,
    offset_x: f32,
    align_y: Align_Y,
    offset_y: f32,
    // Percentage of Container (<=1)
    scale_width: f32,
    scale_height: f32,
    // Shape
    body: crate::graphics::Glyph,
    shape: crate::grid::Shape,
    // Text handling
    text: String,
    text_align_x: Align_X,
    text_align_y: Align_Y,
    padding_x: usize,
    padding_y: usize
}

pub fn create (container: container::Container, template: Element_Template) -> Element {

    let size = Size {
        // Current
        current_width: (container.size.current_width as f32 * template.scale_width) as usize,
        current_height: (container.size.current_height as f32 * template.scale_height) as usize,
        // Target
        target_width: (container.size.target_width as f32 * template.scale_width) as usize,
        target_height: (container.size.target_height as f32 * template.scale_height) as usize,
        animation: Animation {
            current_frame: 0,
            target_frame: 0,
            animation_type: Resize::None,
            
        }
    };
    
    let position = Position {
        current_x: match template.align_x {
            Align_X::Left => {
                0
            }
            Align_X::Center => {
                container.size.current_width / 2 - size.current_width / 2
            }
            Align_X::Right => {
                container.size.current_width - size.current_width
            }
        },
        current_y: match template.align_y {
            Align_Y::Up => {
                0
            }
            Align_Y::Center => {
                container.size.current_height / 2 - size.current_height / 2
            }
            Align_Y::Down => {
                container.size.current_height - size.current_height
            }
        },
        target_x: match template.align_x {
            Align_X::Left => {
                0
            }
            Align_X::Center => {
                container.size.target_width / 2 - size.target_width / 2
            }
            Align_X::Right => {
                container.size.target_width - size.target_width
            }
        },
        target_y: match template.align_y {
            Align_Y::Up => {
                0
            }
            Align_Y::Center => {
                container.size.target_height / 2 - size.target_height / 2
            }
            Align_Y::Down => {
                container.size.target_height - size.target_height
            }
        },
        animation: Animation {
            current_frame: 0,
            target_frame: 0,
            animation_type: Reposition::None,
            
        }
    };

    let appearance = Appearance {
        body: template.body,
        text: Text {
            string: crate::graphics::parse_string_to_glyphs(template.text),
            align_x: template.text_align_x,
            align_y: template.text_align_y,
            padding_x: template.padding_x,
            padding_y: template.padding_y,
            buffer: crate::grid::create_grid(1,1, 0)
        },
        shape: template.shape,
        animation: Animation {
            current_frame: 0,
            target_frame: 0,
            animation_type: Shader::None,
            
        },
        buffer: crate::grid::create_grid(1,1, 0)

    };

    let element: Element = Element {
        name: template.name,
        desc: template.desc,
        elem_id: container.elem_id,
        elem_type: template.elem_type,
        position,
        size,
        appearance,
        template
    };

    container.elem_id += 1;

    return element;
}