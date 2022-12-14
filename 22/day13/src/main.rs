mod packet;

use std::cmp::Ordering;

use packet::Packet;

fn main() {
    const INPUT: &str = include_str!("input");

    let sum_part1 = INPUT
        .split("\n\n")
        .enumerate()
        .filter_map(|(idx, pair)| {
            let (f, s) = pair.split_once('\n').unwrap();

            let (f, s) = (Packet::from(f), Packet::from(s));

            if f.cmp(&s) == Ordering::Less {
                Some(idx + 1)
            } else {
                None
            }
        })
        .sum::<usize>();

    println!("part 1: {}", sum_part1);

    let mut packets: Vec<Packet> = INPUT
        .lines()
        .filter_map(|line| {
            if line.is_empty() {
                return None;
            }
            return Some(Packet::from(line));
        })
        .collect();

    let first = Packet::from("[[2]]");
    let second = Packet::from("[[6]]");
    packets.push(first.clone());
    packets.push(second.clone());

    packets.sort();

    let first_index = packets.iter().position(|packet| packet == &first).unwrap() + 1;
    let second_index = packets.iter().position(|packet| packet == &second).unwrap() + 1;

    println!("second part {}", first_index * second_index);
}
