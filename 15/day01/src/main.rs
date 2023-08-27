const part1: &str = include_str!("input");

fn main() {
    println!("{}", go_to_floor(part1));
    println!("{}", find_basement(part1));
}

fn instruction_map(instruction: char) -> i32 {
    match instruction {
        '(' => 1,
        ')' => -1,
        _ => 0,
    }
}

fn go_to_floor(instructions: &str) -> i32 {
    instructions.chars().map(instruction_map).sum()
}

fn find_basement(instructions: &str) -> usize {
    let mut floor = 0;
    for (index, change) in instructions.chars().map(instruction_map).enumerate() {
        floor += change;
        if floor == -1 {
            return index + 1;
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ground_test() {
        assert_eq!(go_to_floor("(())"), 0);
        assert_eq!(go_to_floor("()()"), 0);
    }
    #[test]
    fn basement_1() {
        assert_eq!(go_to_floor(")))"), -3);
        assert_eq!(go_to_floor(")())())"), -3)
    }
}
