use std::collections::HashMap;
use itertools::Itertools;
use utils::read_input_by_lines;

const DEFAULT_CAP: usize = 100_000_000_000;

fn main() {
    let mut lines = read_input_by_lines();

    let mut last = String::with_capacity(DEFAULT_CAP);
    last.extend(lines.next().unwrap().chars());

    lines.next();

    let mappings: HashMap<_, _> = lines.map(|l| {
        let (old, new) = l.split_once(" -> ").unwrap();
        (old.to_owned(), new.to_owned())
    }).collect();


    let mut new = String::with_capacity(1000000000);

    for iter in 0..40 {
        let first = last.chars().nth(0).unwrap();

        new.clear();

        new.push(first);

        for pair in last.as_bytes().windows(2).map(|pair| String::from_utf8(Vec::from(pair)).unwrap()) {
            let new_element = mappings.get(pair.as_str()).unwrap();
            new.push(new_element.chars().nth(0).unwrap());
            new.push(pair.chars().nth(1).unwrap());
        }

        println!("{}", iter);
        std::mem::swap(&mut new, &mut last);
    }

    let mut element_counter = HashMap::new();

    for element in last.chars() {
        *element_counter.entry(element).or_insert(0) += 1u128;
    }

    let ((_, &min), (_, &max)) = element_counter.iter().minmax_by(
        |(fc, fcnt), (sc, scnt)| fcnt.cmp(scnt)
    ).into_option().unwrap();


    println!("{} {} {}", min, max, max - min);
}