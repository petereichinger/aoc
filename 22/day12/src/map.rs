use std::fmt::Display;

use utils_22::Coord;
type Height = u32;
pub struct Map {
    tiles: Vec<Vec<Height>>,
    maximum: Coord,
}

impl Map {
    pub fn get_tile(&self, coord: &Coord) -> Option<Height> {
        let row = self.tiles.get(coord.y() as usize)?;
        let col = row.get(coord.x() as usize)?;

        Some(col.clone())
    }

    pub fn size(&self) -> Coord {
        self.maximum
    }

    pub fn coord_on_map(&self, coord: &Coord) -> bool {
        let x = coord.x();
        let y = coord.y();
        x >= 0 && x < self.maximum.x() && y >= 0 && y < self.maximum.y()
    }
}

pub fn parse_map(input: impl AsRef<str>) -> (Map, Coord, Coord) {
    let mut start: Option<Coord> = None;
    let mut end: Option<Coord> = None;
    let mut tiles = vec![];
    for (line_no, line) in input.as_ref().lines().enumerate() {
        let row: Vec<Height> = line
            .chars()
            .enumerate()
            .map(|(col_no, col)| {
                let coord = Coord::new(col_no as i32, line_no as i32);

                let height = match col {
                    'S' => {
                        start = Some(coord);
                        0
                    }
                    'E' => {
                        end = Some(coord);
                        25
                    }
                    'a'..='z' => (col as usize - 'a' as usize) as Height,
                    invalid => panic!("invalid char {invalid}"),
                };

                height
            })
            .collect();
        tiles.push(row);
    }

    let start = start.expect("Got no start point");
    let end = end.expect("Got no end point");
    let height = tiles.len();
    let width = tiles.get(0).map_or(0, |row| row.len());
    (
        Map {
            tiles,
            maximum: Coord::new(width as i32, height as i32),
        },
        start,
        end,
    )
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.tiles {
            row.iter().for_each(|h| {
                write!(f, "{:0>2} ", h).unwrap();
            });
            writeln!(f)?;
        }

        std::fmt::Result::Ok(())
    }
}
