use std::cell::RefCell;

const INPUT: &str = include_str!("input");

fn get_stacks() -> Vec<RefCell<Vec<char>>> {
    let mut stacks = vec![];

    stacks.push(RefCell::new(vec!['L', 'C', 'G', 'M', 'Q']));
    stacks.push(RefCell::new(vec!['G', 'H', 'F', 'T', 'C', 'L', 'D', 'R']));
    stacks.push(RefCell::new(vec!['R', 'W', 'T', 'M', 'N', 'F', 'J', 'V']));
    stacks.push(RefCell::new(vec!['P', 'Q', 'V', 'D', 'F', 'J']));
    stacks.push(RefCell::new(vec!['T', 'B', 'L', 'S', 'M', 'F', 'N']));
    stacks.push(RefCell::new(vec!['P', 'D', 'C', 'H', 'V', 'N', 'R']));
    stacks.push(RefCell::new(vec!['T', 'C', 'H']));
    stacks.push(RefCell::new(vec!['P', 'H', 'N', 'Z', 'V', 'J', 'S', 'G']));
    stacks.push(RefCell::new(vec!['G', 'H', 'F', 'Z']));

    for stack in stacks.iter_mut() {
        stack.get_mut().reverse();
    }

    stacks
}
fn main() {
    let stacks = get_stacks();
    let stacks_2 = stacks.clone();
    for op in INPUT.lines() {
        let mut tokens = op.split_whitespace();

        tokens.next();
        let count = tokens.next().unwrap().parse::<usize>().unwrap();

        tokens.next();
        let origin_idx = tokens.next().unwrap().parse::<usize>().unwrap() - 1;

        tokens.next();
        let target_idx = tokens.next().unwrap().parse::<usize>().unwrap() - 1;

        let mut origin = stacks.get(origin_idx).unwrap().borrow_mut();
        let mut target = stacks.get(target_idx).unwrap().borrow_mut();

        for _ in 0..count {
            let c = origin.pop().unwrap();
            target.push(c);
        }

        let mut origin = stacks_2.get(origin_idx).unwrap().borrow_mut();
        let mut target = stacks_2.get(target_idx).unwrap().borrow_mut();

        let new_origin_len = origin.len() - count;

        let (_, copy) = origin.split_at(new_origin_len);

        copy.iter().for_each(|c| target.push(*c));
        origin.resize(new_origin_len, 'A');
    }

    for stack in stacks {
        let stack = stack.borrow();

        let top = stack[stack.len() - 1];
        print!("{top}")
    }
    println!("");

    for stack in stacks_2 {
        let stack = stack.borrow();

        let top = stack[stack.len() - 1];
        print!("{top}")
    }
    println!("");
}
