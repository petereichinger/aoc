use core::num;
use std::collections::{HashMap, HashSet};

use itertools::Itertools;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Node {
    rate: u32,
    neighbours: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Network {
    pub nodes: HashMap<String, Node>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Entry(String, u32);

impl Ord for Entry {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.1.partial_cmp(&other.1).unwrap()
    }
}
impl PartialOrd for Entry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.1.partial_cmp(&other.1)
    }
}

impl From<&str> for Network {
    fn from(input: &str) -> Self {
        let nodes = input
            .lines()
            .map(|line| {
                let (name, rest) = line
                    .strip_prefix("Valve ")
                    .unwrap()
                    .split_once(' ')
                    .unwrap();

                let rest = rest.strip_prefix("has flow rate=").unwrap();

                let (rate, neighbours) = rest
                    .split_once("; tunnels lead to valves ")
                    .unwrap_or_else(|| rest.split_once("; tunnel leads to valve ").unwrap());

                let name = name.into();
                let rate = rate.parse().unwrap();

                let neighbours = neighbours.split(", ").map(String::from).collect();

                (name, Node { rate, neighbours })
            })
            .collect();

        Network { nodes }
    }
}

type PathsType = HashMap<String, HashMap<String, u32>>;

impl Network {
    fn shortest_paths(&self, origin: String) -> HashMap<String, Option<String>> {
        let mut distances = vec![];
        let mut previous = HashMap::new();
        let mut q = HashSet::new();
        for (name, _) in &self.nodes {
            let distance = if name == &origin { 0 } else { u32::MAX };
            distances.push(Entry(name.clone(), distance));
            previous.insert(name.clone(), None::<String>);
            q.insert(name.clone());
        }

        distances.sort();

        while !q.is_empty() {
            let u = distances.iter().find(|e| q.contains(&e.0)).unwrap().clone();

            q.remove(&u.0);

            let neighbours = &self.nodes.get(&u.0).unwrap().neighbours;
            for neighbour in neighbours {
                if q.contains(neighbour) {
                    let neighbour_index = distances.iter().position(|e| &e.0 == neighbour).unwrap();
                    let mut neighbour_entry = distances.get_mut(neighbour_index).unwrap();
                    let new_dist = u.1 + 1;
                    if new_dist < neighbour_entry.1 {
                        neighbour_entry.1 = new_dist;
                        *previous.get_mut(neighbour).unwrap() = Some(u.0.clone());
                        distances.sort();
                    }
                }
            }
        }

        previous
    }

    pub fn get_paths_to_valves_from(&self, origin: String) -> Vec<Vec<String>> {
        let previous = self.shortest_paths(origin.clone());

        self.nodes
            .iter()
            .filter_map(|(name, node)| {
                if node.rate == 0 {
                    return None;
                }

                let mut path = vec![name.clone()];
                let mut current = name;

                while let Some(prev) = previous.get(current).unwrap() {
                    path.push(prev.clone());
                    current = prev;
                }
                path.reverse();
                Some(path)
            })
            .collect()
    }

    fn recurse(
        &self,
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

            let new_released = released + new_remaining_time * self.nodes.get(valve).unwrap().rate;
            let mut new_remaining = remaining_valves.clone();
            new_remaining.remove(valve);

            let new_value = self.recurse(
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

    pub fn find_optimal_order(&self, start: String) -> u32 {
        let valve_nodes: HashSet<_> = self
            .nodes
            .iter()
            .filter_map(|(name, node)| {
                if node.rate == 0 {
                    None
                } else {
                    Some(name.clone())
                }
            })
            .collect();
        let start_vec = vec![start.clone()];
        let paths: PathsType = valve_nodes
            .iter()
            .chain(start_vec.iter())
            .map(|node| {
                let paths = self.get_paths_to_valves_from((*node).clone());
                let paths: HashMap<_, _> = paths
                    .into_iter()
                    .map(|entry| (entry[entry.len() - 1].clone(), entry.len() as u32))
                    .collect();
                ((*node).clone(), paths)
            })
            .collect();

        self.recurse(&paths, "AA".into(), 0, valve_nodes, 30)
    }
}

#[cfg(test)]
mod tests {
    use super::Network;

    const TEST: &str = include_str!("test");
    #[test]
    fn test_output() {
        let network = Network::from(TEST);

        println!("{:#?}", network);
    }

    #[test]
    fn test_paths() {
        let network = Network::from(TEST);

        let max_release = network.find_optimal_order("AA".into());

        println!("{}", max_release);
    }
}
