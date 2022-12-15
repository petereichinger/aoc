mod input;
mod monkey;

#[cfg(test)]
mod test_input;

use std::cell::RefCell;

use monkey::Monkey;

fn play_round(monkeys: &mut Vec<RefCell<Monkey>>, reduction: &dyn Fn(i128) -> i128) {
    for idx in 0..monkeys.len() {
        let mut curr_monkey = monkeys[idx].borrow_mut();
        curr_monkey.items.iter().for_each(|item| {
            let new_worry = (reduction)((curr_monkey.operation)(*item));

            let target_index = if new_worry % curr_monkey.test == 0 {
                curr_monkey.targets.0
            } else {
                curr_monkey.targets.1
            };

            let mut target_monkey = monkeys[target_index].borrow_mut();

            target_monkey.items.push(new_worry);
        });

        curr_monkey.inspect_count += curr_monkey.items.len();
        curr_monkey.items.clear();
    }
}

fn play_n_rounds(n: usize, monkeys: &mut Vec<RefCell<Monkey>>, reduction: &dyn Fn(i128) -> i128) {
    for _ in 0..n {
        play_round(monkeys, reduction);
    }
}
fn main() {
    part1();
    part2();
}
fn part1() {
    println!("Part 1");
    let mut monkeys = input::get_monkeys();

    play_n_rounds(20, &mut monkeys, &|x| x / 3);

    monkeys.sort_unstable_by_key(|m| -(m.borrow().inspect_count as isize));
    for monkey in &monkeys {
        let monkey = monkey.borrow();

        println!("{} {}", monkey.id, monkey.inspect_count);
    }

    println!(
        "{} {} {}",
        monkeys[0].borrow().inspect_count,
        monkeys[1].borrow().inspect_count,
        monkeys[0].borrow().inspect_count * monkeys[1].borrow().inspect_count
    )
}

fn part2() {
    println!("Part 2");
    let mut monkeys = input::get_monkeys();

    let lcm = monkeys
        .iter()
        .map(|m| m.borrow().test)
        .fold(1, |acc, t| acc * t);
    play_n_rounds(10000, &mut monkeys, &|x| x % lcm);

    monkeys.sort_unstable_by_key(|m| -(m.borrow().inspect_count as isize));
    for monkey in &monkeys {
        let monkey = monkey.borrow();

        println!("{} {}", monkey.id, monkey.inspect_count);
    }

    println!(
        "{} {} {}",
        monkeys[0].borrow().inspect_count,
        monkeys[1].borrow().inspect_count,
        monkeys[0].borrow().inspect_count * monkeys[1].borrow().inspect_count
    )
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;

    use super::{monkey::Monkey, test_input::get_monkeys};

    fn play_round(monkeys: &mut Vec<RefCell<Monkey>>) {
        super::play_round(monkeys, &|red| red / 3);
    }

    fn play_n_rounds(n: usize, monkeys: &mut Vec<RefCell<Monkey>>) {
        super::play_n_rounds(n, monkeys, &|red| red / 3);
    }

    #[test]
    fn check_creation() {
        let monkeys = get_monkeys();

        assert_eq!(monkeys.len(), 4)
    }

    #[test]
    fn play_round_doesnt_crash() {
        let mut monkeys = get_monkeys();

        play_round(&mut monkeys);
    }

    #[test]
    fn round_1_works_correctly() {
        let mut monkeys = get_monkeys();

        play_n_rounds(1, &mut monkeys);

        assert_eq!(monkeys[0].borrow().items, vec![20, 23, 27, 26]);
        assert_eq!(
            monkeys[1].borrow().items,
            vec![2080, 25, 167, 207, 401, 1046]
        );
        assert_eq!(monkeys[2].borrow().items, vec![]);
        assert_eq!(monkeys[3].borrow().items, vec![]);
    }

    #[test]
    fn round_2_works_correctly() {
        let mut monkeys = get_monkeys();

        play_n_rounds(2, &mut monkeys);

        assert_eq!(monkeys[0].borrow().items, vec![695, 10, 71, 135, 350]);
        assert_eq!(monkeys[1].borrow().items, vec![43, 49, 58, 55, 362]);
        assert_eq!(monkeys[2].borrow().items, vec![]);
        assert_eq!(monkeys[3].borrow().items, vec![]);
    }
    #[test]
    fn round_3_works_correctly() {
        let mut monkeys = get_monkeys();

        play_n_rounds(3, &mut monkeys);

        assert_eq!(monkeys[0].borrow().items, vec![16, 18, 21, 20, 122]);
        assert_eq!(monkeys[1].borrow().items, vec![1468, 22, 150, 286, 739]);
        assert_eq!(monkeys[2].borrow().items, vec![]);
        assert_eq!(monkeys[3].borrow().items, vec![]);
    }
    #[test]
    fn round_20_works_correctly() {
        let mut monkeys = get_monkeys();

        play_n_rounds(20, &mut monkeys);

        assert_eq!(monkeys[0].borrow().items, vec![10, 12, 14, 26, 34]);
        assert_eq!(monkeys[1].borrow().items, vec![245, 93, 53, 199, 115]);
        assert_eq!(monkeys[2].borrow().items, vec![]);
        assert_eq!(monkeys[3].borrow().items, vec![]);
    }

    #[test]
    fn verify_inspect_counts_after_round_20() {
        let mut monkeys = get_monkeys();

        play_n_rounds(20, &mut monkeys);

        assert_eq!(monkeys[0].borrow().inspect_count, 101);
        assert_eq!(monkeys[1].borrow().inspect_count, 95);
        assert_eq!(monkeys[2].borrow().inspect_count, 7);
        assert_eq!(monkeys[3].borrow().inspect_count, 105);
    }
}
