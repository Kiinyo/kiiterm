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