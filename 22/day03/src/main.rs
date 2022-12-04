use std::collections::HashSet;

const INPUT: &str = include_str!("input");
const ALO_U32: u32 = 'a' as u32;
const AUP_U32: u32 = 'A' as u32;
fn prio_from_item(item: char) -> u32 {
    let item_u32 = item as u32;
    match item {
        'a'..='z' => item_u32 - ALO_U32 + 1,
        'A'..='Z' => item_u32 - AUP_U32 + 27,
        _ => panic!("invalid item"),
    }
}

fn main() {
    part1();
    part2();
}

fn part1() {
    let mut sum = 0;
    for rucksack in INPUT.lines() {
        let len = rucksack.len();

        let (first, second) = rucksack.split_at(len / 2);

        let first_set: HashSet<u32> = first.chars().map(prio_from_item).collect();

        let duplicate = second
            .chars()
            .map(prio_from_item)
            .filter(|item| first_set.contains(item))
            .next()
            .unwrap();

        sum += duplicate;
        // println!("{:?}", first_set);
    }

    println!("Part 1: {sum}")
}

fn get_set_from_rucksack(rucksack: &str) -> HashSet<u32> {
    rucksack.chars().map(prio_from_item).collect()
}
fn part2() {
    let mut lines = INPUT.lines().peekable();
    let mut sum = 0;
    while lines.peek() != None {
        let f = get_set_from_rucksack(lines.next().unwrap());
        let s = get_set_from_rucksack(lines.next().unwrap());
        let t = get_set_from_rucksack(lines.next().unwrap());

        let f_s = f.intersection(&s).copied().collect::<HashSet<u32>>();

        let item = f_s.intersection(&t).take(1).next().unwrap();

        sum += item;
    }

    println!("{sum}")
}
