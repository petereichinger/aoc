fn snafu_to_i64(snafu: &str) -> i64 {
    let mut sum = 0;

    let mut power = 1;

    for digit in snafu.chars().rev() {
        let factor = match digit {
            '0' => 0,
            '1' => 1,
            '2' => 2,
            '-' => -1,
            '=' => -2,
            c => panic!("invalid char {}", c),
        };

        sum += factor * power;

        power *= 5;
    }

    sum
}

fn i64_to_snafu(integer: i64) -> String {
    if integer == 0 {
        return "0".into();
    }

    let mut digits = vec![];
    let mut number = integer;

    while number != 0 {
        let digit = number / 5;
        let remainder = number % 5;
        digits.push(remainder);

        number = digit;
    }

    digits.push(0);
    digits.reverse();

    for idx in (1..digits.len()).rev() {
        while digits[idx] > 2 {
            digits[idx - 1] += 1;
            digits[idx] -= 5;
        }
    }

    digits
        .iter()
        .skip_while(|d| **d == 0)
        .map(|d| {
            let c = match d {
                0 => '0',
                1 => '1',
                2 => '2',
                -1 => '-',
                -2 => '=',
                x => panic!("invalid value {}", x),
            };

            c
        })
        .collect()
}

fn get_snafu_sum(input: &str) -> String {
    let sum = input.lines().map(snafu_to_i64).sum::<i64>();

    let sum_snafu = i64_to_snafu(sum);

    sum_snafu
}

fn main() {
    let snafu_sum = get_snafu_sum(include_str!("input"));

    println!("Part1: '{}'", snafu_sum);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_snafu_to_integer() {
        assert_eq!(snafu_to_i64("1=-0-2"), 1747);
        assert_eq!(snafu_to_i64("12111"), 906);
        assert_eq!(snafu_to_i64("2=0="), 198);
        assert_eq!(snafu_to_i64("21"), 11);
        assert_eq!(snafu_to_i64("2=01"), 201);
        assert_eq!(snafu_to_i64("111"), 31);
        assert_eq!(snafu_to_i64("20012"), 1257);
        assert_eq!(snafu_to_i64("112"), 32);
        assert_eq!(snafu_to_i64("1=-1="), 353);
        assert_eq!(snafu_to_i64("1-12"), 107);
        assert_eq!(snafu_to_i64("12"), 7);
        assert_eq!(snafu_to_i64("1="), 3);
        assert_eq!(snafu_to_i64("122"), 37);
    }

    #[test]
    fn test_integer_to_snafu() {
        assert_eq!(i64_to_snafu(15), String::from("1=0"));
        assert_eq!(i64_to_snafu(1), String::from("1"));
        assert_eq!(i64_to_snafu(2), String::from("2"));
        assert_eq!(i64_to_snafu(3), String::from("1="));
        assert_eq!(i64_to_snafu(4), String::from("1-"));
        assert_eq!(i64_to_snafu(5), String::from("10"));
        assert_eq!(i64_to_snafu(6), String::from("11"));
        assert_eq!(i64_to_snafu(7), String::from("12"));
        assert_eq!(i64_to_snafu(8), String::from("2="));
        assert_eq!(i64_to_snafu(9), String::from("2-"));
        assert_eq!(i64_to_snafu(10), String::from("20"));
        assert_eq!(i64_to_snafu(20), String::from("1-0"));
        assert_eq!(i64_to_snafu(2022), String::from("1=11-2"));
        assert_eq!(i64_to_snafu(12345), String::from("1-0---0"));
        assert_eq!(i64_to_snafu(314159265), String::from("1121-1110-1=0"));
    }

    #[test]
    fn test_function() {
        let snafu = i64_to_snafu(4890);

        println!("{}", snafu);
    }

    #[test]
    fn test_sample() {
        let sum = get_snafu_sum(include_str!("test"));

        assert_eq!(String::from("2=-1=0"), sum);
    }
}
