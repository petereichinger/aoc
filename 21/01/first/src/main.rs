use std::io::{BufRead, BufReader};
use itertools::Itertools;
use utils::read_input_by_lines;

fn main() {
    let increases = read_input_by_lines()
        .filter_map(|l| { l.parse::<i32>().ok() })
        .tuple_windows().filter(|(f, s)| f < s).count();

    println!("{}", increases);
}
