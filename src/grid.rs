pub struct Grid {
    pub width: u16,
    pub height: u16,
    pub tiles: Vec<Vec<u8>>
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
pub fn create_grid (width: u16, height: u16, fill: u8) -> Grid {
    // Loop to fill in everything
    let mut tiles: Vec<Vec<u8>> = Vec::new();
    for _y in 0..height {
        let mut row: Vec<u8> = Vec::new();
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
pub fn create_rectangle (width: u16, height: u16, border: u8, fill: u8) -> Grid {
    let mut tiles: Vec<Vec<u8>> = Vec::new();
    let w_usize:usize = width as usize;
    if border == fill {
        for _y in 0..height {
            let row = vec![fill; w_usize];
            tiles.push(row);
        }
    } else {
        for y in 0..height {
            if y == 0 || y == height - 1 {
                let row: Vec<u8> = vec![border; w_usize];
                tiles.push(row);
            } else {
                let mut row: Vec<u8> = vec![fill; w_usize];
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
pub fn create_circle (radius:u16, border: u8, fill: u8, background: u8) -> Grid {
    // To-Do: Every 45 degree increment, a tile gets drawn twice, optimize?

    // Some declarations to make the math faster and simpler
    let r = radius as usize;
    let radius_squared = r * r;
    let diameter = r * 2 + 1;
    // Skipping a lot of possible heartache
    if border == background && background == fill {
        return create_grid(diameter as u16, diameter as u16, background);
    } else if radius == 0 {
        return create_grid(1,1, border);
    } else if radius == 1 {
        let mut circle = create_grid(diameter as u16, diameter as u16, background);

        circle.tiles[0][1] = border;
        circle.tiles[2][1] = border;
        circle.tiles[1][0] = border;
        circle.tiles[1][2] = border;
        circle.tiles[1][1] = fill;

        return circle;
    }

    // Actually making the circle
    let mut circle: Grid = create_grid(diameter as u16, diameter as u16, background);

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

        if fill != background {
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
        if d1 + d2 * 2 > r {break;}
        // Increment
        d1 += 1;
    }

    circle

}
pub fn create_polygon (width: u16, height: u16, vertices: Vec<u16>, fill: u8, border: u8, background: u8) -> Grid {
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

        polygon = draw_line(polygon, 
            vertices[i - 3], vertices[i - 2], 
            vertices[i - 1], vertices[i], 
            border
        );

        i += 2;
    }
    // And close the polygon
    polygon = draw_line(polygon, 
        vertices[0], vertices[1], 
        vertices[length - 2], vertices[length - 1], 
        border
    );
    // To-Do: Fill the polygon
    polygon
}
/// Draw a line on an existing grid
pub fn draw_line (mut grid: Grid, start_x: u16, start_y: u16, end_x: u16, end_y: u16, fill: u8) -> Grid {
    // Some error handling so it's easier to debug for a user.
    if start_x >= grid.width {
        panic!("kiiterm::draw_line: \"The starting X is greater than the Grid's width!\"");
    } else if end_x >= grid.width {
        panic!("kiiterm::draw_line: \"The ending X is greater than the Grid's width!\"");
    } else if start_y >= grid.height {
        panic!("kiiterm::draw_line: \"The starting Y is greater than the Grid's height!\"");
    } else if end_y >= grid.height {
        panic!("kiiterm::draw_line: \"The ending Y is greater than the Grid's height!\"");
    }
    // Get the differences in x and y
    let mut x_dif: isize = end_x as isize - start_x as isize;
    let mut y_dif: isize = end_y as isize - start_y as isize;
    // Get the overall direction of the line
    let dir: isize = x_dif.abs() - y_dif.abs();

    let mut d: isize = 0;

    if x_dif == 0 {
        // It's a vertical line
        let inc = y_dif.signum();
        for y in 0..y_dif.abs() + 1 {
            grid.tiles[(start_y as isize + y * inc) as usize][start_x as usize] = fill;
        }
    } else if y_dif == 0 {
        // It's a vertical line
        let inc = x_dif.signum();
        for x in 0..x_dif.abs() + 1 {
            grid.tiles[start_y as usize][(start_x as isize + x * inc) as usize] = fill;
        }
    } else if dir < 0 {
        // It's vertical
        let sx: isize;
        let sy: isize;

        let inc = y_dif / x_dif;
        let off = inc / 2;
        let tic = inc.signum();
        
        // To-Do: I haven't written code this bad in a v long time
        // pls fix when it's not 5:04am.

        // Flipping if needed.
        if y_dif > 0 {
            sx = start_x as isize;
            sy = start_y as isize;
        } else {
            y_dif = y_dif.abs();
            sx = end_x as isize;
            sy = end_y as isize;
        }

        for y in 0..y_dif {
            grid.tiles[(sy + y) as usize][(sx + d) as usize ] = fill;
            if (y + off) % inc == 0 {d += tic;}
        }

    } else if dir == 0 {
        // It's diagonal

        // Initializing
        let sx: isize;
        let sy: isize;
        let inc = y_dif.signum();
        // Flipping if needed.
        if x_dif > 0 {
            sx = start_x as isize;
            sy = start_y as isize;
        } else {
            x_dif = x_dif.abs();
            sx = end_x as isize;
            sy = end_y as isize;
        }
        // Drawing the tiles
        for x in 0..x_dif {
            grid.tiles[(sy + x * inc) as usize][(sx + x) as usize] = fill;
        }
    } else if dir > 0 {
        // It's horizontal

        // Initializing
        let sx: isize;
        let sy: isize;
        let inc: isize = x_dif / y_dif;
        let off: isize = inc / 2;
        let tic: isize = inc.signum();
        // Flipping if needed.
        if x_dif > 0 {
            sx = start_x as isize;
            sy = start_y as isize;
        } else {
            x_dif = x_dif.abs();
            sx = end_x as isize;
            sy = end_y as isize;
        }
        // Drawing the tiles
        for x in 0..x_dif {
            grid.tiles[(sy + d) as usize][(sx + x) as usize] = fill;
            if (x + off) % inc == 0 {d += tic;}
        }
    }

    grid
}
///
pub fn flood_fill (mut grid: Grid, x: usize, y: usize, fill: u8, additive: bool) -> Grid {
    let bucket_target: u8 = grid.tiles[y][x];
    let mut bucket: u8 = fill;

    if bucket_target != bucket {
        let width =(grid.width - 1) as usize;
        let height = (grid.height - 1) as usize;

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