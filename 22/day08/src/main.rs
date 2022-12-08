use std::{collections::HashSet, thread::current};

const INPUT: &str = include_str!("input");
const TEST: &str = include_str!("test");

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
struct Coord {
    x: usize,
    y: usize,
}

type Height = i32;

type Entry = (Coord, Height);
fn parse(input: &str) -> Vec<Vec<Entry>> {
    let mut output = vec![];

    for (y, line) in input.lines().enumerate() {
        output.push(
            line.chars()
                .enumerate()
                .map(|(x, c)| {
                    let height = c.to_digit(10).unwrap();
                    let coord = Coord { x, y };
                    (coord, height as i32)
                })
                .collect(),
        )
    }

    return output;
}

fn calc_visibility<'a>(trees: impl Iterator<Item = &'a Entry>) -> HashSet<Coord> {
    let mut current_height = -1;

    let mut output = HashSet::new();

    for (coord, height) in trees {
        if *height > current_height {
            current_height = *height;
            output.insert(coord.clone());
        }
    }

    output
}

struct ColumnIterator<'a> {
    grid: &'a Vec<Vec<Entry>>,
    column: usize,
    index: usize,
}

impl<'a> ColumnIterator<'a> {
    fn new(grid: &'a Vec<Vec<Entry>>, column: usize) -> Self {
        ColumnIterator {
            grid,
            column,
            index: 0,
        }
    }
}

impl<'a> Iterator for ColumnIterator<'a> {
    type Item = &'a Entry;

    fn next(&mut self) -> Option<Self::Item> {
        let current_index = self.index;
        self.index += 1;
        if current_index < self.grid.len() {
            Some(&self.grid[current_index][self.column])
        } else {
            None
        }
    }
}

fn main() {
    let grid = parse(INPUT);

    let mut visible_trees = HashSet::new();

    for row in &grid {
        let fwd = calc_visibility(row.iter());
        let rev = calc_visibility(row.iter().rev());

        visible_trees = visible_trees
            .into_iter()
            .chain(fwd.into_iter().chain(rev.into_iter()))
            .collect();
    }

    let columns = grid[0].len();
    for column in 0..columns {
        let col_entries: Vec<&Entry> = ColumnIterator::new(&grid, column).collect();
        println!("{:?}", col_entries);
        let fwd = calc_visibility(col_entries.iter().copied());
        let rev = calc_visibility(col_entries.iter().copied().rev());

        visible_trees = visible_trees
            .into_iter()
            .chain(fwd.into_iter().chain(rev.into_iter()))
            .collect();
    }

    println!("{:#?}", visible_trees);
    println!("{:?}", visible_trees.len())
}
