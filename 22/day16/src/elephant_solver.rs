use std::{collections::HashSet, io::stdin, thread::current};

use itertools::Itertools;

use crate::network::{Network, PathsType};

#[derive(Debug, Clone)]
enum Task {
    Nothing(String),
    Moving(String, u32),
}

type Tasks = (Task, Task);

fn advance_task(task: &Task, time: u32) -> Task {
    match task {
        Task::Moving(pos, remain) if time < *remain => Task::Moving(pos.clone(), remain - time),
        Task::Moving(pos, remain) if time == *remain => Task::Nothing(pos.clone()),
        _ => panic!("cannot advance task {:?} by {}", task, time),
    }
}
fn recurse_elephant(
    network: &Network,
    paths: &PathsType,
    remaining_valves: HashSet<String>,
    remaining_time: u32,
    current_tasks: Tasks,
) -> u32 {
    let result = match &current_tasks {
        (Task::Nothing(pos_me), Task::Nothing(pos_ele)) => {
            let new_release =
                remaining_time * (network.nodes[pos_me].rate + network.nodes[pos_ele].rate);
            let mut max_release = 0;
            for perms in remaining_valves.iter().permutations(2) {
                let target_me = perms[0];
                let target_ele = perms[1];
                let distance_me = paths.get(pos_me).unwrap().get(target_me).unwrap();
                let distance_ele = paths.get(pos_ele).unwrap().get(target_ele).unwrap();

                if distance_me.min(distance_ele) >= &remaining_time {
                    continue;
                }

                let new_tasks = (
                    Task::Moving(target_me.clone(), *distance_me),
                    Task::Moving(target_ele.clone(), *distance_ele),
                );

                let mut new_remaining = remaining_valves.clone();
                new_remaining.remove(target_me);
                new_remaining.remove(target_ele);

                let released_recursive =
                    recurse_elephant(network, paths, new_remaining, remaining_time, new_tasks);

                max_release = max_release.max(released_recursive);
            }
            new_release + max_release
        }
        (Task::Nothing(pos), te @ Task::Moving(_, _)) => {
            let new_release = remaining_time * network.nodes[pos].rate;
            let mut max_release = 0;
            for valve in &remaining_valves {
                let mut new_remaining = remaining_valves.clone();
                new_remaining.remove(valve);
                let distance = paths.get(pos).unwrap().get(valve).unwrap();

                if &remaining_time <= distance {
                    continue;
                }

                let new_tasks = (Task::Moving(valve.clone(), *distance), te.clone());

                let released_recursive =
                    recurse_elephant(network, paths, new_remaining, remaining_time, new_tasks);

                max_release = max_release.max(released_recursive);
            }
            new_release + max_release
        }
        (tm @ Task::Moving(_, _), Task::Nothing(pos)) => {
            let new_release = remaining_time * network.nodes[pos].rate;
            let mut max_release = 0;
            for valve in &remaining_valves {
                let mut new_remaining = remaining_valves.clone();
                new_remaining.remove(valve);
                let distance = paths.get(pos).unwrap().get(valve).unwrap();
                if &remaining_time <= distance {
                    continue;
                }
                let new_tasks = (tm.clone(), Task::Moving(valve.clone(), *distance));
                let released_recursive =
                    recurse_elephant(network, paths, new_remaining, remaining_time, new_tasks);

                max_release = max_release.max(released_recursive);
            }
            new_release + max_release
        }
        (t1 @ Task::Moving(_, x), t2 @ Task::Moving(_, y)) => {
            let min = x.min(y);
            let new_tasks = (advance_task(t1, *min), advance_task(t2, *min));
            let remaining_time = remaining_time.saturating_sub(*min);

            if remaining_time == 0 {
                return 0;
            }

            recurse_elephant(network, paths, remaining_valves, remaining_time, new_tasks)
        }
    };

    result
}

pub fn find_elephant_order(network: &Network, start: String) -> u32 {
    let (remaining_valves, paths) = network.get_valve_nodes_and_paths(&start);

    recurse_elephant(
        network,
        &paths,
        remaining_valves,
        26,
        (Task::Nothing("AA".into()), Task::Nothing("AA".into())),
    )
}

#[cfg(test)]
mod tests {
    use crate::{elephant_solver::find_elephant_order, network::Network};

    #[test]
    fn test_pathing() {
        const TEST: &str = include_str!("test");
        let network = Network::from(TEST);

        let released = find_elephant_order(&network, "AA".into());

        assert_eq!(released, 1707);
    }
}
