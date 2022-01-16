pub mod bingo;
pub mod coord;

use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn read_by_lines(file_name: &str) -> impl Iterator<Item=String> {
    let file = File::open(file_name).expect("Couldn't find file");

    let reader = BufReader::new(file);

    reader.lines().filter_map(|l| Some(l.unwrap()))
}

pub fn read_input_by_lines() -> impl Iterator<Item=String> {
    read_by_lines("../input")
}

pub fn get_line_count() -> usize {
    let file = std::fs::File::open("../real_input").expect("Couldn't find file");
    let reader = BufReader::new(file);

    reader.lines().count()
}


pub fn convert_bits_to_integer(iter: &mut dyn Iterator<Item=i32>) -> i32 {
    iter.reduce(|acc, item| (acc * 2 + item)).unwrap()
}

pub fn convert_bit_slice_to_integer(slice: &[i32]) -> i32 {
    slice.iter().copied().reduce(|acc, item| (acc * 2 + item)).unwrap()
}