use std::fmt::{Display, Formatter};
use itertools::Itertools;

pub struct Bingo {
    numbers: [Option<u32>; 25],
}

impl Bingo {
    pub fn from_row_strings(rows: [String; 5]) -> Bingo {
        let mut numbers = [None; 25];

        for (row_idx, row) in rows.iter().enumerate() {
            row.split_whitespace().enumerate().for_each(|(col_idx, number)| {
                numbers[row_idx * 5 + col_idx] = Some(number.parse::<u32>().unwrap());
            });
        }

        Bingo { numbers }
    }


    pub fn sum(&self) -> u32 {
        self.numbers.iter().map(|entry| entry.unwrap_or(0)).sum()
    }

    pub fn call(&mut self, call: u32) -> bool {
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