use kiiterm::grid::*;
fn main() {
    // Check grid
    let grid = create_grid(5, 3, 0);
    println!("Grid: {:?}", grid);
    // Check empty rect
    let empty_rect = create_rectangle(3, 3, 5, 0);
    println!("Empty Rectangle: {:?}", empty_rect);
    let filled_circ = create_circle(5, 1, 2);
    println!("Filled Circle: {:?}", filled_circ);
    let mut line_grid = create_grid(20, 20, 0);
    line_grid = overlay_line(line_grid, 0, 0, 18, 14, 1, &Overlay::Simple);
    println!("Grid with line: {:?}", line_grid);
    let vertices = vec![2, 0, 7, 0, 9, 6, 0, 6];
    let simple_poly = create_polygon(10, 7, vertices, 2, 1);
    println!("Simple Poly: {:?}", simple_poly);
    let simple_fill = create_grid(11, 7, 0);
    let simple_fill = overlay_flood_fill(simple_fill, 5, 3, 1, true);
    println!("Grid filled: {:?}", simple_fill);
    let right_iso_big = create_shape(Shape::Right_Iso_Tri, 13, 13, 1, 2);
    println!("Normal Right Iso: {:?}", right_iso_big);
    let right_iso_mini = create_shape(Shape::Right_Iso_Tri, 2, 2, 1, 2);
    println!("Normal Right Iso: {:?}", right_iso_mini);
    let right_iso_tiny = create_shape(Shape::Right_Iso_Tri, 1, 1, 1, 2);
    println!("Normal Right Iso: {:?}", right_iso_tiny);
}