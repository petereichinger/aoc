use std::collections::HashSet;

use utils_22::Coord;

fn coord_from_char(dir: char) -> Coord {
    match dir {
        'R' => Coord::new(1, 0),
        'U' => Coord::new(0, 1),
        'L' => Coord::new(-1, 0),
        'D' => Coord::new(0, 1),
        _ => panic!("Invalid dir {}", dir),
    }
}

fn main() {
    rope_move(2);
    rope_move(10);
}

fn rope_move(num_knots: usize) {
    const INPUT: &str = include_str!("input");

    let mut knots = vec![Coord::default(); num_knots];

    let mut visited_s = HashSet::new();
    visited_s.insert(Coord::default());

    for instruction in INPUT.lines() {
        let (dir, count) = instruction.split_once(' ').unwrap();

        let dir = coord_from_char(dir.chars().next().unwrap());
        let count = count.parse::<i32>().unwrap();

        for _ in 0..count {
            knots[0] += &dir;
            for idx in 1..knots.len() {
                let k = &mut knots[idx - 1..=idx];
                let (curr_h, curr_s) = k.split_at_mut(1);

                let curr_h = &mut curr_h[0];
                let curr_s = &mut curr_s[0];

                if curr_h.distance_to(&curr_s) < 2 {
                    continue;
                }

                let diff = &*curr_h - &*curr_s;

                *curr_s += &diff.limit_to_neighbour();
            }
            visited_s.insert(knots[knots.len() - 1]);
        }
    }

    println!("{} {}", num_knots, visited_s.len());
}
