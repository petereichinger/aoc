use std::thread;

use simulation::Simulation;

mod blueprint;
mod simulation;

const TEST: &str = include_str!("test");

fn main() {
    let mut handles = vec![];
    for line in TEST.lines() {
        let simulation = Simulation::for_blueprint(line.into());

        let result = thread::spawn(move || {
            let result = simulation.simulate();

            simulation.blueprint.id * result
        });

        handles.push(result);
    }

    let mut quality_sum = 0;

    for handle in handles {
        quality_sum += handle.join().unwrap();
    }

    println!("{}", quality_sum);
}
