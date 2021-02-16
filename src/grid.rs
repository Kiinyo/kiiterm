pub struct Grid {
    pub width: usize,
    pub height: usize,
    pub tiles: Vec<Vec<usize>>
}
/// Implimenting debugging because I know I'm going to need it.
impl std::fmt::Debug for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut y:usize = 0;
        let mut tiles_formatted: String = String::new();
        loop {
            if y >= self.height as usize {break;}
            tiles_formatted = format!("{}{:?}\n", tiles_formatted, self.tiles[y]);
            y += 1;
        }
        write!(f, "Width: {}, Height: {}, Tiles:\n{}", self.width, self.height, tiles_formatted)
    }

}

/// Create a grid with with width and height and fill it.
fn create_grid (width: usize, height: usize, fill: usize) -> Grid {
    // Loop to fill in everything
    let mut tiles: Vec<Vec<usize>> = Vec::new();
    for _y in 0..height {
        let mut row: Vec<usize> = Vec::new();
        for x in 0..width{
            if x >= width {break;}
            row.push(fill); 
        }
        tiles.push(row);
    }
    // And return it
    Grid {
        width,
        height,
        tiles
    }
}
/// Create a rectangle grid
fn create_rectangle (width: usize, height: usize, border: usize, fill: usize) -> Grid {
    let mut tiles: Vec<Vec<usize>> = Vec::new();
    let w_usize:usize = width;
    if border == fill {
        for _y in 0..height {
            let row = vec![fill; w_usize];
            tiles.push(row);
        }
    } else {
        for y in 0..height {
            if y == 0 || y == height - 1 {
                let row: Vec<usize> = vec![border; w_usize];
                tiles.push(row);
            } else {
                let mut row: Vec<usize> = vec![fill; w_usize];
                row[0] = border;
                row[w_usize - 1] = border;
                tiles.push(row);
            }
        }
    }
    Grid {
        width,
        height,
        tiles,
    }
}
/// Create a circle in a grid. 
fn create_circle (radius:usize, border: usize, fill: usize) -> Grid {
    // To-Do: Every 45 degree increment, a tile gets drawn twice, optimize?

    // Some declarations to make the math faster and simpler
    let r:usize = radius;
    let radius_squared:usize = r * r;
    let diameter:usize = r * 2 + 1;
    let background:usize = 0;
    // Skipping a lot of possible heartache
    if border == background && background == fill {
        return create_grid(diameter, diameter, background);
    } else if radius == 0 {
        return create_grid(1,1, border);
    } else if radius == 1 {
        let mut circle = create_grid(diameter, diameter, background);

        circle.tiles[0][1] = border;
        circle.tiles[2][1] = border;
        circle.tiles[1][0] = border;
        circle.tiles[1][2] = border;
        circle.tiles[1][1] = fill;

        return circle;
    }

    // Actually making the circle
    let mut circle: Grid = create_grid(diameter, diameter, background);

    // So basically the way I generate the circle is first I
    // start with a point at the top of the circle:
    // x . . . .
    // . . . . .
    // . . . . .
    // . . . . .
    // c . . . .
    // then I move over one and check if it's in the circle
    // by using "x^2 + y^2 < r^2". If it's true nothing happens:
    // > x . . .
    // . . . . .
    // . . . . .
    // . . . . .
    // c . . . .
    // Then I repeat the step  in this case
    // "x^2 + y^2 < r^2" evaluates to false so I move down until
    // it's true again:
    // > v . . .
    // . . x . .
    // . . . . .
    // . . . . .
    // c . . . .
    // If I continued this pattern out I'd get:
    // > v . . .
    // . . v . .
    // . . . v .
    // . . . . v
    // c . . . x
    // However if you look you can see that there is symmetry
    // along 45 degrees so I'm doing twice the work I need to!
    // Instead I only perform this function until I'm past the
    // 45 degree point which would be here:
    // > v . . .
    // . . v . .
    // . . . x .
    // . . . . .
    // c . . . .
    // The simple way you could check to see if you're past 45
    // degrees is to wait until you have to go down more than once
    // to satisfy "x^2 + y^2 < r^2". But I've found that
    // "displacement_x + displacement_y * 2 > r" is a fairly
    // inexpensive check to see if I've gone past halfway!
    // ---
    // Once I've figured out 1/8th of the circle I simply
    // flip and rotate it to draw the other 7/8ths which you
    // can see blow!
    // ---
    // I'm too lazy to type out how I fill the circle as well
    // but hopefully this diagram explains it. ;~; I start at
    // dx and go up until the border every time I step over.
    // > v . . .
    // 4 3 v . .
    // 3 2 1 x .
    // 2 1 . . .
    // 1 . . . .

    let mut d1: usize = 0; // The first offset
    let mut d2: usize = 0; // The second offset
    let threshold: usize = radius_squared - ((r - 1) * (r - 1));

    let mut flag:bool = false;

    loop {
        // Check if I step down
        if radius_squared - d1 * d1 - (r - d2 - 1) * (r - d2 - 1) < threshold {
            d2 += 1;
        }

        // Outline

        // Top (12 o'clock)
        circle.tiles[d2][r + d1] = border; // Right of 12 o'clock (1 o'clock and going CW)
        circle.tiles[d2][r - d1] = border; // Left of 12 o'clock (11 o'clock and going CCW)
        // Bottom (6 o'clock)
        circle.tiles[diameter - 1 - d2][r + d1] = border; // Right of 6 o'clock (5 o'clock and going CCW)
        circle.tiles[diameter - 1 - d2][r - d1] = border; // Left of 6 o'clock (6 o'clock and going CW)
        // Right (3 o'clock)
        circle.tiles[r - d1][diameter - 1 - d2] = border; // Above 3 o'clock (2 o'clock and going CCW)
        circle.tiles[r + d1][diameter - 1 - d2] = border; // Above 3 o'clock (4 o'clock and going CW)
        // Left (9 o'clock)
        circle.tiles[r - d1][d2] = border; // Above 9 o'clock (10 o'clock and going CW)
        circle.tiles[r + d1][d2] = border; // Below 9 o'clock (8 o'clock and going CCW)

        // Fill

        if fill != background && (r as isize - d2 as isize - d1 as isize) >= 0 {
            for d in 0..r-d2-d1 {
                circle.tiles[r - d - d1][r + d1] = fill;
                circle.tiles[r - d - d1][r - d1] = fill;

                circle.tiles[r + d + d1][r + d1] = fill;
                circle.tiles[r + d + d1][r - d1] = fill;

                circle.tiles[r + d1][r - d - d1] = fill;
                circle.tiles[r - d1][r - d - d1] = fill;

                circle.tiles[r + d1][r + d + d1] = fill;
                circle.tiles[r - d1][r + d + d1] = fill;
            }
        }

        // Debugging animation
        //std::thread::sleep(std::time::Duration::from_millis(500));
        //println!("{:?}", circle);

        // Check if past 45 degrees
        if flag {break;}
        if d1 + d2 * 2 > r {flag = true;}
        // Increment
        d1 += 1;
    }

    circle

}
/// Create a grid containing a polygon with specified verts
pub fn create_polygon (width: usize, height: usize, vertices: Vec<usize>, fill: usize, border: usize) -> Grid {
    let background:usize = 0;
    let length: usize = vertices.len();
    if length % 2 == 1 || length < 6 {
        panic!("kiiterm::grid::create_polygon: Invalid number of vertices \"{}\"", length);
    }
    // Skipping some possible heartache
    if border == background && background == fill {
        return create_grid(width,height, fill);
    } else if width == 1 || height == 1 {
        return create_grid(width, height, border);
    }
    // Then we can make our polygon
    let mut polygon = create_grid(width, height, background);
    // Now let's draw the lines
    let mut i: usize = 3;
    loop {
        if i >= length {break;}

        polygon = overlay_line(polygon, 
            vertices[i - 3], vertices[i - 2], 
            vertices[i - 1], vertices[i], 
            border, &Overlay::Simple
        );

        i += 2;
    }
    // And close the polygon
    polygon = overlay_line(polygon, 
        vertices[0], vertices[1], 
        vertices[length - 2], vertices[length - 1], 
        border, &Overlay::Simple
    );
    // To-Do: Fill the polygon
    if fill != background {
        let x = width / 2;
        let mut flag = false;
        for y in 0..height {
            if flag == false {
                if polygon.tiles[y][x] == border {
                    flag = true;
                }
            } else {
                if polygon.tiles[y][x] == background {
                    polygon = flood_fill(polygon, x, y, fill, false);
                    break;
                }
            }
        }

    }
    polygon
}
/// All the possible shapes
#[allow(non_camel_case_types)]
pub enum Shape {
    // Isosceles Triangle
    Right_Iso_Tri,
    Left_Iso_Tri,
    Up_Iso_Tri,
    Down_Iso_Tri,

