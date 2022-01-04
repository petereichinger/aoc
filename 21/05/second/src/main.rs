use std::collections::HashMap;
use utils::{read_input_by_lines, coord::Coord};
use utils::coord::{Line, LineInterpolator};


fn main() {
    let lines: Vec<_> = read_input_by_lines()
        .map(|l| {
            let (f, s) = l.split_once(" -> ").unwrap();
            Line::from_coords(Coord::from(f), Coord::from(s))
        }).collect();


    let mut coord_map = HashMap::new();

    for line in &lines {
        for coord in LineInterpolator::from(line) {
            let mut entry = coord_map.entry(coord).or_insert(0);

            *entry += 1;
        }
    }

    let dangerous = coord_map.iter().filter(|(coord, count)| *count > &1).count();

    println!("{}", dangerous);
}