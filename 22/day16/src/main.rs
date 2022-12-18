use network::Network;

mod network;

const INPUT: &str = include_str!("input");
fn main() {
    let network = Network::from(INPUT);

    let max = network.find_optimal_order("AA".into());

    println!("{max}");
}
