use std::{collections::HashMap, fmt::Display};

use utils_22::Coord;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Cell {
    Empty,
    Wall,
    RestingSand,
}

#[derive(Debug, Clone)]
pub struct Cave {
    min: Coord,
    max: Coord,
    cells: HashMap<Coord, Cell>,
    floor: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DropResult {
    Resting(Coord),
    Overflow,
}

impl From<&str> for Cave {
    fn from(input: &str) -> Self {
        let shapes: Vec<_> = input
            .lines()
            .map(|line| {
                line.split(" -> ")
                    .map(|str| Coord::from(str))
                    .collect::<Vec<_>>()
            })
            .collect();

        let min = shapes
            .iter()
            .flatten()
            .fold(Coord::new(i32::MAX, i32::MAX), |acc, el| acc.min(el));

        let max = shapes
            .iter()
            .flatten()
            .fold(Coord::new(0, 0), |acc, el| acc.max(el));

        let mut cells: HashMap<_, _> = HashMap::new();

        for coords in shapes {
            for pair in coords.windows(2) {
                let begin = &pair[0];
                let end = &pair[1];

                for coord in begin.line_to(end) {
                    cells.insert(coord, Cell::Wall);
                }
            }
        }

        Cave {
            min,
            max,
            cells,
            floor: false,
        }
    }
}

impl Cave {
    pub fn with_floor(input: &str) -> Self {
        let mut cave: Cave = input.into();

        cave.max = &cave.max + &Coord::new(0, 1);

        cave.floor = true;
        cave
    }
}

impl Display for Cave {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in self.min.y()..=self.max.y() {
            for x in self.min.x()..=self.max.x() {
                let cell = self.cells.get(&Coord::new(x, y)).unwrap_or(&Cell::Empty);
                write!(
                    f,
                    "{}",
                    match cell {
                        Cell::Empty => '.',
                        Cell::Wall => '#',
                        Cell::RestingSand => '+',
                    }
                )?
            }
            writeln!(f)?
        }

        Ok(())
    }
}

impl Cave {
    fn get(&self, coord: &Coord) -> &Cell {
        self.cells.get(coord).unwrap_or(&Cell::Empty)
    }

    pub fn drop_sand(&mut self) -> DropResult {
        let mut current_pos = Coord::new(500, 0);

        while current_pos.y() < self.max.y() {
            // println!("{current_pos}");
            let next_posses = [
                &current_pos + &Coord::UP,
                &(&current_pos + &Coord::UP) + &Coord::LEFT,
                &(&current_pos + &Coord::UP) + &Coord::RIGHT,
            ];

            let next_empty = next_posses
                .iter()
                .find(|coord| matches!(self.get(coord), Cell::Empty));

            match next_empty {
                Some(coord) => current_pos = *coord,
                None => {
                    // Found no next spot => Resting Sand and return
                    self.cells.insert(current_pos, Cell::RestingSand);
                    return DropResult::Resting(current_pos);
                }
            }
        }

        if self.floor {
            self.cells.insert(current_pos, Cell::RestingSand);
            DropResult::Resting(current_pos)
        } else {
            DropResult::Overflow
        }
    }
}

#[cfg(test)]
mod tests {

    use utils_22::Coord;

    use super::{Cave, DropResult};

    #[test]
    fn test_display() {
        const EXPECTED: &str = "....#...##
....#...#.
..###...#.
........#.
........#.
#########.
";

        let input = include_str!("test");
        let cave = Cave::from(input);

        let display = format!("{}", cave);

        assert_eq!(display, EXPECTED);
    }

    #[test]
    fn drop_sand() {
        let input = include_str!("test");
        let mut cave = Cave::from(input);

        assert_eq!(cave.drop_sand(), DropResult::Resting(Coord::new(500, 8)));
        assert_eq!(cave.drop_sand(), DropResult::Resting(Coord::new(499, 8)));
        assert_eq!(cave.drop_sand(), DropResult::Resting(Coord::new(501, 8)));
        assert_eq!(cave.drop_sand(), DropResult::Resting(Coord::new(500, 7)));
        assert_eq!(cave.drop_sand(), DropResult::Resting(Coord::new(498, 8)));
    }

    #[test]
    fn after_22() {
        let input = include_str!("test");
        let mut cave = Cave::from(input);

        (0..22).for_each(|_| assert!(matches!(cave.drop_sand(), DropResult::Resting(_))));

        assert_eq!(cave.drop_sand(), DropResult::Resting(Coord::new(497, 5)));
    }
    #[test]
    fn after_23() {
        let input = include_str!("test");
        let mut cave = Cave::from(input);

        (0..23).for_each(|_| assert!(matches!(cave.drop_sand(), DropResult::Resting(_))));

        assert_eq!(cave.drop_sand(), DropResult::Resting(Coord::new(495, 8)));
    }
    #[test]
    fn overflow() {
        let input = include_str!("test");
        let mut cave = Cave::from(input);

        (0..24).for_each(|_| assert!(matches!(cave.drop_sand(), DropResult::Resting(_))));

        assert_eq!(cave.drop_sand(), DropResult::Overflow);
    }

    #[test]
    fn test_floor() {
        let input = include_str!("test");
        let mut cave = Cave::with_floor(input);

        (0..24).for_each(|_| {
            let drop_result = cave.drop_sand();

            println!("{:?}", drop_result);
        });

        print!("{cave}");
        assert_eq!(cave.drop_sand(), DropResult::Resting(Coord::new(493, 10)));
    }

    #[test]
    fn blubb() {
        let input = include_str!("test");
        let mut cave = Cave::with_floor(input);
        let grains = (0..)
            .map_while(|_| match cave.drop_sand() {
                DropResult::Resting(pos) => {
                    println!("{pos}");
                    if pos == Coord::new(500, 0) {
                        None
                    } else {
                        Some(pos)
                    }
                }
                DropResult::Overflow => None,
            })
            .count()
            + 1;

        assert_eq!(grains, 93)
    }
}
