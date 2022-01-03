use std::fmt::{Display, Formatter};
use itertools::Itertools;
use utils::read_input_by_lines;


struct Bingo {
    numbers: [Option<u32>; 25],
}

impl Bingo {
    fn from_row_strings(rows: [String; 5]) -> Bingo {
        let mut numbers = [None; 25];

        for (row_idx, row) in rows.iter().enumerate() {
            row.split_whitespace().enumerate().for_each(|(col_idx, number)| {
                numbers[row_idx * 5 + col_idx] = Some(number.parse::<u32>().unwrap());
            });
        }

        Bingo { numbers }
    }


    fn sum(&self) -> u32 {
        self.numbers.iter().map(|entry| entry.unwrap_or(0)).sum()
    }

    fn call(&mut self, call: u32) -> bool {
        if let Some((idx, value)) = self.numbers.iter_mut().find_position(|v| **v == Some(call)) {
            *value = None;

            let row = idx / 5;
            let col = idx % 5;

            let row_bingoed = self.numbers[row * 5..(row + 1) * 5].iter().all(|entry| *entry == None);

            let col_bingoed = self.numbers.iter().enumerate().filter_map(|(idx, entry)| {
                if idx % 5 == col {
                    Some(entry)
                } else {
                    None
                }
            }).all(|entry| {
                entry.is_none()
            });

            row_bingoed || col_bingoed
        } else {
            false
        }
    }
}

fn write_val(val: &Option<u32>, f: &mut Formatter<'_>) -> std::fmt::Result {
    if let Some(value) = val {
        write!(f, "{:>2} ", value)
    } else {
        write!(f, "__ ")
    }
}

impl Display for Bingo {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in self.numbers.chunks(5) {
            row.iter().for_each(|val| { let _ = write_val(val, f); });
            println!();
        }

        Result::Ok(())
    }
}

fn main() {
    let mut lines = read_input_by_lines();

    let numbers = lines.next().unwrap();

    let calls: Vec<u32> = numbers.split(',').map(|val| val.parse::<u32>().unwrap()).collect();

    let mut bingos = Vec::new();

    while let Some(_) = lines.next() {
        let numbers = [lines.next().unwrap(), lines.next().unwrap(), lines.next().unwrap(), lines.next().unwrap(), lines.next().unwrap()];

        let bingo = Bingo::from_row_strings(numbers);

        bingos.push(bingo);
    }

    'calls: for call in calls {
        for bingo in bingos.iter_mut() {
            let bingoed = bingo.call(call);

            if bingoed {
                println!("BINGO with {}", call);
                println!("{}", bingo);

                println!("{} {} {}", call, bingo.sum(), call * bingo.sum());
                break 'calls;
            }
        }
    }
}