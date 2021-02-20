use crate::ui::*;
pub struct Element {
    // Identifiers
    name: String,
    desc: String,
    elem_id: usize,
    // Things the element can do
    elem_type: ElementType,
    // Physical properties
    position: Position,
    size: Size,
    appearance: Appearance,
    // Template for later use/reference
    reference: ElementReference
}
pub enum ElementType {
    Default,
    Interactive {
        press: ElementAction,
        release: ElementAction,

        click: ElementAction,
        abandon: ElementAction,

        drag: ElementAction,

        scroll: ElementAction,
    }
}
pub enum ElementAction {
    None,

    DeleteElement,
    DeleteContainer
}

pub struct ElementTemplate {
    name: String,
    desc: String,
    elem_type: ElementType,
    // Percentage of Container (<=1)
    align_x: AlignX,
    offset_x: f32,
    align_y: AlignY,
    offset_y: f32,
    // Percentage of Container (<=1)
    scale_width: f32,
    scale_height: f32,
    // Shape
    body: crate::graphics::Glyph,
    shape: crate::grid::Shape,
    // Text handling
    text: String,
    text_align_x: AlignX,
    text_align_y: AlignY,
    padding_x: usize,
    padding_y: usize
}

pub fn create (container: &mut container::Container, template: ElementTemplate) -> Element {

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
            AlignX::Left => {
                0 + (template.offset_x * container.size.current_width as f32) as usize
            }
            AlignX::Center => {
                ((container.size.current_width / 2 - size.current_width / 2) as isize
                + (template.offset_x * container.size.current_width as f32) as isize) as usize
            }
            AlignX::Right => {
                container.size.current_width - size.current_width
                - (template.offset_x * container.size.current_width as f32) as usize
            }
        },
        current_y: match template.align_y {
            AlignY::Up => {
                0 
            }
            AlignY::Center => {
                ((container.size.current_height / 2 - size.current_height / 2) as isize
                + (template.offset_y * container.size.current_height as f32) as isize) as usize
            }
            AlignY::Down => {
                container.size.current_height - size.current_height
            }
        },
        target_x: match template.align_x {
            AlignX::Left => {
                0
                + (template.offset_x * container.size.target_width as f32) as usize
            }
            AlignX::Center => {
                ((container.size.target_width / 2 - size.target_width / 2) as isize
                + (template.offset_x * container.size.target_width as f32) as isize) as usize
            }
            AlignX::Right => {
                container.size.target_width - size.target_width
                - (template.offset_x * container.size.target_width as f32) as usize
            }
        },
        target_y: match template.align_y {
            AlignY::Up => {
                0
                + (template.offset_y * container.size.target_height as f32) as usize
            }
            AlignY::Center => {
                ((container.size.target_height / 2 - size.target_height / 2) as isize
                + (template.offset_y * container.size.target_height as f32) as isize) as usize
            }
            AlignY::Down => {
                container.size.target_height - size.target_height
                - (template.offset_y * container.size.target_height as f32) as usize
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
            AlignX: template.text_align_x,
            AlignY: template.text_align_y,
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
        reference: ElementReference{
            align_x: template.align_x,
            align_y: template.align_y,
            offset_x: template.offset_x,
            offset_y: template.offset_y,
            scale_width: template.scale_width,
            scale_height: template.scale_height
        }
    };

    container.elem_id += 1;

    return element;
}

pub struct ElementReference {
    align_x: AlignX,
    align_y: AlignY,
    offset_x: f32,
    offset_y: f32,
    scale_width: f32,
    scale_height: f32

}