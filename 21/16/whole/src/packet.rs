#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Type {
    None,
    Sum,
    Product,
    Minimum,
    Maximum,
    Literal,
    GreaterThan,
    LessThan,
    EqualTo,
}

impl Type {
    fn eval(&self, mut operands: impl Iterator<Item=u128>) -> u128 {
        match self {
            Type::Sum => { operands.sum() }
            Type::Product => { operands.reduce(|f, s| f * s).unwrap() }
            Type::Minimum => { operands.min().unwrap() }
            Type::Maximum => { operands.max().unwrap() }
            Type::GreaterThan | Type::LessThan | Type::EqualTo => {
                let (f, s) = (operands.next().unwrap(), operands.next().unwrap());

                match self {
                    Type::GreaterThan => { if f > s { 1 } else { 0 } }
                    Type::LessThan => { if f < s { 1 } else { 0 } }
                    Type::EqualTo => { if f == s { 1 } else { 0 } }
                    _ => panic!("NOT HAPPENING")
                }
            }
            _ => panic!("Invalid operator for eval")
        }
    }
}

impl Default for Type {
    fn default() -> Self {
        Type::None
    }
}

impl From<u8> for Type {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Sum,
            1 => Self::Product,
            2 => Self::Minimum,
            3 => Self::Maximum,
            4 => Self::Literal,
            5 => Self::GreaterThan,
            6 => Self::LessThan,
            7 => Self::EqualTo,
            _ => Self::None
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Header {
    version: u8,
    type_id: Type,
}

impl Header {
    fn new(version: u8, type_id: Type) -> Self {
        Header { version, type_id }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Value(u128);

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

            let digit = u128::from_str_radix(digit, 2).unwrap();
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
        let header = Header::new(version, Type::from(type_id));

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

    pub fn sum_versions(&self) -> u32 {
        match self {
            Packet::Literal(header, _) => { header.version as u32 }
            Packet::Operator(header, _, operands) => {
                (header.version as u32) + operands.0.iter().map(|op| op.sum_versions()).sum::<u32>()
            }
        }
    }

    pub fn eval(&self) -> u128 {
        match self {
            Packet::Literal(_, Value(val)) => { *val as u128 }
            Packet::Operator(header, _, Operands(ops)) => {
                let evaluated_operands = ops.iter().map(|op| op.eval());
                header.type_id.eval(evaluated_operands)
            }
        }
    }

    fn parse(bits: &str) -> (Self, &str) {
        let (header, bits) = Packet::parse_header(bits);

        match header.type_id {
            Type::Literal => {
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
    use crate::packet::{Header, LengthType, Operands, Packet, Value, Type};

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

        let expected_operands = Operands(vec![Box::new(Packet::Literal(Header { version: 6, type_id: Type::Literal }, Value(10))),
                                              Box::new(Packet::Literal(Header { version: 2, type_id: Type::Literal }, Value(20))),
        ]);

        assert_eq!(operator,
                   Packet::Operator(Header { version: 1, type_id: Type::from(6) },
                                    LengthType::Bits(27),
                                    expected_operands));
    }

    #[test]
    fn parse_operator_count() {
        let operator = Packet::from("EE00D40C823060");

        let expected_header = Header::new(7, Type::from(3));
        let expected_operands = vec![
            Box::new(Packet::Literal(Header::new(2, Type::from(4)), Value(1))),
            Box::new(Packet::Literal(Header::new(4, Type::from(4)), Value(2))),
            Box::new(Packet::Literal(Header::new(1, Type::from(4)), Value(3))),
        ];
        let expected = Packet::Operator(expected_header, LengthType::Count(3), Operands(expected_operands));

        assert_eq!(operator, expected);
    }

    #[test]
    fn parse_literal_works() {
        let literal = Packet::from("D2FE28");

        assert_eq!(literal, Packet::Literal(Header::new(6, Type::from(4)), Value(2021)))
    }

    #[test]
    fn parse_recursive_operator() {
        let packet = Packet::from("8A004A801A8002F478");

        let expected_operator =
            Packet::Operator(Header::new(4, Type::from(2)), LengthType::Count(1), Operands(vec![
                Box::new(Packet::Operator(Header::new(1, Type::from(2)), LengthType::Count(1), Operands(vec![
                    Box::new(Packet::Operator(Header::new(5, Type::from(2)), LengthType::Bits(11), Operands(vec![
                        Box::new(Packet::Literal(Header::new(6, Type::from(4)), Value(15)))
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

    #[test]
    fn eval() {
        assert_eq!(Packet::from("C200B40A82").eval(), 3);
        assert_eq!(Packet::from("04005AC33890").eval(), 54);
        assert_eq!(Packet::from("880086C3E88112").eval(), 7);
        assert_eq!(Packet::from("CE00C43D881120").eval(), 9);
        assert_eq!(Packet::from("D8005AC2A8F0").eval(), 1);
        assert_eq!(Packet::from("F600BC2D8F").eval(), 0);
        assert_eq!(Packet::from("9C005AC2F8F0").eval(), 0);
        assert_eq!(Packet::from("9C0141080250320F1802104A08").eval(), 1);
    }
}
