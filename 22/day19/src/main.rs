use std::thread;

use simulation::Simulation;

mod blueprint;
mod simulation;

const INPUT: &str = include_str!("input");

fn main() {
    let result = quality_sum(INPUT, 24, u8::MAX);

    println!("part 1 {result}");

    let result = quality_prod(INPUT, 32, 3);

    println!("part 2 {result}")
}

fn quality_sum(input: &str, simulation_duration: u8, limit: u8) -> u32 {
    let mut handles = vec![];
    for line in input.lines().take(limit.into()) {
        let simulation = Simulation::for_blueprint(line.into());

        let result = thread::spawn(move || {
            let result = simulation.simulate(simulation_duration) as u32;

            simulation.blueprint.id as u32 * result
        });

        handles.push(result);
    }

    let mut quality_sum = 0;

    for handle in handles {
        quality_sum += handle.join().unwrap();
    }

    quality_sum
}

fn quality_prod(input: &str, simulation_duration: u8, limit: u8) -> u32 {
    let mut handles = vec![];
    for line in input.lines().take(limit.into()) {
        let simulation = Simulation::for_blueprint(line.into());

        let result = thread::spawn(move || {
            let result = simulation.simulate(simulation_duration) as u32;

            result
        });

        handles.push(result);
    }

    let mut quality_prod = 1;

    for handle in handles {
        quality_prod *= handle.join().unwrap();
    }

    quality_prod
}
