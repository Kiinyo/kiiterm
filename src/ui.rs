pub struct Position {
    x: isize,
    y: isize
}
pub struct Size {
    weight: usize,
    width: usize
}
#[allow(non_camel_case_types)]
pub struct Animation <Animation_Style> {
    frame: usize,
    duration: usize,
    style: Animation_Style
}
mod element {
    pub struct Element {}
}
mod container {
    #[allow(non_camel_case_types)]
    pub enum Container_Style {
        Default,
        Text_Box
    }
    #[allow(non_camel_case_types)]
    pub enum Container_Reposition_Animation {
        None,

        Enter_Stage_Right
    }
    #[allow(non_camel_case_types)]
    pub enum Container_Resize_Animation {
        None,

        Lerp
    }
    #[allow(non_snake_case)]
    pub struct Container {
        name: String,
        desc: String,
        style: Container_Style,
        // Position
        Position: crate::ui::Position,
        Position_Target: crate::ui::Position,
        Position_Animation: crate::ui::Animation<Container_Reposition_Animation>,
        // Size
        Size: crate::ui::Size,
        Size_Target: crate::ui::Size,
        Size_Animation: crate::ui::Animation<Container_Resize_Animation>,
        // Elements
        Elements: Vec<crate::ui::element::Element>,


    }
}