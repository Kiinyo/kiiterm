use kiiterm::{grid::*, terminal::clear};
fn simple_parse (string: &str) -> Shape {
    match string {
        "Right_Iso_Tri" => {
            return Shape::Right_Iso_Tri;
        }
        "Left_Iso_Tri" => {
            return Shape::Left_Iso_Tri;
        }
        "Up_Iso_Tri" => {
            return Shape::Up_Iso_Tri;
        }
        "Down_Iso_Tri" => {
            return Shape::Down_Iso_Tri;
        }
        "Circle" => {
            return Shape::Circle;
        }
        "Rectangle" => {
            return Shape::Rectangle;
        }
        _ => {
            println!("That shape isn't supported!");
            return Shape::Rectangle;
        }
    }
}
fn main() {
    loop {
        println!("Enter a shape! Options are: [Direction]_Iso_Tri, Rectangle, Circle");
        let mut shape = String::new();
        std::io::stdin()
            .read_line(&mut shape)
            .unwrap();
        let shape = shape.trim();
        let shape = simple_parse(shape);
        println!("Enter a width:");
        let mut width = String::new();
        std::io::stdin()
            .read_line(&mut width)
            .unwrap();
        let width: usize = width.trim().parse().unwrap();
        println!("Enter a height:");
        let mut height = String::new();
        std::io::stdin()
            .read_line(&mut height)
            .unwrap();
        let height: usize = height.trim().parse().unwrap();
        let grid = create_shape(shape, width, height, 1, 5);
        println!("You made: {:?}", grid);
        println!("Make another? (y/n)");
        let mut choice = String::new();
        std::io::stdin()
            .read_line(&mut choice)
            .unwrap();
        match choice.as_str() {
            "n" => {break},
            _ => {}
        }
        clear();
    }
}