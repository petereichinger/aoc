const INPUT: &str = include_str!("input1");

fn main() {
    part1();
    part2();
}

fn part2() {
    let mut total = 0;
    for line in INPUT.lines() {
        let mut chars = line.chars();

        let (enemy, _space, outcome) = (
            chars.next().unwrap(),
            chars.next().unwrap(),
            chars.next().unwrap(),
        );

        let score = match outcome {
            'X' => {
                // Lose
                0 + match enemy {
                    'A' => 3,
                    'B' => 1,
                    'C' => 2,
                    _ => 0,
                }
            }
            'Y' => {
                // Draw
                3 + match enemy {
                    'A' => 1,
                    'B' => 2,
                    'C' => 3,
                    _ => 0,
                }
            }
            'Z' => {
                // Win
                6 + match enemy {
                    'A' => 2,
                    'B' => 3,
                    'C' => 1,
                    _ => 0,
                }
            }
            _ => 0,
        };
        total += score;
    }
    println!("Part 2 {}", total);
}

fn part1() {
    let mut total = 0;
    for line in INPUT.lines() {
        let mut chars = line.chars();

        let (enemy, _space, mine) = (
            chars.next().unwrap(),
            chars.next().unwrap(),
            chars.next().unwrap(),
        );

        let score = match mine {
            'X' => {
                // Rock
                1 + match enemy {
                    'A' => 3,
                    'B' => 0,
                    'C' => 6,
                    _ => 0,
                }
            }
            'Y' => {
                // Paper
                2 + match enemy {
                    'A' => 6,
                    'B' => 3,
                    'C' => 0,
                    _ => 0,
                }
            }
            'Z' => {
                // Scissors
                3 + match enemy {
                    'A' => 0,
                    'B' => 6,
                    'C' => 3,
                    _ => 0,
                }
            }
            _ => 0,
        };
        total += score;
    }
    println!("Part 1 {}", total);
}
