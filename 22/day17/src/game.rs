use std::fmt::Display;

use crate::{
    direction_stream::{get_direction_iter, Direction},
    shapes::{get_shapes_iter, Shape},
};

struct Row {
    cells: [bool; 7],
}

impl Row {
    fn new() -> Self {
        Row { cells: [false; 7] }
    }
}

pub struct Game {
    pub height: usize,
    pub drops: usize,
    rows: Vec<Row>,
    direction_stream: Box<dyn Iterator<Item = Direction>>,
    shapes: Box<dyn Iterator<Item = Shape>>,
}

impl Game {
    pub fn new(input: &'static str) -> Self {
        Self {
            height: 0,
            drops: 0,
            rows: vec![],
            direction_stream: Box::new(get_direction_iter(input)),
            shapes: Box::new(get_shapes_iter()),
        }
    }

    fn check_collision(&self, shape: &Shape, pos: &(usize, usize)) -> bool {
        shape.rows.iter().enumerate().any(|(idx, shape_row)| {
            let tower_row = &self.rows[pos.1 + idx].cells[pos.0..(pos.0 + shape.width)];
            assert_eq!(shape_row.len(), tower_row.len());

            shape_row.iter().zip(tower_row).any(|(&s, &t)| s && t)
        })
    }
    pub fn drop_rock(&mut self) -> (usize, usize) {
        let shape = self.shapes.next().unwrap();

        let new_rows = shape.height + 3;

        self.rows.resize_with(self.height + new_rows, || Row::new());

        let mut pos = (2usize, self.height + 3);

        loop {
            let dir = self.direction_stream.next().unwrap();
            let new_x = match dir {
                Direction::Left => {
                    if pos.0 == 0 {
                        0
                    } else {
                        let test_pos = (pos.0 - 1, pos.1);
                        if self.check_collision(&shape, &test_pos) {
                            pos.0
                        } else {
                            pos.0 - 1
                        }
                    }
                }
                Direction::Right => {
                    if pos.0 + shape.width < 7 {
                        let test_pos = (pos.0 + 1, pos.1);
                        if self.check_collision(&shape, &test_pos) {
                            pos.0
                        } else {
                            pos.0 + 1
                        }
                    } else {
                        pos.0
                    }
                }
            };

            pos.0 = new_x;

            if pos.1 == 0 {
                break;
            }

            let new_y = pos.1 - 1;

            let test_pos = (pos.0, new_y);
            let collision = self.check_collision(&shape, &test_pos);

            if collision {
                break;
            }
            pos.1 = new_y;
        }

        for (idx, shape_row) in shape.rows.iter().enumerate() {
            let tower_row = &mut self.rows[pos.1 + idx].cells[pos.0..(pos.0 + shape.width)];

            tower_row
                .iter_mut()
                .zip(shape_row.iter())
                .for_each(|(t, s)| {
                    *t |= s;
                });
        }

        self.drops += 1;
        self.height = self.rows.len()
            - self
                .rows
                .iter()
                .rev()
                .take_while(|r| r.cells.iter().all(|c| !c))
                .count();

        pos
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.rows.iter().rev() {
            let row = row
                .cells
                .iter()
                .map(|&c| if c { '#' } else { '.' })
                .collect::<String>();
            writeln!(f, "{}", row)?
        }
        writeln!(f, "-------")
    }
}
