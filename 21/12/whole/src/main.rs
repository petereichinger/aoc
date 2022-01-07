use std::collections::{HashMap, HashSet};
use utils::read_input_by_lines;

type MapType = HashMap<String, Vec<String>>;

struct Graph {
    graph: MapType,
}

impl Graph {
    fn new(graph: MapType) -> Graph {
        Graph { graph }
    }
}

#[derive(Debug, Clone, Default)]
struct Path {
    nodes: Vec<String>,
    visited: HashSet<String>,
    double_visited: Option<String>,
}

impl Path {
    fn new() -> Path {
        Path {
            nodes: vec!["start".to_owned()],
            visited: HashSet::from(["start".to_owned()]),
            ..Default::default()
        }
    }

    fn can_enter(&self, allow_duplicate: bool, node: &str) -> bool {
        node != "start" && (!self.visited.contains(node) || (allow_duplicate && self.double_visited.is_none()))
    }

    fn add_node(&self, new_node: String) -> Path {
        let mut new_path = self.clone();

        if new_node.chars().all(char::is_lowercase) {
            let already_visited = new_path.visited.insert(new_node.clone());
            if !already_visited {
                new_path.double_visited = Some(new_node.clone())
            }
        }
        new_path.nodes.push(new_node);

        new_path
    }
}


fn dfs(graph: &Graph) {
    let mut stack = vec![];

    stack.push(Path::new());

    let mut path_count = 0;

    while !stack.is_empty() {
        let path = stack.pop().unwrap();

        let current = path.nodes.last().unwrap();

        if current == "end" {
            // dbg!(&path);
            path_count += 1;
        } else {
            for neighbour in graph.graph.get(current.as_str()).unwrap().iter()
                .filter(|neigh| path.can_enter(true, neigh.as_str())) {
                stack.push(path.add_node(neighbour.clone()))
            }
        }

        if path_count % 100 == 0 {
            println!("{}", path_count);
        }
    }

    println!("{}", path_count);
}

fn main() {
    let mut map_fwd: MapType = HashMap::new();

    for line in read_input_by_lines() {
        let (left, right) = line.split_once('-').unwrap();

        let key = left.to_owned();
        let value = right.to_owned();

        map_fwd.entry(key.clone()).or_insert(vec![]).push(value.clone());

        map_fwd.entry(value).or_insert(vec![]).push(key);
    }

    println!("{:?}", map_fwd);

    let graph = Graph::new(map_fwd);

    dfs(&graph);
}

