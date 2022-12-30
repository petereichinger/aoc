use std::{cell::RefCell, collections::HashMap, io::stdin, thread};

type MonkeyName = String;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum OpType {
    Add,
    Sub,
    Mul,
    Div,
}
#[derive(Debug, Clone, PartialEq)]
enum Job {
    Literal(f64),
    Op(OpType, MonkeyName, MonkeyName),
}

#[derive(Debug, Clone, PartialEq)]
struct Monkey {
    name: MonkeyName,
    job: Job,
}

type MonkeyMap = HashMap<String, Monkey>;
type MonkeyCache = RefCell<HashMap<String, f64>>;

fn parse_monkeys(input: &str) -> Vec<Monkey> {
    input
        .lines()
        .map(|l| {
            let (name, job) = l.split_once(": ").unwrap();

            if job.chars().next().unwrap().is_numeric() {
                let literal = job.parse::<f64>().unwrap();
                Monkey {
                    name: name.into(),
                    job: Job::Literal(literal),
                }
            } else {
                let first = job[0..4].into();
                let second = job[7..11].into();

                let op = match &job[5..6] {
                    "+" => OpType::Add,
                    "-" => OpType::Sub,
                    "*" => OpType::Mul,
                    "/" => OpType::Div,
                    x => panic!("invalid operation: '{x}'"),
                };

                Monkey {
                    name: name.into(),
                    job: Job::Op(op, first, second),
                }
            }
        })
        .collect()
}

fn monkey_map(input: &str) -> HashMap<String, Monkey> {
    let monkeys = parse_monkeys(input);

    monkeys.into_iter().map(|m| (m.name.clone(), m)).collect()
}

fn search_cache_or_recurse(
    monkey_map: &MonkeyMap,
    monkey_cache: Option<&MonkeyCache>,
    monkey_name: &String,
) -> (f64, bool) {
    if let Some(cache) = monkey_cache {
        if let Some(result) = cache.borrow().get(monkey_name) {
            return (*result, false);
        }
    }
    let (number, cacheable) = recursive_function(&monkey_map, monkey_cache, monkey_name);

    if cacheable {
        if let Some(cache) = monkey_cache {
            cache.borrow_mut().insert(monkey_name.clone(), number);
        }
    }

    (number, cacheable)
}

fn recursive_function(
    monkey_map: &MonkeyMap,
    monkey_cache: Option<&MonkeyCache>,
    monkey_name: &String,
) -> (f64, bool) {
    let monkey = &monkey_map[monkey_name];

    match &monkey.job {
        Job::Literal(x) => (*x, monkey_name != "humn"),
        Job::Op(op, a, b) => {
            let (a, cache_a) = search_cache_or_recurse(&monkey_map, monkey_cache, a);
            let (b, cache_b) = search_cache_or_recurse(&monkey_map, monkey_cache, b);

            (
                match op {
                    OpType::Add => a + b,
                    OpType::Sub => a - b,
                    OpType::Mul => a * b,
                    OpType::Div => a / b,
                },
                cache_a && cache_b,
            )
        }
    }
}

fn find_monkey_number(
    monkey_map: &MonkeyMap,
    monkey_cache: Option<&MonkeyCache>,
    monkey_name: &String,
) -> f64 {
    recursive_function(&monkey_map, monkey_cache, monkey_name).0
}

fn yell_number(monkey_map: &mut MonkeyMap, value: f64) {
    let entry = monkey_map.get_mut("humn".into()).unwrap();
    entry.job = Job::Literal(value);
}

fn main() {
    const INPUT: &str = include_str!("input");

    let mut monkey_map = monkey_map(INPUT);

    let monkey_cache = RefCell::new(HashMap::new());
    let root_number = find_monkey_number(&monkey_map, Some(&monkey_cache), &"root".into());

    println!("Part 1 {root_number}");

    let root = monkey_map["root".into()].clone();

    let mut range = (3000000000000f64, 4000000000000f64);
    // first is too large, second too small
    if let Job::Op(_, a, b) = root.job {
        let result = loop {
            let middle = (range.1 + range.0) / 2.0;

            yell_number(&mut monkey_map, middle);

            let a = find_monkey_number(&monkey_map, Some(&monkey_cache), &a);
            let b = find_monkey_number(&monkey_map, Some(&monkey_cache), &b);

            let diff = a - b;

            if diff > 0.0 {
                range = (middle, range.1);
            } else if diff < 0.0 {
                range = (range.0, middle);
            } else {
                println!("Part 2 {middle}");
                break middle;
            }
        };

        // for number in (result - 1000)..(result + 1000) {
        //     yell_number(&mut monkey_map, number);
        //     let a = find_monkey_number(&monkey_map, Some(&monkey_cache), &a);
        //     let b = find_monkey_number(&monkey_map, Some(&monkey_cache), &b);

        //     if a == b {
        //         println!("Part 2 {number}");
        //     }
        // }
    } else {
        panic!("root has wrong job");
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    const TEST: &str = include_str!("test");

    #[test]
    fn parse_works() {
        let monkeys = parse_monkeys(TEST);

        assert_eq!(monkeys.len(), 15);
    }

    #[test]
    fn literal_works() {
        let monkey_map = monkey_map(TEST);

        let number = find_monkey_number(&monkey_map, None, &"dbpl".into());
        assert_eq!(number, 5);

        let number = find_monkey_number(&monkey_map, None, &"hmdt".into());
        assert_eq!(number, 32);

        let number = find_monkey_number(&monkey_map, None, &"zczc".into());
        assert_eq!(number, 2);
    }

    #[test]
    fn simple_op_works() {
        let monkey_map = monkey_map(TEST);
        let number = find_monkey_number(&monkey_map, None, &"drzm".into());

        assert_eq!(number, 30);
    }

    #[test]
    fn test_scenario_works() {
        let monkey_map = monkey_map(TEST);

        let number = find_monkey_number(&monkey_map, None, &"root".into());

        assert_eq!(number, 152);
    }

    #[test]
    fn test_part_2() {
        let mut monkey_map = monkey_map(TEST);

        let root = &monkey_map["root".into()].clone();

        if let Job::Op(_, a, b) = &root.job {
            yell_number(&mut monkey_map, 301);

            assert_eq!(
                find_monkey_number(&monkey_map, None, a),
                find_monkey_number(&monkey_map, None, b)
            );
        } else {
            panic!("root has wrong job");
        }
    }
}
