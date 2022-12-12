mod map;

use std::collections::{BinaryHeap, HashMap};

use map::Map;
use termion::{color, style};
use utils_22::{Coord, ORTHOGONAL_NEIGHBOURS};

use crate::map::parse_map;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Entry {
    coord: Coord,
    score: u32,
}

impl Ord for Entry {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.score.cmp(&other.score)
    }
}

impl PartialOrd for Entry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.score.partial_cmp(&other.score)
        // .map(|cmp| cmp.reverse())
    }
}

fn path_finding(map: &Map, start: Coord, end: Coord) -> Vec<Coord> {
    let h: Box<dyn Fn(Coord) -> u32> = Box::new(|coord| (&end.dist(&coord) * 100.0f32) as u32);

    let mut open_set = BinaryHeap::from([Entry {
        coord: start,
        score: h(start),
    }]);
    let mut came_from = HashMap::new();

    let mut g_score = HashMap::new();
    g_score.insert(start, 0u32);

    let mut f_score = HashMap::new();
    f_score.insert(start, h(start));

    while let Some(current) = open_set.pop() {
        if current.coord == end {
            let mut current = current.coord;
            let mut total_path = vec![current];

            while came_from.contains_key(&current) {
                current = *came_from.get(&current).unwrap();
                total_path.push(current);
            }

            total_path.reverse();

            return total_path;
        }

        let current_height = map.get_tile(&current.coord).unwrap();
        for neigh in ORTHOGONAL_NEIGHBOURS
            .iter()
            .map(|&n| current.coord + n)
            .filter(|n| map.coord_on_map(n))
            .filter(|n| {
                let height = map.get_tile(n);

                match height {
                    Some(h) => h.saturating_sub(current_height) < 2,
                    None => false,
                }
            })
        {
            let tentative_g = g_score.get(&current.coord).unwrap() + 1;

            if !g_score.contains_key(&neigh) || tentative_g < *g_score.get(&neigh).unwrap() {
                came_from.insert(neigh, current.coord);
                g_score.insert(neigh, tentative_g);

                if let None = open_set.iter().find(|entry| entry.coord == neigh) {
                    open_set.push(Entry {
                        coord: neigh,
                        score: tentative_g + h(neigh),
                    })
                }
            }
        }
    }

    vec![]
}

fn print_path(map: &Map, path: &Vec<Coord>) {
    for row in 0..map.size().y() {
        for col in 0..map.size().x() {
            let coord = Coord::new(col, row);
            if path.contains(&coord) {
                print!("{}{}", style::Bold, color::Bg(color::LightBlue));
            } else {
                print!("{}{}", style::Reset, color::Bg(color::Reset));
            };
            print!(
                "{}",
                ('a' as u8 + map.get_tile(&coord).unwrap() as u8) as char
            );
        }
        println!("{}{}", style::Reset, color::Bg(color::Reset));
    }
}

fn part1() {
    const INPUT: &str = include_str!("input");
    let (map, start, end) = parse_map(INPUT);
    let path = path_finding(&map, start, end);

    print_path(&map, &path);
    println!("{}", path.len());
}

fn part2() {
    const INPUT: &str = include_str!("input");
    let (map, _, end) = parse_map(INPUT);

    let mut min_path = usize::MAX;
    for y in 0..map.size().y() {
        for x in 0..map.size().x() {
            let coord = Coord::new(x, y);
            if let Some(0) = map.get_tile(&coord) {
                let path = path_finding(&map, coord, end);

                if !path.is_empty() {
                    min_path = min_path.min(path.len());
                }
            }
        }
    }

    println!("{}", min_path);
}

fn main() {
    part1();
    part2();
}

#[cfg(test)]
mod tests {

    use utils_22::Coord;

    use crate::map::parse_map;

    #[test]
    fn parse_works() {
        const INPUT: &str = include_str!("test");
        let (map, start, end) = parse_map(INPUT);

        assert_eq!(map.size(), Coord::new(8, 5));
        assert_eq!(start, Coord::new(0, 0));
        assert_eq!(end, Coord::new(5, 2));
    }

    #[test]
    fn heights_are_correct() {
        const INPUT: &str = include_str!("test");
        let (map, start, end) = parse_map(INPUT);

        assert_eq!(map.get_tile(&start), Some(0));
        assert_eq!(map.get_tile(&end), Some(25));
    }

    #[test]
    fn path_finding() {
        const INPUT: &str = include_str!("test");
        let (map, start, end) = parse_map(INPUT);

        let path = super::path_finding(&map, start, end);

        assert_eq!(path.len() - 1, 31);
    }
}
