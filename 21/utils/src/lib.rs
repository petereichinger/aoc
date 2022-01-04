pub mod bingo;
pub mod coord;

use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::iter::{FilterMap};

type ReadIterator = FilterMap<Lines<BufReader<File>>, fn(std::io::Result<String>) -> Option<String>>;

pub fn read_input_by_lines() -> ReadIterator {
    let file = std::fs::File::open("../input").expect("Couldn't find file");

    let reader = BufReader::new(file);

    reader.lines().filter_map(|l| l.ok())
}

pub fn get_line_count() -> usize {
    let file = std::fs::File::open("../input").expect("Couldn't find file");
    let reader = BufReader::new(file);

    reader.lines().count()
}


pub fn convert_bits_to_integer(iter: &mut dyn Iterator<Item=i32>) -> i32 {
    iter.reduce(|acc, item| (acc * 2 + item)).unwrap()
}

pub fn convert_bit_slice_to_integer(slice: &[i32]) -> i32 {
    slice.iter().copied().reduce(|acc, item| (acc * 2 + item)).unwrap()
}