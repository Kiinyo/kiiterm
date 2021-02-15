fn main() {
    // Check grid
    let grid = kiiterm::grid::create_grid(5, 3, 0);
    println!("Grid: {:?}", grid);
    // Check empty rect
    let empty_rect = kiiterm::grid::create_rectangle(3, 3, 5, 0);
    println!("Empty Rectangle: {:?}", empty_rect);
    let filled_circ = kiiterm::grid::create_circle(5, 1, 2, 0);
    println!("Filled Circle: {:?}", filled_circ);
    let mut line_grid = kiiterm::grid::create_grid(10, 10, 0);
    line_grid = kiiterm::grid::draw_line(line_grid, 0, 0, 9, 8, 1);
    println!("Grid with line: {:?}", line_grid);
    let vertices = vec![2, 0, 7, 0, 9, 6, 0, 6];
    let simple_poly = kiiterm::grid::create_polygon(10, 7, vertices, 2, 1, 0);
    println!("Simple Poly: {:?}", simple_poly);
    let simple_poly = kiiterm::grid::flood_fill(simple_poly, 5, 3, 3, false);
    println!("Simple Poly filled: {:?}", simple_poly);
}