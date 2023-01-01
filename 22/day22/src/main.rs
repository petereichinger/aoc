#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
    Open,
    Wall,
    Void,
}

type Row = Vec<Cell>;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Map {
    rows: Vec<Row>,
    width: isize,
    height: isize,
    face_size: isize,
    cube: bool,
}

impl Map {
    fn cube(input: &str) -> Self {
        let rows = Self::rows_from_input(input);

        let height = rows.len() as isize;
        let width = rows[0].len() as isize;

        let face_size = width / 4;

        Self {
            rows,
            height,
            width,
            cube: true,
            face_size,
        }
    }

    fn flat(input: &str) -> Self {
        let rows = Self::rows_from_input(input);

        let height = rows.len() as isize;
        let width = rows[0].len() as isize;

        let face_size = width / 4;

        Self {
            rows,
            height,
            width,
            cube: true,
            face_size,
        }
    }
    fn rows_from_input(input: &str) -> Vec<Row> {
        let width = input.lines().map(|l| l.len()).max().expect("got no lines");

        let row = vec![Cell::Void; width];

        input
            .lines()
            .map(|l| {
                let mut row = row.clone();
                l.chars().zip(row.iter_mut()).for_each(|(c, cell)| {
                    *cell = match c {
                        ' ' => Cell::Void,
                        '.' => Cell::Open,
                        '#' => Cell::Wall,
                        invalid => panic!("found invalid char in map: '{invalid}'"),
                    };
                });

                row
            })
            .collect()
    }

    fn do_step(&self, position: Position) -> Option<Position> {
        let dir = match position.2 {
            Orientation::Up => (-1, 0),
            Orientation::Right => (0, 1),
            Orientation::Down => (1, 0),
            Orientation::Left => (0, -1),
        };

        let mut next_pos = (
            (position.0 + dir.0 + self.height) % self.height,
            (position.1 + dir.1 + self.width) % self.width,
        );

        while let Cell::Void = self.rows[next_pos.0 as usize][next_pos.1 as usize] {
            // println!("{:?}", next_pos);
            next_pos = (
                (next_pos.0 + dir.0 + self.height) % self.height,
                (next_pos.1 + dir.1 + self.width) % self.width,
            );
        }

        match self.rows[next_pos.0 as usize][next_pos.1 as usize] {
            Cell::Open => Some((next_pos.0, next_pos.1, position.2)),
            Cell::Wall => None,
            Cell::Void => panic!("Should have skipped all void!"),
        }
    }

    fn move_to_adjacent_face(&self, position: Position) -> Option<Position> {
        let face_coord = (position.0 / self.face_size, position.1 / self.face_size);
        let local_coord = (
            position.0 - face_coord.0 * self.face_size,
            position.1 - face_coord.1 * self.face_size,
        );
        let transformation = match face_coord {
            (0, 2) => match position.2 {
                Orientation::Up => (1, 0, Orientation::Up, -1),
                Orientation::Right => (2, 4, Orientation::Right, 1),
                Orientation::Left => (1, 1, Orientation::Up, 1),
                _ => panic!("not needed (0,2)"),
            },
            (1, 0) => match position.2 {
                Orientation::Up => (0, 2, Orientation::Up, -1),
                Orientation::Left => (2, 4, Orientation::Down, -1),
                Orientation::Down => (2, 3, Orientation::Down, -1),
                _ => panic!("not needed (1,1)"),
            },
            (1, 1) => todo!(),
            (1, 2) => todo!(),
            (2, 3) => todo!(),
            (3, 3) => todo!(),

            (y, x) => panic!("Invalid face coordinate ({y},{x})"),
        };

        todo!()
    }

    fn do_cube_step(&self, position: Position) -> Option<Position> {
        let dir = match position.2 {
            Orientation::Up => (-1, 0),
            Orientation::Right => (0, 1),
            Orientation::Down => (1, 0),
            Orientation::Left => (0, -1),
        };

        let next_pos = ((position.0 + dir.0), (position.1 + dir.1));

        let move_to_adjacent_face =
            if let Cell::Void = self.rows[next_pos.0 as usize][next_pos.1 as usize] {
                true
            } else if next_pos.0 < 0
                || next_pos.1 < 0
                || next_pos.0 == self.height
                || next_pos.1 == self.width
            {
                true
            } else {
                false
            };

        if move_to_adjacent_face {
            self.move_to_adjacent_face(position)
        } else {
            match self.rows[next_pos.0 as usize][next_pos.1 as usize] {
                Cell::Open => Some((next_pos.0, next_pos.1, position.2)),
                Cell::Wall => None,
                _ => panic!("Void shouldn't happen in this case!"),
            }
        }
    }

    fn do_move(&self, mut position: Position, distance: usize) -> Position {
        for _ in 0..distance {
            let step_result = if self.cube {
                self.do_cube_step(position)
            } else {
                self.do_step(position)
            };
            if let Some(step_result) = step_result {
                position = step_result;
            } else {
                break;
            }
        }

        position
    }

