use std::cmp::Ordering;
use std::collections::BinaryHeap;
use utils::{read_input_by_lines};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Basin {
    index: i32,
    size: i32,
}

impl Ord for Basin {
    fn cmp(&self, other: &Self) -> Ordering {
        self.size.cmp(&other.size)
    }
}

impl PartialOrd for Basin {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

fn main() {
    let mut height_map = [[9; 102]; 102];

    for (row, line) in read_input_by_lines().enumerate() {
        for (col, char) in line.chars().enumerate() {
            height_map[col + 1][row + 1] = char.to_digit(10).unwrap();
        }
    }

    let height_map = height_map;

    println!("Part 1");

    let mut risk_sum = 0;

    for x in 1..=100 {
        for y in 1..=100 {
            let current = height_map[x][y];
            let neighbours = [height_map[x][y - 1], height_map[x - 1][y], height_map[x + 1][y], height_map[x][y + 1]];
            if neighbours.iter().all(|neighbour| *neighbour > current) {
                risk_sum += 1 + current;
            }
        }
    }

    println!("{:?}", risk_sum);

    println!("Part 2");


    let mut basin_index = 1;

    let mut basin_map = height_map.map(|col| col.map(|h| (h, if h == 9 { 0 } else { -1 })));

    let mut basin_heap: BinaryHeap<Basin> = BinaryHeap::new();

    for x in 1..=100 {
        for y in 1..=100 {
            let (height, basin) = basin_map[x][y];

            if basin == -1 {
                let basin_size = flood_fill(&mut basin_map, (x, y), basin_index);
                basin_heap.push(Basin { index: basin_index, size: basin_size });
                basin_index += 1
            }
        }
    }

    let biggest_basins = [basin_heap.pop().unwrap(), basin_heap.pop().unwrap(), basin_heap.pop().unwrap()];

    println!("{}", biggest_basins[0].size * biggest_basins[1].size * biggest_basins[2].size);
}

fn flood_fill(mut basin_map: &mut [[(u32, i32); 102]; 102], position: (usize, usize), index: i32) -> i32 {
    let mut to_visit = vec![position];

    let mut counter = 0;
    while !to_visit.is_empty() {
        let (x, y) = to_visit.pop().unwrap();

        let (height, basin) = basin_map[x][y];
        if basin == -1 && height < 9u32 {
            counter += 1;
            basin_map[x][y] = (height, index);
            to_visit.push((x, y - 1));
            to_visit.push((x - 1, y));
            to_visit.push((x + 1, y));
            to_visit.push((x, y + 1));
        }
    }

    counter
}

