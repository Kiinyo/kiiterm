use std::collections::HashMap;

use crate::graphics::{Glyph, parse_string_to_glyphs};
use crate::ui::*;
use crate::grid::Shape;

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
    pub elem_order: Vec<String>,
    pub elements: HashMap<String, element::Element>,
    pub buffer: Vec<Vec<Glyph>>,
}
pub enum ContainerType {
    Default
}
pub struct ContainerTemplate {
    name: String,
    desc: String,
    cont_id: usize,

    cont_type: ContainerType,

    width: usize,
    height:usize,

    text: String,

    align_x: AlignX,
    align_y: AlignY,

    padding_x: usize,
    padding_y: usize,

    x: usize, 
    y: usize,

    body: Glyph,

    elements: Vec<element::ElementTemplate>
}
/// Creates a container using a template
pub fn create (mut template: ContainerTemplate) -> Container {
    let mut container: Container = Container {
        name: template.name,
        desc: template.desc,
        cont_id: template.cont_id,
        cont_type: template.cont_type,
        position: Position {
            current_x: template.x,
            current_y: template.y,
            target_x: template.x,
            target_y: template.y,
            animation: Animation { 
                current_frame: 0, 
                target_frame: 0, 
                animation_type: Reposition::None
            },

        },
        size: Size {
            current_width: template.width,
            current_height: template.height,
            target_width: template.width,
            target_height: template.height,
            animation: Animation { 
                current_frame: 0, 
                target_frame: 0, 
                animation_type: Resize::None
            },
        },
        appearance: Appearance { 
            body: template.body.clone(), 
            text: Text { 
                string: parse_string_to_glyphs(template.text), 
                AlignX: template.align_x, 
                AlignY: template.align_y, 
                padding_x: template.padding_x, 
                padding_y: template.padding_y, 
                buffer: vec![vec![template.body.clone()]]
            }, 
            shape: Shape::Rectangle, 
            animation: Animation { 
                current_frame: 0, 
                target_frame: 0, 
                animation_type: Shader::None
            }, 
            buffer: vec![vec![template.body.clone()]]
        },
        elem_order: Vec::new(),
        elements: HashMap::new(),
        buffer: vec![vec![template.body.clone()]]   
    };

    for _e in 0..template.elements.len() {
        // Create the element
        let element = element::create(template.elements.pop().unwrap(), &mut container);
        add_element(&mut container, element);
    };

    return container;
}
/// Adds an element and returns its name/key in the hashmap.
pub fn add_element (container: &mut Container, element: element::Element) -> String {
    let name = element.name.clone();
    // Determine the element's draw order position
    container.elem_order.push(name.clone());
    // Add element to the hash map
    container.elements.insert(name.clone(), element);

    return name;
}
/// Removes an element and returns it.
pub fn remove_element (container: &mut Container, name: String) -> element::Element{
    // Remove it from the draw order
    for e in 0..container.elem_order.len() {
        if container.elem_order[e] == name {
            container.elem_order.remove(e);
            break;
        }
    }
    // Remove it from the hash map and return it
    container.elements.remove(&name).unwrap()
}
/// Scales the elements appropriately
fn format_elements (container: &mut Container) {
    for (_, element) in container.elements.iter_mut() {
        match element.elem_type {
            element::ElementType::Header => {
                element.appearance.text.string = crate::graphics::parse_string_to_glyphs(container.name.clone())
            },
            _ => {}
        };
        // Size formatting
        element.size.current_width = ( 
            element.reference.scale_width 
            * container.size.current_width as f32 
        ) as usize;
        element.size.current_height = ( 
            element.reference.scale_height 
            * container.size.current_height as f32 
        ) as usize;
        // Position formatting
        element.position.current_x = match element.reference.align_x {
            AlignX::Left => {
                0 + (element.reference.offset_x * container.size.current_width as f32) as usize
            }
            AlignX::Center => {
                ((container.size.current_width / 2 - element.size.current_width / 2) as isize
                + (element.reference.offset_x * container.size.current_width as f32) as isize) as usize
            }
            AlignX::Right => {
                container.size.current_width - element.size.current_width
                - (element.reference.offset_x * container.size.current_width as f32) as usize
            }
        };
        element.position.current_y = match element.reference.align_y {
            AlignY::Up => {
                0 + (element.reference.offset_y * container.size.current_height as f32) as usize
            }
            AlignY::Center => {
                ((container.size.current_height / 2 - element.size.current_height / 2) as isize
                + (element.reference.offset_y * container.size.current_height as f32) as isize) as usize
            }
            AlignY::Down => {
                container.size.current_height - element.size.current_height
                - (element.reference.offset_y * container.size.current_height as f32) as usize
            }
        };
    }
}

pub fn update (container: &mut Container) {
    // Apply any animations

    // 

    // Make any changes to the elements
    format_elements(container);
    // Now update them
    update_elements(container);
}

fn update_elements (container: &mut Container) {
    for (_, element) in container.elements.iter_mut() {
        element::update(element);
    }
}