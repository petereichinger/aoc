use std::collections::HashSet;
use std::iter::Iterator;

const INPUT: &str = include_str!("./input");

fn main() {
    let dirs: Vec<_> = INPUT.chars().map(map_dir).collect();

    let first_year = positions_part_1(&mut dirs.iter());

    let second_year = positions_part_2(&mut dirs.iter());

    println!("{} {}", first_year.len(), second_year.len());
}

fn positions_part_2<'a>(mut dirs: impl Iterator<Item = &'a (i32, i32)>) -> HashSet<(i32, i32)> {
    let mut poss_1 = HashSet::new();

    let mut pos_1 = (0, 0);

    poss_1.insert(pos_1);

    let mut pos_2 = (0, 0);

    let mut poss_2 = poss_1.clone();

    dirs.enumerate().for_each(|(idx, (x, y))| {
        let (pos, poss) = if idx % 2 == 0 {
            (&mut pos_1, &mut poss_1)
        } else {
            (&mut pos_2, &mut poss_2)
        };
        let new_pos = (pos.0 + x, pos.1 + y);

        poss.insert(new_pos);

        *pos = new_pos;
    });

    poss_1.union(&poss_2).copied().collect()
}

fn positions_part_1<'a>(mut dirs: impl Iterator<Item = &'a (i32, i32)>) -> HashSet<(i32, i32)> {
    let mut positions = HashSet::new();

    let mut position = (0, 0);

    positions.insert(position);

    dirs.for_each(|(x, y)| {
        let new_pos = (position.0 + x, position.1 + y);

        positions.insert(new_pos);

        position = new_pos;
    });

    positions
}

fn map_dir(dir: char) -> (i32, i32) {
    match dir {
        '^' => (0, 1),
        '>' => (1, 0),
        'v' => (0, -1),
        '<' => (-1, 0),
        _ => panic!("invalid dir: {}", dir),
    }
}
