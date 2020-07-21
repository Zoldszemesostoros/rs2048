use rand::random;

use std::fmt;
#[derive(Copy, Clone, Debug)]
/// Represens a tile in the table.
pub struct Tile {
    /// Holds the value of the tile.
    value: u32,
}

impl Tile {
    /// Returns a new tile, with the given value.
    pub fn new(value: u32) -> Tile {
        Tile { value }
    }

    /// Doubles the value of the tile.
    pub fn double(&mut self) {
        self.value *= 2;
    }

    /// Waits for 4 tiles.
    /// The empty ones go to the end.
    /// Same AND contacting tiles are collapsing.
    /// If a tile comes from a previous collapse (in this turn), it won't collapse with others.
    pub fn squeeze(tiles: [Option<Tile>; 4]) -> [Option<Tile>; 4] {
        let mut result: Vec<Tile> = Vec::with_capacity(4);
        let mut last_doubled = false;
        for current_tile in tiles.iter() {
            if current_tile.is_some() {
                let current_tile = current_tile.as_ref().unwrap();
                if !last_doubled
                    && result.len() > 0
                    && current_tile == result.last().expect("Cannot read last element. :(")
                {
                    let len = result.len().clone();
                    result[len - 1].double();
                    last_doubled = true;
                } else {
                    result.push(*current_tile);
                }
            }
        }
        let mut result_arr = [None, None, None, None];
        for (i, item) in result.iter().enumerate() {
            result_arr[i] = Some(*item);
        }
        result_arr
    }
}

impl PartialEq for Tile {
    /// Checks the equality of 2 tile.
    /// If their values are equal, they're equal.
    fn eq(&self, other: &Tile) -> bool {
        self.value == other.value
    }
}

impl fmt::Display for Tile {
    /// Displays a tile (value with padding).
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:^6}", self.value)
    }
}
#[derive(Copy, Clone, PartialEq)]
/// Represents the game table.
/// Contains 16 tile.
pub struct Table {
    tiles: [Option<Tile>; 16],
}

impl Table {
    /// Creates a new, empty table.
    pub fn new() -> Table {
        Table {
            tiles: [
                None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None,
            ],
        }
    }
    /// Sends empty tiles to the end.
    /// Same AND horizontally contacting tiles are collapsing.
    /// If a tile comes from a previous collapse (in this turn), it won't collapse with others.
    pub fn swipe_left(&mut self) {
        for i in 0..4 {
            let result = Tile::squeeze([
                self.tiles[i * 4 + 0],
                self.tiles[i * 4 + 1],
                self.tiles[i * 4 + 2],
                self.tiles[i * 4 + 3],
            ]);
            self.tiles[i * 4 + 0] = result[0];
            self.tiles[i * 4 + 1] = result[1];
            self.tiles[i * 4 + 2] = result[2];
            self.tiles[i * 4 + 3] = result[3];
        }
    }

    /// Sends empty tiles to the left.
    /// Same AND horizontally contacting tiles are collapsing.
    /// If a tile comes from a previous collapse (in this turn), it won't collapse with others.
    pub fn swipe_right(&mut self) {
        for i in 0..4 {
            let result = Tile::squeeze([
                self.tiles[i * 4 + 3],
                self.tiles[i * 4 + 2],
                self.tiles[i * 4 + 1],
                self.tiles[i * 4 + 0],
            ]);
            self.tiles[i * 4 + 3] = result[0];
            self.tiles[i * 4 + 2] = result[1];
            self.tiles[i * 4 + 1] = result[2];
            self.tiles[i * 4 + 0] = result[3];
        }
    }

    /// Sends empty tiles to the top.
    /// Same AND vertically contacting tiles are collapsing.
    /// If a tile comes from a previous collapse (in this turn), it won't collapse with others.
    pub fn swipe_down(&mut self) {
        for i in 0..4 {
            let result = Tile::squeeze([
                self.tiles[3 * 4 + i],
                self.tiles[2 * 4 + i],
                self.tiles[1 * 4 + i],
                self.tiles[0 * 4 + i],
            ]);
            self.tiles[3 * 4 + i] = result[0];
            self.tiles[2 * 4 + i] = result[1];
            self.tiles[1 * 4 + i] = result[2];
            self.tiles[0 * 4 + i] = result[3];
        }
    }

    /// Sends empty tiles to the bottom.
    /// Same AND vertically contacting tiles are collapsing.
    /// If a tile comes from a previous collapse (in this turn), it won't collapse with others.
    pub fn swipe_up(&mut self) {
        for i in 0..4 {
            let result = Tile::squeeze([
                self.tiles[0 * 4 + i],
                self.tiles[1 * 4 + i],
                self.tiles[2 * 4 + i],
                self.tiles[3 * 4 + i],
            ]);
            self.tiles[0 * 4 + i] = result[0];
            self.tiles[1 * 4 + i] = result[1];
            self.tiles[2 * 4 + i] = result[2];
            self.tiles[3 * 4 + i] = result[3];
        }
    }

    /// Adds a new tile to the table.
    /// It's value is 2 or 4 (randomly chosen).
    /// It's place is randomly chosen too.
    /// If the table is full, it returns an Err<()>.
    pub fn add_tile(&mut self) -> Result<(), ()> {
        let mut empty = Vec::with_capacity(4);
        for i in 0..16 {
            if self.tiles[i].is_none() {
                empty.push(i);
            }
        }
        let emptys = empty.len();
        if emptys == 0 {
            Err(())
        } else {
            let random_index_index = random::<usize>() % emptys;
            let random_tile_index = empty[random_index_index];
            let value = (random::<u32>() % 2 + 1) * 2;
            self.tiles[random_tile_index] = Some(Tile::new(value));
            Ok(())
        }
    }

    /// If the game is over, returns true.
    /// It means that the table is full, and there aren't any same and contacting tiles.
    pub fn is_game_over(&self) -> bool {
        for i in 0..16 {
            if self.tiles[i].is_none() {
                return false;
            }

            if i % 4 != 3 {
                if self.tiles[i] == self.tiles[i + 1] {
                    return false;
                }
            }

            if i < 12 {
                if self.tiles[i] == self.tiles[i + 4] {
                    return false;
                }
            }
        }

        true
    }
}

impl fmt::Display for Table {
    /// Displays the table.
    /// Tiles are separated with hyphens and vertical bars.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        const VISUAL_SEPARATOR: &str = "-----------------------------\n";
        write!(f, "{}", VISUAL_SEPARATOR).expect("Cannot write visual separator. :(");
        for i in 0..4 {
            let mut content: String = "".to_string();
            for j in 0..4 {
                match &self.tiles[i * 4 + j] {
                    Some(tile) => content = format!("{}|{}", content, tile),
                    None => content = format!("{}|      ", content),
                }
            }
            content = format!("{}|\n", content);
            write!(f, "{}", content).expect("Cannot write content. :(");
            write!(f, "{}", VISUAL_SEPARATOR).expect("Cannot write visual separator. :(");
        }
        Ok(())
    }
}