    Circle,
    Rectangle
}
fn parse_shape(shape: Shape, width: usize, height: usize) -> Vec<usize> {
    let w = width - 1;
    let h = height - 1;
    match shape {
        Shape::Right_Iso_Tri => {
            return vec![
                0, 0,
                w, h / 2,
                0, h
            ]
        }
        Shape::Left_Iso_Tri => {
            return vec![
                w, 0,
                w, h,
                w,  h / 2
            ]
        }
        Shape::Up_Iso_Tri => {
            return vec![
                w / 2, 0,
                w, h,
                0, h
            ]
        }
        Shape::Down_Iso_Tri => {
            return vec![
                0, 0,
                w, 0,
                w / 2, h
            ]
        }
        _ => panic!("Wrong function dummy!")
    }
}
/// Create a grid containing a grid::Shape
pub fn create_shape (shape: Shape, width: usize, height: usize, fill: usize, border: usize) -> Grid {
    match shape {
        Shape::Circle => {
            return create_circle(width / 2, border, fill)
        },
        Shape::Rectangle => {
            return create_rectangle(width,height, border,fill)
        },
        _ => {
            let vertices: Vec<usize> = parse_shape(shape, width, height);
            return create_polygon(width, height, vertices, fill, border)
        }
    }
}
/// All the different overlay types.
pub enum Overlay {
    // Copies overlay tile to recieving tile
    Simple,
    // Copies the overlay tile to the recieving tile if the overlay tile != 0
    Transparent,

