pub struct Grid {
    pub width: u16,
    pub height: u16,
    pub tiles: Vec<Vec<u8>>
}

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

pub fn create (width: u16, height: u16, fill: u8) -> Grid {
    // Loop to fill in everything
    let mut x: u16 = 0;
    let mut y: u16 = 0;
    let mut tiles: Vec<Vec<u8>> = Vec::new();
    loop {
        if y >= height {break;}
        let mut row: Vec<u8> = Vec::new();

        loop {
            if x >= width {break;}
            row.push(fill); 
            x += 1;
        }
        tiles.push(row);

        x = 0;
        y += 1;
    }

    Grid {
        width,
        height,
        tiles
    }
}