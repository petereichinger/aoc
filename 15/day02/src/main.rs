const INPUT: &str = include_str!("input");

fn main() {
    let presents: Vec<_> = INPUT.lines().map(parse_dimensions).collect();

    let wrapper_size = presents.iter().map(get_paper_size).sum::<i32>();

    println!("{}", wrapper_size);

    let ribbon_length = presents.iter().map(get_ribbon_length).sum::<i32>();

    println!("{}", ribbon_length);
}

fn parse_dimensions(present: &str) -> (i32, i32, i32) {
    let mut splits = present.split('x');

    let (first, second, third) = (
        splits.next().unwrap(),
        splits.next().unwrap(),
        splits.next().unwrap(),
    );

    (
        first.parse::<i32>().unwrap(),
        second.parse::<i32>().unwrap(),
        third.parse::<i32>().unwrap(),
    )
}

fn get_paper_size(present: &(i32, i32, i32)) -> i32 {
    let (first, second, third) = present;
    let side_one = first * second;
    let side_two = second * third;
    let side_three = first * third;

    let min = side_one.min(side_two).min(side_three);

    min + 2 * side_one + 2 * side_two + 2 * side_three
}

fn get_ribbon_length(present: &(i32, i32, i32)) -> i32 {
    let (first, second, third) = present;

    let mut slice = [first, second, third];
    slice.sort();
    let small1 = slice[0];
    let small2 = slice[1];

    2 * small1 + 2 * small2 + first * second * third
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(get_paper_size(&parse_dimensions("2x3x4")), 58);
        assert_eq!(get_paper_size(&parse_dimensions("1x1x10")), 43);
    }
}
