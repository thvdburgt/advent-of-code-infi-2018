use std::str::FromStr;

#[derive(Clone, Eq, PartialEq, Hash)]
pub struct Tile {
    pub exit_north: bool,
    pub exit_east: bool,
    pub exit_west: bool,
    pub exit_south: bool,
}

impl Tile {
    pub fn new() -> Self {
        Tile {
            exit_north: false,
            exit_east: false,
            exit_west: false,
            exit_south: false,
        }
    }
}

impl FromStr for Tile {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        let mut tile = Self::new();

        match chars.next().ok_or(())? {
            ' ' => {}
            '║' => {
                tile.exit_north = true;
                tile.exit_south = true;
            }
            '╔' => {
                tile.exit_east = true;
                tile.exit_south = true;
            }
            '╗' => {
                tile.exit_south = true;
                tile.exit_west = true;
            }
            '╠' => {
                tile.exit_north = true;
                tile.exit_east = true;
                tile.exit_south = true;
            }
            '╦' => {
                tile.exit_east = true;
                tile.exit_south = true;
                tile.exit_west = true;
            }
            '╚' => {
                tile.exit_north = true;
                tile.exit_east = true;
            }
            '╝' => {
                tile.exit_north = true;
                tile.exit_west = true;
            }
            '╬' => {
                tile.exit_north = true;
                tile.exit_east = true;
                tile.exit_south = true;
                tile.exit_west = true;
            }
            '╩' => {
                tile.exit_north = true;
                tile.exit_east = true;
                tile.exit_west = true;
            }
            '═' => {
                tile.exit_east = true;
                tile.exit_west = true;
            }
            '╣' => {
                tile.exit_north = true;
                tile.exit_south = true;
                tile.exit_west = true;
            }
            _ => {
                println!("WTF");
                return Err(());
            }
        }
        if chars.next() != None {
            println!("WTF");
            Err(())
        } else {
            Ok(tile)
        }
    }
}
