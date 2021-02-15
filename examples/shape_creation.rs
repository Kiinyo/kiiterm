fn main() {
    // Check grid
    let grid = kiiterm::grid::create_grid(5, 3, 0);
    println!("Grid: {:?}", grid);
    // Check empty rect
    let empty_rect = kiiterm::grid::create_rectangle(3, 3, 5, 0);
    println!("Empty Rectangle: {:?}", empty_rect);
    let filled_circ = kiiterm::grid::create_circle(5, 1, 2, 0);
    println!("Filled Circle: {:?}", filled_circ);
}