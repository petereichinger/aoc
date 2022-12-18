use network::Network;

use crate::{elephant_solver::find_elephant_order, solo_solver::find_solo_order};

mod elephant_solver;
mod network;
mod solo_solver;

const INPUT: &str = include_str!("input");
fn main() {
    let network = Network::from(INPUT);

    let max = find_solo_order(&network, "AA".into());

    println!("solo: {max}");

    let max = find_elephant_order(&network, "AA".into());

    println!("ele: {max}");
}
