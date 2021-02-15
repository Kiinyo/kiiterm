fn main() {
    // Check grid
    let grid = kiiterm::grid::create_grid(5, 3, 0);
    println!("Grid: {:?}", grid);
    // Check empty rect
    let empty_rect = kiiterm::grid::create_rectangle(3, 3, 5, 0);
    println!("Empty Rectangle: {:?}", empty_rect);
    // Check filled rect
    let filled_rect = kiiterm::grid::create_rectangle(3, 5, 3, 3);
    println!("Filled Rectangle: {:?}", filled_rect);
    // Chece edge case tiny rect
    let tiny_rect = kiiterm::grid::create_rectangle(1, 1, 3, 3);
    println!("Tiny Rect: {:?}", tiny_rect);
}