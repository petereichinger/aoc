use std::{collections::HashMap, fmt::Display};

use utils_22::Coord;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
struct Elf {
    current: Coord,
    next: Option<Coord>,
}

struct ElvesMap {
    map: HashMap<Coord, Elf>,
}

struct Proposal {
    movement: Coord,
    checks: [Coord; 3],
}

impl ElvesMap {
    fn from_text(input: &str) -> Self {
        let mut map = HashMap::new();

        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if c != '#' {
                    continue;
                }

                let x = x as i32;
                let y = -(y as i32);
                let position = Coord::new(x, y);
                map.insert(
                    position,
                    Elf {
                        current: position,
                        next: None,
                    },
                );
            }
        }

        Self { map }
    }

    const PROPOSALS: [Proposal; 4] = [
        Proposal {
            movement: Coord::UP,
            checks: [Coord::UP_LEFT, Coord::UP, Coord::UP_RIGHT],
        },
        Proposal {
            movement: Coord::DOWN,
            checks: [Coord::DOWN_LEFT, Coord::DOWN, Coord::DOWN_RIGHT],
        },
        Proposal {
            movement: Coord::LEFT,
            checks: [Coord::UP_LEFT, Coord::LEFT, Coord::DOWN_LEFT],
        },
        Proposal {
            movement: Coord::RIGHT,
            checks: [Coord::UP_RIGHT, Coord::RIGHT, Coord::DOWN_RIGHT],
        },
    ];

    fn plan_moves(self, proposals_offset: usize) -> Self {
        let mut result = HashMap::new();
        'elf: for (coord, elf) in &self.map {
            let any_neighbour = Coord::NEIGHBOURS
                .iter()
                .map(|n| coord + n)
                .any(|n| self.map.contains_key(&n));

            if any_neighbour {
                for proposal_index in 0..4 {
                    let proposal = &Self::PROPOSALS[(proposal_index + proposals_offset) % 4];
                    let empty = proposal
                        .checks
                        .iter()
                        .map(|c| coord + c)
                        .all(|n| !self.map.contains_key(&n));

                    if empty {
                        result.insert(
                            coord.clone(),
                            Elf {
                                current: *coord,
                                next: Some(coord + &proposal.movement),
                            },
                        );
                        continue 'elf;
                    }
                }
            }
            result.insert(*coord, *elf);
        }

        Self { map: result }
    }

    fn execute_move(self) -> Self {
        let mut nexts = HashMap::new();

        for (coord, elf) in self.map {
            let entry = match elf.next {
                Some(next) => nexts.entry(next).or_insert(vec![]),
                None => nexts.entry(coord).or_insert(vec![]),
            };
            entry.push(elf);
        }

        let mut elves_map = HashMap::new();

        for (coord, elves) in nexts {
            if elves.len() > 1 {
                for elf in elves {
                    elves_map.insert(
                        elf.current,
                        Elf {
                            current: elf.current,
                            next: None,
                        },
                    );
                }
            } else {
                elves_map.insert(
                    coord,
                    Elf {
                        current: coord,
                        next: None,
                    },
                );
            }
        }

        Self { map: elves_map }
    }

    fn run_n_rounds(mut self, n: usize) -> Self {
        for round in 0..n {
            self = self.plan_moves(round).execute_move()
        }

        self
    }

    fn run_to_completion(mut self) -> (Self, usize) {
        for round in 0.. {
            self = self.plan_moves(round);

            let finished = self.map.iter().all(|(_, e)| e.next.is_none());

            if finished {
                return (self, round + 1);
            }

            self = self.execute_move()
        }

        panic!("FUCK!!!");
    }

    fn get_min_max_coordinates(&self) -> (Coord, Coord) {
        let mut min = Coord::new(i32::MAX, i32::MAX);
        let mut max = Coord::new(i32::MIN, i32::MIN);

        for (coord, _elf) in &self.map {
            min = min.min(coord);
            max = max.max(coord);
        }

        (min, max)
    }

    fn count_empty_tiles(&self) -> usize {
        let (min, max) = self.get_min_max_coordinates();
        let mut counter = 0;
        for y in min.y()..=max.y() {
            for x in min.x()..=max.x() {
                let coord = Coord::new(x, y);

                if !self.map.contains_key(&coord) {
                    counter += 1;
                }
            }
        }

        counter
    }
}

