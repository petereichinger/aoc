use utils::read_input_by_lines;

fn main() {
    let mut x = 0;
    let mut y = 0;
    let mut aim = 0;
    for (dist, aim_delta) in read_input_by_lines()
        .filter_map(|l| {
            let mut split_line = l.split_whitespace();

            Some((split_line.next().unwrap().to_string(),
                  split_line.next().unwrap().parse::<i32>().unwrap()))
        }
        ).map(
        |(instr, dist)| {
            match instr.as_str() {
                "forward" => (dist, 0),
                "up" => (0, -dist),
                "down" => (0, dist),
                _ => panic!("FUCK")
            }
        }
    ) {
        aim += aim_delta;
        x += dist;
        y += aim * dist;
    }

    println!("{}", x * y);
    //.for_each(|(instr, dist)| println!("{}", instr));
}
