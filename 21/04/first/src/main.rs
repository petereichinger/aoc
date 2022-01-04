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