    // Performs the operation to the recieving tile using the overlay tile.
    Add,
    Subtract,
    Multiply
}
/// Helper function to parse overlays
fn parse_overlay (r: usize, o: usize, overlay: &Overlay) -> usize {
    match overlay {
        Overlay::Simple => {
            return o;
        }
        Overlay::Transparent => {
            if o != 0 {
                return o
            } else {
                return r
            }
        }
        Overlay::Add => {
            return r + o;
        }
        Overlay::Subtract => {
            return r - o;
        }
        Overlay::Multiply => {
            return r * o;
        }
    }
}

/// Overlays a grid (o_grid) on a recieving grid (r_grid).
pub fn overlay_grid (mut r_grid: Grid, o_grid: Grid, x: usize, y: usize, overlay: &Overlay) -> Grid {
    for h in 0..o_grid.height as usize {
        for w in 0..o_grid.width as usize {
            r_grid.tiles[y + h][x + w] = parse_overlay(r_grid.tiles[y + h][x + w], o_grid.tiles[h][w], overlay);
        }
    }
    return r_grid;
}
/// Draw a line on an existing grid.
pub fn overlay_line (mut grid: Grid, x1: usize, y1: usize, x2: usize, y2: usize, fill: usize, overlay: &Overlay) -> Grid {
    // Some error handling so it's easier to debug for a user.
    if x1 >= grid.width {
        panic!("kiiterm::overlay_line: \"The starting X is greater than the Grid's width!\"");
    } else if x2 >= grid.width {
        panic!("kiiterm::overlay_line: \"The ending X is greater than the Grid's width!\"");
    } else if y1 >= grid.height {
        panic!("kiiterm::overlay_line: \"The starting Y is greater than the Grid's height!\"");
    } else if y2 >= grid.height {
        panic!("kiiterm::overlay_line: \"The ending Y is greater than the Grid's height!\"");
    };
    // Variables for X
    let dx: isize = x2 as isize - x1 as isize;
    let sx: isize = dx.signum();
    let mut x: isize = 0;
    // Variables for Y
    let dy: isize = y2 as isize - y1 as isize;
    let sy: isize = dy.signum();
    let mut y: isize = 0;
    // Check for vertical and horizonal lines
    if dx == 0 {
        // The line is vertical
        loop {
            grid.tiles[(y1 as isize + y) as usize][x1] = parse_overlay(grid.tiles[(y1 as isize + y) as usize][x1],fill, overlay);
            if y + y1 as isize == y2 as isize {break;};
            y += sy;
        }
        return grid;
    } else if dy == 0 {
        // The line is horizontal
        loop {
            grid.tiles[y1][(x1 as isize + x) as usize] = parse_overlay(grid.tiles[y1][(x1 as isize + x) as usize], fill, overlay);
            if x + x1 as isize == x2 as isize {break;};
            x += sx;
        }
        return grid;
    }
    // How many times we need to step up
    let m: isize = dx.abs() - dy.abs();
    // Determine the direction
    if m > 0 {
        // How often we need to step up
        let slope = dy as f32 / dx as f32;
        // The line is more horizontal
        loop {
            // Fill in the tile!
            grid.tiles[(y + y1 as isize) as usize][(x + x1 as isize) as usize] = fill;
            // Increment!
            x += sx;
            // Check if we're done
            if x + x1 as isize == x2 as isize {
                grid.tiles[y2][x2] = fill;
                break;
            };
            // Inc
            if (slope * x as f32).abs() > (y as f32).abs() {y += sy;};
        }
    } else if m == 0 {
        // The line is diagonal and we can't divide by zero!
        loop {
            // Fill in the tile!
            grid.tiles[y as usize][x as usize] = fill;
            // Check if we're done
            if x == x2 as isize {break;};
            // Increment if not!
            x += sx;
            y += sy;
        }
    } else {
        // How often we need to step up
        let slope = dx as f32 / dy as f32;
        // The line is more vertical
        loop {
            // Fill in the tile!
            grid.tiles[(y + y1 as isize) as usize][(x + x1 as isize) as usize] = fill;
            // Increment if not!
            y += sy;
            // Check if we're done
            if y + y1 as isize == y2 as isize {
                grid.tiles[y2 as usize][x2 as usize] = fill;
                break;
            };
            // Inc
            if (slope * y as f32).abs() > (x as f32).abs() {x += sx;};
        }
    }

    grid

}
/// Returns a grid with p1 being the top left corner and p2 being the bottom right corner of the original grid.
pub fn crop_grid (grid: Grid, x1: usize, y1: usize, x2: usize, y2: usize) -> Grid {
    let width = x2 - x1;
    let height = y2 - y1;

    let mut new_grid = create_grid(width, height, 0); 
    
    for h in 0..height {
        for w in 0..width {
            new_grid.tiles[h][w] = grid.tiles[y1 + h][x1 + w]
        }
    }

    return new_grid
}
/// Fill a grid at (x, y).
pub fn flood_fill (mut grid: Grid, x: usize, y: usize, fill: usize, additive: bool) -> Grid {
    let bucket_target: usize = grid.tiles[y][x];
    let mut bucket: usize = fill;
    grid.tiles[y][x] = bucket;          
    if additive {bucket += 1};

    if bucket_target != bucket {
        let width = grid.width - 1;
        let height = grid.height - 1;

        let mut current_queue: Vec<(usize, usize)> = vec![(x, y); 1];
        let mut future_queue: Vec<(usize, usize)> = Vec::new();

        loop {
            loop {
                let current_tile: (usize, usize) = current_queue.pop().unwrap();

                // Check left
                if current_tile.0 > 0 {
                    if grid.tiles[current_tile.1][current_tile.0 - 1] == bucket_target {
                        grid.tiles[current_tile.1][current_tile.0 - 1] = bucket;
                        future_queue.push((current_tile.0 - 1, current_tile.1));
                    }
                }
                // Check right
                if current_tile.0 < width {
                    if grid.tiles[current_tile.1][current_tile.0 + 1] == bucket_target {
                        grid.tiles[current_tile.1][current_tile.0 + 1] = bucket;
                        future_queue.push((current_tile.0 + 1, current_tile.1));
                    }
                }
                // Check up
                if current_tile.1 > 0 {
                    if grid.tiles[current_tile.1 - 1][current_tile.0] == bucket_target {
                        grid.tiles[current_tile.1 - 1][current_tile.0] = bucket;
                        future_queue.push((current_tile.0, current_tile.1 - 1));
                    }
                }
                // Check down
                if current_tile.1 < height {
                    if grid.tiles[current_tile.1 + 1][current_tile.0] == bucket_target {
                        grid.tiles[current_tile.1 + 1][current_tile.0] = bucket;
                        future_queue.push((current_tile.0, current_tile.1 + 1));
                    }
                }
    
                if current_queue.is_empty() {break;};
            }
                
    
            // If there's nothing in the queue we're done!
            if future_queue.is_empty() {break;};
            // Else transfer the queues
            current_queue = future_queue.clone();
            // Increment the bucket if needed!            
            if additive {bucket += 1};
            // Prepare the queue for another round! 
            future_queue.clear();
        }
    }

    // Return the grid
    grid
}