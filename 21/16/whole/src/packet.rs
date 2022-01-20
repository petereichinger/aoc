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

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Value(i32);

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LengthType {
    Bits(usize),
    Count(usize),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Operands(Vec<Box<Packet>>);

#[derive(Clone, Debug, PartialEq)]
pub enum Packet {
    Literal(Header, Value),
    Operator(Header, LengthType, Operands),
}

impl Packet {
    fn parse_literal_value(mut bits: &str) -> (Value, &str) {
        let mut number = 0;

        loop {
            let continue_flag = &bits[0..1];
            let digit = &bits[1..5];

            let digit = i32::from_str_radix(digit, 2).unwrap();
            number *= 16;
            number += digit;

            bits = &bits[5..];
            if continue_flag == "0" {
                break;
            }
        }

        (Value(number), bits)
    }
    fn parse_literal(header: Header, bits: &str) -> (Packet, &str) {
        let (number, remainder) = Packet::parse_literal_value(bits);
        let literal = Packet::Literal(header, number);

        (literal, remainder)
    }
    fn parse_header(bits: &str) -> (Header, &str) {
        let (version, bits) = bits.split_at(3);

        let (type_id, bits) = bits.split_at(3);

        let version = u8::from_str_radix(version, 2).unwrap();
        let type_id = u8::from_str_radix(type_id, 2).unwrap();
        let header = Header::new(version, type_id);

        (header, bits)
    }
    fn parse_operands_bits(length: usize, bits: &str) -> (Operands, &str) {
        let (mut operands_bits, remainder) = bits.split_at(length);

        let mut packets = vec![];
        while operands_bits != "" {
            let (packet, remaining_operands) = Packet::parse(operands_bits);

            packets.push(Box::new(packet));
            operands_bits = remaining_operands;
        }

        (Operands(packets), remainder)
    }
    fn parse_operands_count(count: usize, bits: &str) -> (Operands, &str) {
        let mut remainder = bits;

        let mut operands = vec![];

        for _ in 0..count {
            let (packet, new_remainder) = Self::parse(remainder);
            operands.push(Box::new(packet));
            remainder = new_remainder;
        }

        (Operands(operands), remainder)
    }
    fn parse_operator(header: Header, bits: &str) -> (Packet, &str) {
        let (length_type, bits) = bits.split_at(1);

        let (length_type, bits) = match u8::from_str_radix(length_type, 2) {
            Ok(0) => {
                let (length_bits, bits) = bits.split_at(15);
                (LengthType::Bits(usize::from_str_radix(length_bits, 2).unwrap()), bits)
            }
            Ok(1) => {
                let (length_count, bits) = bits.split_at(11);
                (LengthType::Count(usize::from_str_radix(length_count, 2).unwrap()), bits)
            }
            _ => panic!("invalid length type {:?}", length_type)
        };


        let (operands, bits) = match length_type {
            LengthType::Bits(length) => {
                Self::parse_operands_bits(length, bits)
            }
            LengthType::Count(count) => {
                Self::parse_operands_count(count, bits)
            }
        };

        let operator = Packet::Operator(header, length_type, operands);

        (operator, bits)
    }

    fn sum_versions(&self) -> u32 {
        match self {
            Packet::Literal(header, _) => { header.version as u32 }
            Packet::Operator(header, _, operands) => {
                (header.version as u32) + operands.0.iter().map(|op| op.sum_versions()).sum::<u32>()
            }
        }
    }

    fn parse(bits: &str) -> (Self, &str) {
        let (header, bits) = Packet::parse_header(bits);

        match header.type_id {
            4 => {
                Packet::parse_literal(header, bits)
            }
            _ => {
                Packet::parse_operator(header, bits)
            }
        }
    }
}

impl From<&str> for Packet {
    fn from(hex_string: &str) -> Self {
        let bits: String = hex_string.chars().map(|c| format!("{:04b}", c.to_digit(16).unwrap())).collect();

        Packet::parse(bits.as_str()).0
    }
}

#[cfg(test)]
mod tests {
    use crate::packet::{Header, LengthType, Operands, Packet, Value};

    #[test]
    fn parse_literal_value() {
        let (number, remainder) = Packet::parse_literal_value("101111111000101000");
        assert_eq!(number, Value(2021));
        assert_eq!(remainder, "000");
    }


    #[test]
    fn parse_complex_literal_value() {
        let result = Packet::parse_literal_value("010100101001000100100");

        assert_eq!(result, (Value(10), "0101001000100100"));
    }

    #[test]
    fn parse_literal_value_with_remainder() {
        let (number, remainder) = Packet::parse_literal_value("10111111100010100000");
        assert_eq!(number, Value(2021));
        assert_eq!(remainder, "00000");
    }

    #[test]
    fn parse_operator_bits() {
        let operator = Packet::from("38006F45291200");

        let expected_operands = Operands(vec![Box::new(Packet::Literal(Header { version: 6, type_id: 4 }, Value(10))),
                                              Box::new(Packet::Literal(Header { version: 2, type_id: 4 }, Value(20))),
        ]);

        assert_eq!(operator,
                   Packet::Operator(Header { version: 1, type_id: 6 },
                                    LengthType::Bits(27),
                                    expected_operands));
    }

    #[test]
    fn parse_operator_count() {
        let operator = Packet::from("EE00D40C823060");

        let expected_header = Header::new(7, 3);
        let expected_operands = vec![
            Box::new(Packet::Literal(Header::new(2, 4), Value(1))),
            Box::new(Packet::Literal(Header::new(4, 4), Value(2))),
            Box::new(Packet::Literal(Header::new(1, 4), Value(3))),
        ];
        let expected = Packet::Operator(expected_header, LengthType::Count(3), Operands(expected_operands));

        assert_eq!(operator, expected);
    }

    #[test]
    fn parse_literal_works() {
        let literal = Packet::from(include_str!("../../literal"));

        assert_eq!(literal, Packet::Literal(Header::new(6, 4), Value(2021)))
    }

    #[test]
    fn parse_recursive_operator() {
        let packet = Packet::from("8A004A801A8002F478");

        let expected_operator =
            Packet::Operator(Header::new(4, 2), LengthType::Count(1), Operands(vec![
                Box::new(Packet::Operator(Header::new(1, 2), LengthType::Count(1), Operands(vec![
                    Box::new(Packet::Operator(Header::new(5, 2), LengthType::Bits(11), Operands(vec![
                        Box::new(Packet::Literal(Header::new(6, 4), Value(15)))
                    ])))])))]));
        assert_eq!(packet, expected_operator);
    }

    #[test]
    fn version_sum() {
        assert_eq!(Packet::from("8A004A801A8002F478").sum_versions(), 16);
        assert_eq!(Packet::from("620080001611562C8802118E34").sum_versions(), 12);
        assert_eq!(Packet::from("C0015000016115A2E0802F182340").sum_versions(), 23);
        assert_eq!(Packet::from("A0016C880162017C3686B18A3D4780").sum_versions(), 31);
    }
}
