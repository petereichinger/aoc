const INPUT: &str = include_str!("input");

struct Range {
    min: i32,
    max: i32,
}

impl Range {
    fn contains(&self, other: &Self) -> bool {
        self.min <= other.min && self.max >= other.max
    }

    fn overlap(&self, other: &Self) -> bool {
        let (_min1, min2) = min_max(self.min, other.min);
        let (max1, _max2) = min_max(self.max, other.max);

        max1 >= min2
    }
}

fn min_max(a: i32, b: i32) -> (i32, i32) {
    if a < b {
        (a, b)
    } else {
        (b, a)
    }
}
fn get_range_from_str(input: &str) -> Range {
    let (str_min, str_max) = input.split_once('-').unwrap();

    let min = str_min.parse().unwrap();
    let max = str_max.parse().unwrap();

    Range { min, max }
}

fn main() {
    let containing_rows = INPUT
        .lines()
        .filter(|&l| {
            let (f, s) = l.split_once(',').unwrap();
            let f_range = get_range_from_str(f);
            let s_range = get_range_from_str(s);

            f_range.contains(&s_range) || s_range.contains(&f_range)
        })
        .count();

    println!("containing {containing_rows}");

    let overlapping_rows = INPUT
        .lines()
        .filter(|&l| {
            let (f, s) = l.split_once(',').unwrap();
            let f_range = get_range_from_str(f);
            let s_range = get_range_from_str(s);

            f_range.overlap(&s_range)
        })
        .count();

    println!("{overlapping_rows}");
}
