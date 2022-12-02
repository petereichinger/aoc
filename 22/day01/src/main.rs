use std::collections::HashMap;

const INPUT_1: &str = include_str!("input1.txt");

fn main() {
    let mut elf_map = HashMap::new();
    let (mut max_idx, mut max_cals) = (0, 0);
    let (mut idx, mut cals) = (0, 0);
    for line in INPUT_1.lines() {
        match line {
            "" => {
                elf_map.insert(idx, cals);
                if cals > max_cals {
                    (max_idx, max_cals) = (idx, cals);
                }
                (idx, cals) = (idx + 1, 0)
            }
            cal => {
                let cal = cal.parse::<i32>().unwrap();
                cals += cal;
            }
        }
    }

    println!("{} {}", max_idx, max_cals);

    let mut elf_vec: Vec<(i32, i32)> = elf_map
        .iter()
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect();

    elf_vec.sort_by(|(_, ca), (_, cb)| ca.cmp(cb).reverse());

    let sum = elf_vec.iter().take(3).map(|(_, c)| c).sum::<i32>();

    println!("{}", sum);
}
