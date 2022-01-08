use std::collections::HashSet;
use itertools::Itertools;
use utils::read_input_by_lines;

type Point = (u32, u32);

#[derive(Debug, Copy, Clone)]
enum Fold {
    X(u32),
    Y(u32),
}

impl Fold {
    fn fold(&self, (x, y): Point) -> Point {
        match self {
            &Fold::X(col) => {
                (if x > col { col - (x - col) } else { x }, y)
            }
            &Fold::Y(row) => {
                (x, if y > row { row - (y - row) } else { y })
            }
        }
    }
}

fn main() {
    let mut lines = read_input_by_lines();
    let mut points = HashSet::new();

    while let Some(l) = lines.next() {
        if l.is_empty() {
            break;
        }

        let p: Point = l.split_once(',').map(|(f, s)| (f.parse().unwrap(), s.parse().unwrap())).unwrap();

        points.insert(p);
    }

    let folds: Vec<Fold> = lines.filter_map(|l| {
        let (axis, index) = l.strip_prefix("fold along ").unwrap().split_once('=').unwrap();
        let index = index.parse().unwrap();
        match axis {
            "x" => Some(Fold::X(index)),
            "y" => Some(Fold::Y(index)),
            _ => None
        }
    }).collect();

    for fold in folds {
        points = points.iter().map(|p| fold.fold(*p)).collect();
    }


    let (minx, maxx) = points.iter().map(|p| p.0).minmax().into_option().unwrap();
    let (miny, maxy) = points.iter().map(|p| p.1).minmax().into_option().unwrap();


    for y in 0..=maxy {
        for x in 0..=maxx {
            print!("{}", if points.contains(&(x, y)) {
                "X"
            } else {
                " "
            });
        }
        println!();
    }
}