    fn do_instruction(&self, instruction: &Instruction, position: Position) -> Position {
        match instruction {
            Instruction::Move(dist) => self.do_move(position, *dist),
            Instruction::RotRight => (position.0, position.1, rotate_right(&position.2)),
            Instruction::RotLeft => (position.0, position.1, rotate_left(&position.2)),
        }
    }
    fn follow_instructions(&self, instructions: &Instructions, mut position: Position) -> Position {
        for i in instructions {
            position = self.do_instruction(i, position);
        }

        position
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Instruction {
    Move(usize),
    RotRight,
    RotLeft,
}

type Instructions = Vec<Instruction>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Orientation {
    Up,
    Right,
    Down,
    Left,
}

type Position = (isize, isize, Orientation);

fn parse_instructions(mut input: &str) -> Instructions {
    let mut instructions = vec![];
    loop {
        let next_rot = input.find(|c| c == 'R' || c == 'L').unwrap_or(input.len());

        let movement = input[0..next_rot]
            .parse::<usize>()
            .expect("got invalid movement");

        instructions.push(Instruction::Move(movement));

        if next_rot < input.len() {
            let rot = match &input[next_rot..(next_rot + 1)] {
                "R" => Instruction::RotRight,
                "L" => Instruction::RotLeft,
                x => panic!("got invalid rotation {x}"),
            };
            instructions.push(rot);
            input = &input[(next_rot + 1)..];
        } else {
            break;
        }
    }

    instructions
}

fn parse(input: &str) -> (Map, Instructions) {
    let (map_input, instruction_input) = input.split_once("\n\n").unwrap();

    (Map::flat(map_input), parse_instructions(instruction_input))
}

fn get_start_position(map: &Map) -> Position {
    (
        0,
        map.rows[0]
            .iter()
            .position(|cell| matches!(cell, Cell::Open))
            .expect("found no open spot on row 0") as isize,
        Orientation::Right,
    )
}

fn rotate_right(orientation: &Orientation) -> Orientation {
    match orientation {
        Orientation::Up => Orientation::Right,
        Orientation::Right => Orientation::Down,
        Orientation::Down => Orientation::Left,
        Orientation::Left => Orientation::Up,
    }
}
fn rotate_left(orientation: &Orientation) -> Orientation {
    match orientation {
        Orientation::Up => Orientation::Left,
        Orientation::Right => Orientation::Up,
        Orientation::Down => Orientation::Right,
        Orientation::Left => Orientation::Down,
    }
}

fn calculate_password(position: Position) -> isize {
    1000 * (position.0 + 1)
        + 4 * (position.1 + 1)
        + match position.2 {
            Orientation::Up => 3,
            Orientation::Right => 0,
            Orientation::Down => 1,
            Orientation::Left => 2,
        }
}

fn main() {
    const INPUT: &str = include_str!("input");

    let (map, instructions) = parse(INPUT);

    let position = map.follow_instructions(&instructions, get_start_position(&map));

    println!("{}", calculate_password(position));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = include_str!("test");

    #[test]
    fn parse_map_works() {
        let map_input = "        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.";
        let map = Map::flat(map_input);

        assert_eq!(map.rows.len(), 12);
        assert_eq!(map.rows[0].len(), 16);
        assert_eq!(map.rows[4][3], Cell::Wall);
    }
    #[test]
    fn parse_instructions_work() {
        let instructions_input = "10R5L5R10L4R5L5";

        let instructions = parse_instructions(instructions_input);

        assert_eq!(instructions.len(), 13);
    }

    #[test]
    fn parse_works() {
        let (map, instructions) = parse(TEST);

        assert_eq!(map.rows.len(), 12);
        assert_eq!(instructions.len(), 13);
    }

    #[test]
    fn start_position_works() {
        let (map, _) = parse(TEST);

        assert_eq!(get_start_position(&map), (0, 8, Orientation::Right));
    }

    #[test]
    fn step_into_open_works() {
        let (map, _) = parse(TEST);

        let position = (4, 0, Orientation::Right);

        let next_pos = map.do_step(position);

        assert_eq!(next_pos, Some((4, 1, Orientation::Right)))
    }

    #[test]
    fn step_into_wall_works() {
        let (map, _) = parse(TEST);

        let position = (4, 2, Orientation::Right);

        let next_pos = map.do_step(position);

        assert_eq!(next_pos, None)
    }

    #[test]
    fn step_into_void_works() {
        let (map, _) = parse(TEST);

        let position = (4, 0, Orientation::Up);

        let next_pos = map.do_step(position);

        assert_eq!(next_pos, Some((7, 0, Orientation::Up)))
    }
    #[test]
    fn step_into_void_then_wall_works() {
        let (map, _) = parse(TEST);

        let position = (4, 0, Orientation::Left);

        let next_pos = map.do_step(position);

        assert_eq!(next_pos, None)
    }

    #[test]
    fn test_straight_move() {
        let (map, _) = parse(TEST);

        let position = (0, 8, Orientation::Right);

        let position = map.do_move(position, 10);

        assert_eq!(position, (0, 10, Orientation::Right));
    }

    #[test]
    fn test_void_move() {
        let (map, _) = parse(TEST);

        let position = (0, 8, Orientation::Left);

        let position = map.do_move(position, 10);

        assert_eq!(position, (0, 8, Orientation::Left));
    }

    #[test]
    fn test_map_with_instructions() {
        let (map, instructions) = parse(TEST);

        let final_pos = map.follow_instructions(&instructions, get_start_position(&map));

        assert_eq!(final_pos, (5, 7, Orientation::Right));
    }

    #[test]
    fn password_is_correct() {
        assert_eq!(6032, calculate_password((5, 7, Orientation::Right)));
    }
}
