use std::{fmt::Display, thread::current};

use utils_22::Coord;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Cell {
    Empty,
    Wall,
    RestingSand,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cave {
    min: Coord,
    max: Coord,
    cells: Vec<Vec<Cell>>,
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

        let mut cells: Vec<_> = (0..=max.y())
            .map(|_| vec![Cell::Empty; (max.x() + 1) as usize])
            .collect();

        for coords in shapes {
            for pair in coords.windows(2) {
                let begin = &pair[0];
                let end = &pair[1];

                for coord in begin.line_to(end) {
                    let row = cells.get_mut(coord.y() as usize).unwrap();
                    let cell = row.get_mut(coord.x() as usize).unwrap();

                    *cell = Cell::Wall;
                }
            }
        }

        Cave { min, max, cells }
    }
}

impl Display for Cave {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.cells.iter().skip(self.min.y() as usize) {
            for cell in row.iter().skip(self.min.x() as usize) {
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

            writeln!(f)?;
        }

        Ok(())
    }
}

impl Cave {
    fn get(&self, coord: &Coord) -> Option<&Cell> {
        let row = self.cells.get(coord.y() as usize)?;
        row.get(coord.x() as usize)
    }
    fn get_mut(&mut self, coord: &Coord) -> Option<&mut Cell> {
        let row = self.cells.get_mut(coord.y() as usize)?;
        row.get_mut(coord.x() as usize)
    }

    pub fn drop_sand(&mut self) -> DropResult {
        let mut current_pos = Coord::new(500, 0);

        while current_pos.y() < self.max.y() {
            let next_posses = [
                &current_pos + &Coord::UP,
                &(&current_pos + &Coord::UP) + &Coord::LEFT,
                &(&current_pos + &Coord::UP) + &Coord::RIGHT,
            ];
            println!("{:?}", next_posses);
            let next_empty = next_posses
                .iter()
                .find(|coord| matches!(self.get(coord), Some(Cell::Empty)));

            match next_empty {
                Some(coord) => current_pos = *coord,
                None => {
                    // Found no next spot => Resting Sand and return
                    *(self.get_mut(&current_pos).unwrap()) = Cell::RestingSand;
                    return DropResult::Resting(current_pos);
                }
            }
        }

        DropResult::Overflow
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
}
