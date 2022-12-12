enum Instruction {
    NoOp,
    Add(i32),
}

fn get_instructions_from_input(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|l| {
            let mut split = l.split_whitespace();

            let instr = split.next().unwrap();

            match instr {
                "noop" => Instruction::NoOp,
                "addx" => {
                    let num = split.next().unwrap().parse::<i32>().unwrap();

                    Instruction::Add(num)
                }
                _ => panic!("Invalid instruction {}", instr),
            }
        })
        .collect()
}

fn get_signal_strength(instructions: &Vec<Instruction>, cycle: i32) -> i32 {
    let (mut curr_cycle, mut curr_x) = (0, 1);
    for instruction in instructions {
        let next_state = match instruction {
            Instruction::NoOp => (curr_cycle + 1, curr_x),
            Instruction::Add(change) => (curr_cycle + 2, curr_x + change),
        };

        if next_state.0 >= cycle {
            return cycle * curr_x;
        }

        (curr_cycle, curr_x) = (next_state.0, next_state.1);
    }

    panic!()
}

fn read_display(instructions: &Vec<Instruction>) -> Vec<Vec<char>> {
    let mut x_states = vec![1];
    let (mut curr_cycle, mut curr_x) = (0, 1);
    for instruction in instructions {
        let next_state = match instruction {
            Instruction::NoOp => {
                x_states.push(curr_x);
                (curr_cycle + 1, curr_x)
            }
            Instruction::Add(change) => {
                x_states.push(curr_x);
                x_states.push(curr_x + change);
                (curr_cycle + 2, curr_x + change)
            }
        };

        (curr_cycle, curr_x) = (next_state.0, next_state.1);
    }

    (0..6)
        .map(|row| {
            (0..40)
                .map(|col| {
                    let index = row * 40 + col;
                    let x = x_states[index as usize];

                    if (x - col).abs() < 2 {
                        '#'
                    } else {
                        '.'
                    }
                })
                .collect::<Vec<char>>()
        })
        .collect::<Vec<_>>()
}

fn main() {
    const INPUT: &str = include_str!("input");

    let instructions = get_instructions_from_input(INPUT);

    let part1 = (0..6)
        .map(|x| 20 + x * 40)
        .map(|cycle| get_signal_strength(&instructions, cycle))
        .sum::<i32>();

    println!("{part1}");

    let display = read_display(&instructions);

    for row in display {
        let row_str = row.iter().collect::<String>();

        println!("{row_str}");
    }
}

#[cfg(test)]
mod tests {
    const TEST: &str = include_str!("test");

    use std::fs::ReadDir;

    use super::*;

    #[test]
    fn step_20() {
        let instructions = get_instructions_from_input(TEST);
        assert_eq!(420, get_signal_strength(&instructions, 20))
    }
    #[test]
    fn step_60() {
        let instructions = get_instructions_from_input(TEST);

        assert_eq!(1140, get_signal_strength(&instructions, 60))
    }
    #[test]
    fn step_100() {
        let instructions = get_instructions_from_input(TEST);

        assert_eq!(1800, get_signal_strength(&instructions, 100))
    }
    #[test]
    fn step_140() {
        let instructions = get_instructions_from_input(TEST);

        assert_eq!(2940, get_signal_strength(&instructions, 140))
    }
    #[test]
    fn step_180() {
        let instructions = get_instructions_from_input(TEST);

        assert_eq!(2880, get_signal_strength(&instructions, 180))
    }
    #[test]
    fn step_220() {
        let instructions = get_instructions_from_input(TEST);

        assert_eq!(3960, get_signal_strength(&instructions, 220))
    }

    #[test]
    fn read_display_works() {
        let instructions = get_instructions_from_input(TEST);

        assert_eq!(
            read_display(&instructions),
            vec![
                "##..##..##..##..##..##..##..##..##..##.."
                    .chars()
                    .collect::<Vec<char>>(),
                "###...###...###...###...###...###...###."
                    .chars()
                    .collect::<Vec<char>>(),
                "####....####....####....####....####...."
                    .chars()
                    .collect::<Vec<char>>(),
                "#####.....#####.....#####.....#####....."
                    .chars()
                    .collect::<Vec<char>>(),
                "######......######......######......####"
                    .chars()
                    .collect::<Vec<char>>(),
                "#######.......#######.......#######....."
                    .chars()
                    .collect::<Vec<char>>(),
            ]
        )
    }
}
