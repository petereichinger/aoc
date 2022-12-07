use itertools::Itertools;

const INPUT: &str = include_str!("input");

fn main() {
    println!("{}", find_unique_slice(INPUT, 4));
    println!("{}", find_unique_slice(INPUT, 14));
}

fn find_unique_slice(input: &str, len: usize) -> usize {
    for end in len..input.len() {
        let start = end - len;
        let window = &input[start..end];

        let uniques = window.chars().unique().count();

        if uniques == len {
            return end;
        }
    }

    return input.len();
}
