use std::collections::{BinaryHeap, HashMap};
use utils::read_input_by_lines;

fn main() {
    let valids = HashMap::from([('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')]);

    let error_points = HashMap::from([(')', 3), (']', 57), ('}', 1197), ('>', 25137)]);

    let incomplete_points = HashMap::from([(')', 1), (']', 2), ('}', 3), ('>', 4)]);

    let mut incomplete_scores = BinaryHeap::new();

    println!("Part 1");

    let mut error_sum = 0;
    'line: for line in read_input_by_lines() {
        let mut chunk_stack = vec![];
        for current in line.chars() {
            if let Some(matching) = valids.get(&current) {
                // Opening chunk -> Add to stack
                chunk_stack.push(matching.clone());
            } else {
                // Closing chunk -> pop and verify stack
                let last = chunk_stack.pop().unwrap();

                if last != current {
                    error_sum += error_points.get(&current).unwrap();
                    continue 'line;
                }
            }
        }

        let mut incomplete_score = 0u128;

        while !chunk_stack.is_empty() {
            let closing = chunk_stack.pop().unwrap();
            incomplete_score *= 5;
            incomplete_score += incomplete_points.get(&closing).unwrap();
        }

        incomplete_scores.push(incomplete_score);
    }

    let incomplete_scores = incomplete_scores.into_sorted_vec();

    let incomplete_score = incomplete_scores[incomplete_scores.len() / 2];

    println!("Error Sum {} Incomplete Sum {}", error_sum, incomplete_score);
}