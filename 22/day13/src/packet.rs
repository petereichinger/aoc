#[derive(Debug, PartialEq, Eq)]
pub enum Packet {
    Integer(i32),
    List(Vec<Packet>),
}

const NUMBER_DELIMITER: &[char] = &[',', ']'];

impl<T> From<T> for Packet
where
    T: AsRef<str> + std::fmt::Debug,
{
    fn from(s: T) -> Self {
        #[cfg(test)]
        eprintln!("{s:?}");

        let s = s.as_ref();
        if s.is_empty() {
            return Packet::List(vec![]);
        }
        let first_char = s.chars().nth(0).unwrap();

        match first_char {
            '[' => {
                let list_end = s.rfind(']').unwrap();

                let inner_list = &s[1..list_end];
                if inner_list.is_empty() {
                    Packet::List(vec![])
                } else {
                    let items: Vec<Packet> =
                        s[1..list_end].split(',').map(|item| item.into()).collect();

                    Packet::List(items)
                }
            }
            '0'..='9' => {
                let number = s.parse::<i32>().unwrap();
                Packet::Integer(number)
            }
            _ => panic!(),
        }
    }
}

#[cfg(test)]
mod test {

    use super::Packet;

    #[test]
    fn empty_list_works() {
        assert_eq!(Packet::List(vec![]), "[]".into())
    }

    #[test]
    fn simple_integer_works() {
        assert_eq!(Packet::Integer(3), "3".into())
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
}