impl Display for ElvesMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (min, max) = self.get_min_max_coordinates();

        for y in (min.y()..=max.y()).rev() {
            for x in min.x()..=max.x() {
                let char_to_print = match self.map.get(&Coord::new(x, y)) {
                    Some(_) => '#',
                    None => '.',
                };

                write!(f, "{char_to_print}")?
            }
            writeln!(f)?
        }

        Ok(())
    }
}

fn main() {
    const INPUT: &str = include_str!("input");

    let map = ElvesMap::from_text(INPUT).run_n_rounds(10);

    let empty_count = map.count_empty_tiles();

    println!("Part 1 {}", empty_count);

    let (_map, round) = ElvesMap::from_text(INPUT).run_to_completion();

    println!("Part 2 {}", round);
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = include_str!("test");
    const MINI_TEST: &str = ".....
..##.
..#..
.....
..##.
.....";

    #[test]
    fn parse_works() {
        let elves = ElvesMap::from_text(TEST);

        assert_eq!(elves.map.len(), 22);
    }

    #[test]
    fn planning_is_correct() {
        let elves_map = ElvesMap::from_text(MINI_TEST);

        let plans = elves_map.plan_moves(0);

        assert_eq!(
            plans.map[&Coord::new(2, -1)],
            Elf {
                current: Coord::new(2, -1),
                next: Some(Coord::new(2, 0))
            }
        );
        assert_eq!(
            plans.map[&Coord::new(3, -1)],
            Elf {
                current: Coord::new(3, -1),
                next: Some(Coord::new(3, 0))
            }
        );
    }

    #[test]
    fn execute_works() {
        let elves_map = ElvesMap::from_text(MINI_TEST);

        let elves_map = elves_map.run_n_rounds(1);

        assert!(elves_map.map.contains_key(&Coord::new(2, 0)));
        assert!(elves_map.map.contains_key(&Coord::new(3, 0)));
        assert!(elves_map.map.contains_key(&Coord::new(2, -2)));
        assert!(elves_map.map.contains_key(&Coord::new(3, -3)));
        assert!(elves_map.map.contains_key(&Coord::new(2, -4)));
    }
    #[test]
    fn execute_works_two_steps() {
        let elves_map = ElvesMap::from_text(MINI_TEST);

        let elves_map = elves_map.run_n_rounds(2);

        assert!(elves_map.map.contains_key(&Coord::new(2, -5)));
        assert!(elves_map.map.contains_key(&Coord::new(2, -1)));
        assert!(elves_map.map.contains_key(&Coord::new(3, -1)));
        assert!(elves_map.map.contains_key(&Coord::new(4, -3)));

        assert!(elves_map.map.contains_key(&Coord::new(1, -2)));
    }

    #[test]
    fn execute_works_three_steps() {
        let elves_map = ElvesMap::from_text(MINI_TEST);

        let elves_map = elves_map.run_n_rounds(3);

        assert!(elves_map.map.contains_key(&Coord::new(2, 0)));
        assert!(elves_map.map.contains_key(&Coord::new(4, -1)));
        assert!(elves_map.map.contains_key(&Coord::new(0, -2)));
        assert!(elves_map.map.contains_key(&Coord::new(4, -3)));
        assert!(elves_map.map.contains_key(&Coord::new(2, -5)));
    }

    #[test]
    fn test_full_case() -> std::fmt::Result {
        use std::fmt::Write;

        let elves_map = ElvesMap::from_text(TEST);

        let elves_map = elves_map.run_n_rounds(10);

        let mut test = String::new();

        write!(test, "{}", elves_map)?;

        assert_eq!(
            test,
            "......#.....
..........#.
.#.#..#.....
.....#......
..#.....#..#
#......##...
....##......
.#........#.
...#.#..#...
............
...#..#..#..
"
        );
        Ok(())
    }
    #[test]
    fn test_empty_count() {
        let map = ElvesMap::from_text(TEST).run_n_rounds(10);

        assert_eq!(map.count_empty_tiles(), 110);
    }

    #[test]
    fn test_run_to_completion() {
        let (_map, round) = ElvesMap::from_text(TEST).run_to_completion();

        assert_eq!(round, 20);
    }
}
