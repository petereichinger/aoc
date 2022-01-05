use itertools::Itertools;
use utils::{read_input_by_lines};


fn main() {
    let positions: Vec<i32> = read_input_by_lines().next().unwrap().split(',').map(|str_pos| str_pos.parse().unwrap()).collect();


    if let Some((&first, &last)) = positions.iter().minmax().into_option() {
        let mut minimum_constant = usize::MAX;
        let mut minimum_increasing = usize::MAX;
        for target in first..=last {
            let fuel_constant = positions.iter().map(|pos| (pos - target).abs() as usize).sum();
            let fuel_increasing = positions.iter().map(|pos| (pos - target).abs() as usize)
                .map(|dist| (dist * (dist + 1)) / 2).sum();
            if fuel_constant < minimum_constant {
                minimum_constant = fuel_constant;
            }

            if fuel_increasing < minimum_increasing {
                minimum_increasing = fuel_increasing
            }
        }
        println!("Minimum w/ constant {}", minimum_constant);
        println!("Minimum w/ increasing {}", minimum_increasing);
    }
}