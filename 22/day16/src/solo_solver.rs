use std::collections::HashSet;

use crate::network::{Network, PathsType};

fn recurse_solo(
    network: &Network,
    paths: &PathsType,
    current: String,
    released: u32,
    remaining_valves: HashSet<String>,
    remaining_time: u32,
) -> u32 {
    let mut max = released;
    for valve in &remaining_valves {
        let new_remaining_time =
            remaining_time.saturating_sub(*paths.get(&current).unwrap().get(valve).unwrap());

        if new_remaining_time == 0 {
            continue;
        }

        let new_released = released + new_remaining_time * network.nodes.get(valve).unwrap().rate;
        let mut new_remaining = remaining_valves.clone();
        new_remaining.remove(valve);

        let new_value = recurse_solo(
            network,
            paths,
            valve.clone(),
            new_released,
            new_remaining,
            new_remaining_time,
        );
        max = max.max(new_value);
    }

    max
}

pub fn find_solo_order(network: &Network, start: String) -> u32 {
    let (remaining_valves, paths) = network.get_valve_nodes_and_paths(&start);
    recurse_solo(network, &paths, start, 0, remaining_valves, 30)
}

#[cfg(test)]
mod tests {
    use crate::{network::Network, solo_solver::find_solo_order};

    #[test]
    fn test_paths() {
        const TEST: &str = include_str!("test");
        let network = Network::from(TEST);

        let max_release = find_solo_order(&network, "AA".into());

        assert_eq!(max_release, 1651);
    }
}
