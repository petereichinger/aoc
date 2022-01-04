use utils::{read_input_by_lines};

type Sim = [u128; 9];

fn simulate(days: i32, mut fishies: Sim) {
    for day in 1..=days {
        let mut new_fishies: Sim = [0u128; 9];
        for timer in (1..=8).rev() {
            new_fishies[timer - 1] = fishies[timer];
        }

        new_fishies[6] += fishies[0];
        new_fishies[8] += fishies[0];

        fishies = new_fishies;

        println!("After {} days: {:?}", day, fishies.iter().sum::<u128>());
    }
}

fn main() {
    let mut fishes: Sim = [0u128; 9];

    for fish in read_input_by_lines().next().unwrap().split(',').map(|str_val| str_val.parse::<usize>().unwrap()) {
        fishes[fish] += 1
    }

    let fish_map80 = fishes;
    let fish_map256: Sim = fish_map80.clone();

    simulate(80, fish_map80);
    simulate(256, fish_map256);
}