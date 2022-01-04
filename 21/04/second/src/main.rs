use retain_mut::RetainMut;
use utils::{bingo::Bingo, read_input_by_lines};

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

    for call in calls {
        if bingos.len() != 1 {
            bingos.retain_mut(|bingo| !bingo.call(call));
        } else {
            let mut bingo = &mut bingos[0];

            if !bingo.call(call) {
                continue;
            }

            println!("BINGO with {}", call);
            println!("{}", bingo);
            println!("{} {} {}", call, bingo.sum(), call * bingo.sum());
            break;
        }
    }
}