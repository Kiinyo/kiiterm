use kiiterm::{grid::*, terminal::clear};
fn simple_parse (string: &str) -> Shape {
    match string {
        "Right_Iso_Tri" => {
            return Shape::RightIsoTri;
        }
        "Left_Iso_Tri" => {
            return Shape::LeftIsoTri;
        }
        "Up_Iso_Tri" => {
            return Shape::UpIsoTri;
        }
        "Down_Iso_Tri" => {
            return Shape::DownIsoTri;
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
        let mut grid = create_shape(shape, width, height, 1, 5);
        println!("You made: {:?}", grid);
        println!("Would you like to draw a line on it? (y/n)");
        let mut choice = String::new();
        std::io::stdin()
            .read_line(&mut choice)
            .unwrap();
        match choice.as_str() {
            "n" => {break},
            _ => {
                println!("Choose a start x:");
                let mut x1 = String::new();
                std::io::stdin()
                    .read_line(&mut x1)
                    .unwrap();
                let x1: usize = x1.trim().parse().unwrap();

                println!("Choose a start y:");
                let mut y1 = String::new();
                std::io::stdin()
                    .read_line(&mut y1)
                    .unwrap();
                let y1: usize = y1.trim().parse().unwrap();

                println!("Choose an end x:");
                let mut x2 = String::new();
                std::io::stdin()
                    .read_line(&mut x2)
                    .unwrap();
                let x2: usize = x2.trim().parse().unwrap();

                println!("Choose an end y:");
                let mut y2 = String::new();
                std::io::stdin()
                    .read_line(&mut y2)
                    .unwrap();
                let y2: usize = y2.trim().parse().unwrap();

                println!("Choose an fill number:");
                let mut fill = String::new();
                std::io::stdin()
                    .read_line(&mut fill)
                    .unwrap();
                let fill: usize = fill.trim().parse().unwrap();

                overlay_line(&mut grid, x1,y1,x2,y2,fill,&Overlay::Simple);
                
                println!("You made: {:?}", grid);

                println!("Enter anything to continue");
                let mut choice = String::new();
                std::io::stdin()
                    .read_line(&mut choice)
                    .unwrap();
            }
        }
        clear();
    }
}