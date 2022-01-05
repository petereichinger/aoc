use std::collections::HashMap;
use itertools::Itertools;
use utils::{read_input_by_lines};

#[derive(Clone)]
struct Entry {
    sequence: Vec<String>,
    digits: Vec<String>,
}

fn get_digit_by_len(digits: &mut Vec<String>, len: usize) -> String {
    let pos = digits.iter().position(|e| e.len() == len).unwrap();
    digits.swap_remove(pos)
}

fn digit_superset_of_other(digit: &String, other: &String) -> bool {
    other.chars().all(|c| {
        digit.contains(c)
    })
}

fn get_digit_by_subset(digits: &mut Vec<String>, subset: &String) -> String {
    let pos = digits.iter().position(|digit| digit_superset_of_other(digit, subset)).unwrap();
    digits.swap_remove(pos)
}

fn digit_contains_other_partially(digit: &String, other: &String, num_elements: usize) -> bool {
    let contains = other.chars().filter(|c| digit.contains(*c)).count();

    contains == num_elements
}

fn get_digit_by_partial_contains(digits: &mut Vec<String>, subset: &String, num_elements: usize) -> String {
    let pos = digits.iter().position(|digit| digit_contains_other_partially(digit, subset, num_elements)).unwrap();
    digits.swap_remove(pos)
}

fn insert_digit(map: &mut HashMap<String, i32>, mut digit: String, value: i32) {
    let sorted: String = digit.chars().sorted().collect();
    map.insert(sorted, value);
}

fn main() {
    let mut entries = vec![];

    for line in read_input_by_lines() {
        let (seq_str, digits_str) = line.split_once('|')
            .map(|(seq, digits)| (seq.to_owned(), digits.to_owned())).unwrap();

        let entry = Entry {
            sequence: seq_str.split_whitespace().map(|s| s.to_owned()).collect(),
            digits: digits_str.split_whitespace().map(|s| s.to_owned()).collect(),
        };

        entries.push(entry);
    }

    println!("Part 1");

    let p1 = entries.clone();

    let count = p1.into_iter().map(|e| e.digits).concat().into_iter()
        .filter(|digit| {
            match digit.len()
            {
                2 | 3 | 4 | 7 => true,
                _ => false
            }
        }).count();

    println!("{}", count);


    println!("Part 2");

    let mut p2 = entries.clone();

    let mut sum = 0;

    for entry in &mut p2 {
        // find segments for 1,4,7,8

        let digit1 = get_digit_by_len(&mut entry.sequence, 2);
        let digit4 = get_digit_by_len(&mut entry.sequence, 4);
        let digit7 = get_digit_by_len(&mut entry.sequence, 3);
        let digit8 = get_digit_by_len(&mut entry.sequence, 7);

        assert_eq!(entry.sequence.len(), 6);

        let (mut len5, mut len6): (Vec<String>, Vec<String>) = entry.sequence.iter().cloned().partition(|a| a.len() == 5);

        let digit9 = get_digit_by_subset(&mut len6, &digit4);
        assert_eq!(len6.len(), 2);
        let digit0 = get_digit_by_subset(&mut len6, &digit1);
        let digit6 = len6.pop().unwrap();
        assert!(len6.is_empty());

        let digit3 = get_digit_by_subset(&mut len5, &digit1);
        assert_eq!(len5.len(), 2);
        let digit5 = get_digit_by_partial_contains(&mut len5, &digit4, 3);
        let digit2 = len5.pop().unwrap();
        assert!(len5.is_empty());

        let mut map: HashMap<String, i32> = HashMap::new();
        insert_digit(&mut map, digit1, 1);
        insert_digit(&mut map, digit2, 2);
        insert_digit(&mut map, digit3, 3);
        insert_digit(&mut map, digit4, 4);
        insert_digit(&mut map, digit5, 5);
        insert_digit(&mut map, digit6, 6);
        insert_digit(&mut map, digit7, 7);
        insert_digit(&mut map, digit8, 8);
        insert_digit(&mut map, digit9, 9);
        insert_digit(&mut map, digit0, 0);

        let mut number = 0;

        for digit in &entry.digits {
            number *= 10;

            let sorted_digit: String = digit.chars().sorted().collect();

            number += map.get(sorted_digit.as_str()).unwrap();
        }

        sum += number;
    }

    println!("{}", sum);
}

