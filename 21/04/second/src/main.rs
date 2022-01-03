use itertools::Itertools;
use utils::{convert_bit_slice_to_integer, convert_bits_to_integer, get_line_count, read_input_by_lines};

fn find_value(mut values: Vec<Vec<i32>>, most_common: bool) -> Vec<i32> {
    for digit in 0..12 {
        let sum = values.iter().map(|vals| vals[digit] as f32).sum::<f32>();


        let filter = if most_common {
            if sum >= (values.len() as f32 / 2.0) {
                1
            } else {
                0
            }
        } else {
            if sum >= (values.len() as f32 / 2.0) {
                0
            } else {
                1
            }
        };

        println!("{} {}", sum, values.len());

        values.retain(|vals| vals[digit] == filter);


        if values.len() == 1 {
            break;
        }
    }

    assert_eq!(values.len(), 1);

    values.pop().unwrap()
}

fn main() {
    // let count = get_line_count() as i32;
    let parsed_lines = read_input_by_lines()
        .map(|l| {
            l.chars().map(|c| (c.to_digit(2).unwrap() as i32)).collect::<Vec<i32>>()
        }).collect::<Vec<_>>();

    let oxygen = find_value(parsed_lines.clone(), true);
    println!();
    let co2 = find_value(parsed_lines.clone(), false);

    let oxygen = convert_bit_slice_to_integer(oxygen.as_slice());
    let co2 = convert_bit_slice_to_integer(co2.as_slice());

    println!("{} {} {}", oxygen, co2, oxygen * co2);
}
