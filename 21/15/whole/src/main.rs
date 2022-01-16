use std::fmt::{Display, Formatter};
use utils::{read_by_lines, read_input_by_lines};

#[derive(Default, Debug, Copy, Clone, Eq, PartialEq)]
struct Coordinate {
    y: usize,
    x: usize,
}


impl From<(usize, usize)> for Coordinate {
    fn from((y, x): (usize, usize)) -> Self {
        Self { y, x }
    }
}

impl Coordinate {
    fn heuristic_distance(&self, other: &Self) -> u32 {
        let y_diff = other.y as i32 - self.y as i32;
        let x_diff = other.x as i32 - self.x as i32;
        (y_diff.abs() + x_diff.abs()) as u32
    }
}

#[derive(Default, Debug, Copy, Clone)]
struct Cell {
    entry_cost: u32,
    total_cost: u32,
    predecessor: Option<Coordinate>,
}

impl Cell {
    fn new(entry_cost: u32) -> Cell {
        Cell {
            entry_cost: ((entry_cost - 1) % 9) + 1,
            total_cost: u32::MAX,
            predecessor: None,
        }
    }
}

#[derive(Default, Debug, Clone)]
struct Maze {
    cells: Vec<Vec<Cell>>,
    height: usize,
    width: usize,
}

impl Display for Maze {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in &self.cells {
            for col in row {
                write!(f, "{}", col.entry_cost)?;
            }
            writeln!(f, " ")?;
        }
        writeln!(f);

        Result::Ok(())
    }
}

impl Maze {
    fn from_input(file_name: &str, times: u32) -> Maze {
        let mut file_cells: Vec<Vec<Cell>> = vec![];

        for (y, line_str) in read_by_lines(file_name).enumerate() {
            file_cells.push(Default::default());

            let row = file_cells.get_mut(y).unwrap();
            for (_x, value) in line_str.chars().enumerate() {
                let cost = value.to_digit(10).unwrap();
                row.push(Cell::new(cost));
            }
        }

        let mut x_repeated: Vec<Vec<Cell>> = vec![];

        for row in &file_cells {
            let mut line = vec![];
            for repeat in 0..times {
                line.extend(row.iter().map(|c| Cell::new(c.entry_cost + repeat)));
            }
            x_repeated.push(line);
        }

        let mut cells = vec![];

        for repeat in 0..times {
            for row in &x_repeated {
                let mapped_row: Vec<Cell> = row.iter().map(|c| Cell::new(c.entry_cost + repeat)).collect();
                cells.push(mapped_row);
            }
        }


        cells[0][0].total_cost = 0;

        let height = cells.len();
        let width = cells[0].len();
        Maze { cells, height, width }
    }


    fn get_cell(&self, coord: Coordinate) -> &Cell {
        &self.cells[coord.y][coord.x]
    }

    fn get_cell_mut(&mut self, coord: Coordinate) -> &mut Cell {
        &mut self.cells[coord.y][coord.x]
    }

    fn get_neighbour(&self, coord: Coordinate, (diry, dirx): (i32, i32)) -> Option<(Cell, Coordinate)>
    {
        let (neighbour_y, neighbour_x) = (coord.y as i32 + diry, coord.x as i32 + dirx);

        if neighbour_y >= 0 && neighbour_y < self.height as i32
            && neighbour_x >= 0 && neighbour_x < self.width as i32 {
            let neighbour_x = neighbour_x as usize;
            let neighbour_y = neighbour_y as usize;
            Some((self.cells[neighbour_y][neighbour_x], Coordinate::from((neighbour_y, neighbour_x))))
        } else {
            None
        }
    }
}


fn main() {
    let mut maze = Maze::from_input("../input", 5);

    println!("{}x{}", maze.height, maze.width);

    println!("{}", maze);
    // return;
    let end = Coordinate { y: maze.height - 1, x: maze.width - 1 };
    let start = Coordinate { y: 0, x: 0 };

    let mut fringe = vec![(start, 0)];

    while !fringe.is_empty() {
        for (coord, value) in &mut fringe {
            *value = maze.get_cell(*coord).total_cost + coord.heuristic_distance(&end);
        }

        fringe.sort_by(|f, s| s.1.cmp(&f.1));

        // println!("{:?}", fringe);

        let (current, _val) = fringe.pop().unwrap();

        if current == end {
            break;
        }

        let neighbours = [maze.get_neighbour(current, (-1, 0)),
            maze.get_neighbour(current, (1, 0)),
            maze.get_neighbour(current, (0, -1)),
            maze.get_neighbour(current, (0, 1))];

        for (neighbour, coord) in neighbours.into_iter().filter_map(|n| n) {
            let new_cost = maze.get_cell(current).total_cost + neighbour.entry_cost;

            if new_cost < neighbour.total_cost {
                *maze.get_cell_mut(coord) = Cell {
                    total_cost: new_cost,
                    predecessor: Some(current),
                    ..neighbour
                };

                fringe.push(((coord), 0));
            }
        }
    }

    println!("{}", maze.get_cell(end).total_cost);

    // let mut cells = vec![end];
    //
    // let mut bwd_mover = maze.get_cell(end);
    //
    // while let Some(pred) = bwd_mover.predecessor {
    //     cells.push(pred);
    //     bwd_mover = maze.get_cell(pred)
    // }
    //
    // // cells.iter().map(|c| maze.get_cell(c));
    //
    // println!("{} {:?}", cells.len(), cells);
}