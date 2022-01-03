use utils::{convert_bits_to_integer, get_line_count, read_input_by_lines};

fn main() {
    let count = get_line_count() as u32;

    let sum = read_input_by_lines()
        .map(|l| {
            l.chars().map(|c| (c.to_digit(2).unwrap())).collect::<Vec<u32>>()
        })
        .reduce(|acc, item| {
            acc.iter().zip(item.iter()).map(|(a, b)| a + b).collect()
        })
        .unwrap();

    let bits = sum.iter().map(
        |digit| {
            if digit > &(count / 2) {
                1
            } else {
                0
            }
        }
    ).collect::<Vec<i32>>();

    let gamma = convert_bits_to_integer(&mut bits.iter().copied());
    let epsilon = convert_bits_to_integer(&mut bits.iter().copied().map(|digit| 1 - digit));
    println!("Gamma {} Epsilon {} Product {}", gamma, epsilon, gamma * epsilon);
}
