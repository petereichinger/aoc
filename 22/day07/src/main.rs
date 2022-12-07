use std::collections::HashMap;

const INPUT: &str = include_str!("input");

#[derive(Debug)]
enum Entry {
    File(u32),
    Folder(HashMap<String, Entry>),
}

struct FileTree {
    cwd: Vec<String>,
    root: HashMap<String, Entry>,
}

fn cd(tree: &mut FileTree, parameter: &str) {
    match parameter {
        "/" => {
            tree.cwd.clear();
            tree.cwd.push("/".into())
        }
        ".." => {
            let _ = tree.cwd.pop();
        }
        sub_dir => tree.cwd.push(sub_dir.into()),
    }
}

fn ls(tree: &mut FileTree) -> &mut HashMap<String, Entry> {
    let mut dir = &mut tree.root;
    for cwd in &tree.cwd {
        let entry = dir
            .entry(cwd.clone())
            .or_insert(Entry::Folder(HashMap::new()));
        match entry {
            Entry::Folder(map) => dir = map,
            _ => panic!("File was inserted"),
        }
    }

    dir
}

fn command(mut tree: &mut FileTree, command: &str) {
    let mut split = command.split_whitespace();
    let _prefix = split.next();

    let command = split.next().unwrap();

    match command {
        "cd" => {
            let parameter = split.next().unwrap();
            cd(&mut tree, parameter)
        }
        "ls" => {
            let _ = ls(&mut tree);
        }
        cmd => panic!("unknown command {cmd}"),
    }
}
fn directory(_tree: &mut FileTree, _command: &str) {}
fn file_entry(mut tree: &mut FileTree, command: &str) {
    let mut split = command.split_whitespace();

    let size = split.next().unwrap().parse::<u32>().unwrap();
    let name = split.next().unwrap().to_owned();

    let dir = ls(&mut tree);

    dir.insert(name, Entry::File(size));
}

fn recursive_dir_size(
    mut sizes: &mut Vec<(String, u32)>,
    limit: u32,
    entries: &HashMap<String, Entry>,
    dir_name: &String,
) -> u32 {
    let mut sum = 0;
    for (dir_name, entry) in entries {
        match entry {
            Entry::File(size) => sum += *size,
            Entry::Folder(entries) => {
                sum += recursive_dir_size(&mut sizes, limit, &entries, dir_name)
            }
        }
    }

    if sum <= limit {
        sizes.push((dir_name.clone(), sum));
    }

    sum
}

fn main() {
    let mut file_tree = FileTree {
        cwd: vec![],
        root: HashMap::new(),
    };
    for line in INPUT.lines() {
        let first = *line.chars().peekable().peek().unwrap();

        match first {
            '$' => command(&mut file_tree, line),
            'd' => directory(&mut file_tree, line),
            '0'..='9' => file_entry(&mut file_tree, line),
            prefix => panic!("unknown prefix {prefix}"),
        }
    }

    let root_string = String::from("");
    // println!("{:#?}", file_tree.root);
    let mut sizes = vec![];
    recursive_dir_size(&mut sizes, 10000, &file_tree.root, &root_string);

    let sum = sizes.iter().map(|e| e.1).sum::<u32>();

    println!("{sum}");

    let root_size = recursive_dir_size(&mut vec![], 0, &file_tree.root, &root_string);
    // println!("{root_size}");

    let to_delete_size = 30000000 - (70000000 - root_size);

    println!("{to_delete_size}");

    let mut sizes = vec![];
    recursive_dir_size(&mut sizes, 70000000, &file_tree.root, &root_string);

    sizes.retain(|e| e.1 >= to_delete_size);
    sizes.sort_by(|f, s| f.1.cmp(&s.1));

    println!("{:?}", sizes[0]);
}
