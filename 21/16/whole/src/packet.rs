use itertools::Itertools;
use utils::read_by_lines;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Header {
    version: u8,
    type_id: u8,
}

impl Header {
    fn new(version: u8, type_id: u8) -> Self {
        Header { version, type_id }
    }
}


#[derive(Clone, Debug, PartialEq)]
pub enum Packet {
    Literal(Header, i32),
    Operator(Header, Vec<Box<Packet>>),
}

impl Packet {
    fn parse_literal_value(bits: &str) -> (i32, &str) {
        let mut number = 0;
        let mut digits_parsed = 0;
        for mut element in &bits.chars().chunks(5) {
            let continue_flag = element.next().unwrap();
            let digit_string: String = element.collect();
            let digit = i32::from_str_radix(digit_string.as_str(), 2).unwrap();
            number *= 16;
            number += digit;
            digits_parsed += 1;
            if continue_flag == '0' {
                break;
            }
        }

        (number, &bits[digits_parsed * 5..])
    }

    fn parse(bits: &str) -> Self {
        let (version, bits) = bits.split_at(3);

        let (type_id, bits) = bits.split_at(3);

        let version = u8::from_str_radix(version, 2).unwrap();
        let type_id = u8::from_str_radix(type_id, 2).unwrap();
        let header = Header::new(version, type_id);


        match type_id {
            4 => {
                let (number, _) = Packet::parse_literal_value(bits);
                Packet::Literal(header, number)
            }
            _ => {
                Packet::Operator(header, vec![])
            }
        }
    }
}

impl From<&str> for Packet {
    fn from(hex_string: &str) -> Self {
        let bits: String = hex_string.chars().map(|c| format!("{:04b}", c.to_digit(16).unwrap())).collect();

        Packet::parse(bits.as_str())
    }
}

#[cfg(test)]
mod tests {
    use crate::packet::{Header, Packet};

    #[test]
    fn parse_literal_value() {
        let (number,remainder) = Packet::parse_literal_value("101111111000101000");
        assert_eq!(number, 2021);
        assert_eq!(remainder, "000");
    }

    #[test]
    fn parse_literal_works() {
        let literal = Packet::from(include_str!("../../literal"));

        assert_eq!(literal, Packet::Literal(Header::new(6, 4), 2021))
    }
}
