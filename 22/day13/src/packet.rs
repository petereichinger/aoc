const NUMBER_DELIMITER: &[char] = &[',', ']'];

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Packet {
    Integer(i32),
    List(Vec<Packet>),
}

fn parse(mut input: &str) -> (&str, Packet) {
    let mut items = Vec::new();

    while !input.is_empty() {
        let mut chars = input.chars();
        if let Some((next_char, rest)) = chars.next().map(|c| (c, chars.as_str())) {
            match next_char {
                '[' => {
                    let (rest, item) = parse(rest);
                    items.push(item);
                    input = rest;
                }
                ']' => {
                    return (&rest, Packet::List(items));
                }
                ',' => input = rest,
                x if x.is_numeric() => {
                    let number_end = input.find(NUMBER_DELIMITER).unwrap_or(input.len());
                    let number = input[..number_end].parse::<i32>().unwrap();
                    items.push(Packet::Integer(number));
                    input = &input[number_end..];
                }
                _ => panic!(),
            }
        }
    }

    assert_eq!(1, items.len());

    return ("", items.pop().unwrap());
}

impl<T> From<T> for Packet
where
    T: AsRef<str> + std::fmt::Debug,
{
    fn from(input: T) -> Self {
        let (rest, packet) = parse(input.as_ref());
        assert!(rest.is_empty());

        packet
    }
}
impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn compare_list_to_int(l: &Vec<Packet>, x: &i32) -> std::cmp::Ordering {
    let x_vec = vec![Packet::Integer(*x)];

    compare_lists(l, &x_vec)
}

fn compare_lists(l1: &Vec<Packet>, l2: &Vec<Packet>) -> std::cmp::Ordering {
    use std::cmp::Ordering;
    let end_result = if l1.len() < l2.len() {
        Ordering::Less
    } else if l2.len() < l1.len() {
        Ordering::Greater
    } else {
        Ordering::Equal
    };
    for (a, b) in l1.iter().zip(l2.iter()) {
        let cmp = a.cmp(b);
        if cmp != Ordering::Equal {
            return cmp;
        }
    }

    end_result
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self {
            Packet::Integer(x) => match other {
                Packet::Integer(y) => x.cmp(y),
                Packet::List(l) => compare_list_to_int(l, x).reverse(),
            },
            Packet::List(l) => match other {
                Packet::Integer(x) => compare_list_to_int(l, x),
                Packet::List(l2) => compare_lists(l, l2),
            },
        }
    }
}

#[cfg(test)]
mod test {

    use super::Packet;
    use std::cmp::Ordering;

    #[test]
    fn empty_list_works() {
        assert_eq!(Packet::List(vec![]), "[]".into())
    }

    #[test]
    fn integer_list_works() {
        assert_eq!(
            Packet::List(vec![
                Packet::Integer(1),
                Packet::Integer(2),
                Packet::Integer(3)
            ]),
            "[1,2,3]".into()
        )
    }

    #[test]
    fn nested_lists_work() {
        assert_eq!(Packet::List(vec![Packet::List(vec![])]), "[[]]".into())
    }

    #[test]
    fn multi_nested_lists_work() {
        assert_eq!(
            Packet::List(vec![Packet::List(vec![Packet::List(vec![])])]),
            "[[[]]]".into()
        )
    }
    #[test]
    fn two_lists_in_one() {
        assert_eq!(
            Packet::List(vec![Packet::List(vec![]), Packet::List(vec![])]),
            "[[],[]]".into()
        )
    }
    #[test]
    fn bit_more_complex() {
        assert_eq!(
            Packet::List(vec![
                Packet::List(vec![Packet::Integer(1), Packet::Integer(2)]),
                Packet::List(vec![Packet::Integer(1), Packet::Integer(1)])
            ]),
            "[[1,2],[1,1]]".into()
        )
    }

    #[test]
    fn even_more_complex() {
        assert_eq!(
            Packet::List(vec![
                Packet::List(vec![
                    Packet::Integer(1),
                    Packet::Integer(2),
                    Packet::List(vec![Packet::Integer(4)])
                ]),
                Packet::List(vec![Packet::Integer(1), Packet::Integer(1)])
            ]),
            "[[1,2,[4]],[1,1]]".into()
        )
    }

    #[test]
    fn verify_test_case_packets_work() {
        const INPUT: &str = include_str!("test");

        for line in INPUT.lines() {
            if !line.is_empty() {
                let _packet = Packet::from(line);
            }
        }
    }
    #[test]
    fn verify_input_packets_work() {
        const INPUT: &str = include_str!("input");

        for line in INPUT.lines() {
            if !line.is_empty() {
                let _packet = Packet::from(line);
            }
        }
    }

    #[test]
    fn compare_simple_ordering() {
        assert_eq!(Ordering::Equal, Packet::from("[]").cmp(&Packet::from("[]")))
    }

    #[test]
    fn order_0_1_elements() {
        assert_eq!(Ordering::Less, Packet::from("[]").cmp(&Packet::from("[1]")))
    }

    #[test]
    fn order_1_0_elements() {
        assert_eq!(
            Ordering::Greater,
            Packet::from("[1]").cmp(&Packet::from("[]"))
        )
    }

    #[test]
    fn order_number_elements() {
        assert_eq!(
            Ordering::Less,
            Packet::from("[1]").cmp(&Packet::from("[2]"))
        )
    }

    #[test]
    fn order_equal_number_elements() {
        assert_eq!(
            Ordering::Equal,
            Packet::from("[1]").cmp(&Packet::from("[1]"))
        )
    }

    #[test]
    fn test_cases() {
        let test: (Packet, Packet) = ("[1,1,3,1,1]".into(), "[1,1,5,1,1]".into());
        assert_eq!(Ordering::Less, test.0.cmp(&test.1));
        let test: (Packet, Packet) = ("[[1],[2,3,4]]".into(), "[[1],4]".into());
        assert_eq!(Ordering::Less, test.0.cmp(&test.1));
        let test: (Packet, Packet) = ("[9]".into(), "[[8,7,6]]".into());
        assert_eq!(Ordering::Greater, test.0.cmp(&test.1));
        let test: (Packet, Packet) = ("[[4,4],4,4]".into(), "[[4,4],4,4,4]".into());
        assert_eq!(Ordering::Less, test.0.cmp(&test.1));
        let test: (Packet, Packet) = ("[7,7,7,7]".into(), "[7,7,7]".into());
        assert_eq!(Ordering::Greater, test.0.cmp(&test.1));
        let test: (Packet, Packet) = ("[]".into(), "[3]".into());
        assert_eq!(Ordering::Less, test.0.cmp(&test.1));
        let test: (Packet, Packet) = ("[[[]]]".into(), "[[]]".into());
        assert_eq!(Ordering::Greater, test.0.cmp(&test.1));
        let test: (Packet, Packet) = (
            "[1,[2,[3,[4,[5,6,7]]]],8,9]".into(),
            "[1,[2,[3,[4,[5,6,0]]]],8,9]".into(),
        );
        assert_eq!(Ordering::Greater, test.0.cmp(&test.1));
    }
}
