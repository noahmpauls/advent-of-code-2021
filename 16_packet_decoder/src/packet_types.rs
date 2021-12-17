use crate::packet_parse::{version, type_id};

struct Header {
    version: u8,
    type_id: u8,
}

impl Header {
    fn from(bin: &str) -> Header {
        Header {
            version: version(bin),
            type_id: type_id(bin),
        }
    }
}

pub mod literal {
    use crate::packet::{Packet};
    use crate::packet_parse::{literal_value};
    use super::{Header};

    pub struct Literal {
        header: Header,
        value: u128,
    }
    
    impl Literal {
        pub fn new(binary: &str) -> Literal {
            let header = Header::from(binary);
            let value = literal_value(binary);
            let value = u128::from_str_radix(&value, 2).unwrap();
            Literal { header, value }
        }
    }
    
    impl Packet for Literal {
        fn versions(&self) -> Vec<u8> {
            vec![self.header.version]
        }

        fn values(&self) -> Vec<u128> {
            vec![self.value]
        }

        fn evaluate(&self) -> u128 {
            self.value
        }

        fn eval_string(&self) -> String {
            format!("{}", self.value)
        }
    }
}

pub mod operator {
    use crate::packet::{parse_bin, Packet};
    use crate::packet_parse::{find_subpackets};
    use super::{Header};

    pub struct Operator {
        header: Header,
        subpackets: Vec<Box<dyn Packet>>,
    }
    
    impl Operator {
        pub fn new(binary: &str) -> Operator {
            let header = Header::from(binary);

            let subpackets = find_subpackets(binary).into_iter()
                .map(|p| parse_bin(&p))
                .collect();

            Operator { header, subpackets }
        }

        fn operate(&self) -> u128 {
            let mut evaluated = self.evaluated().into_iter();
            match self.header.type_id {
                // sum
                0 => evaluated.sum(),
                // product
                1 => evaluated.reduce(|a, b| a * b).unwrap(),
                // min
                2 => evaluated.min().unwrap(),
                // max
                3 => evaluated.max().unwrap(),
                // greater
                5 => if evaluated.next().unwrap() > evaluated.next().unwrap() { 1 } else { 0 },
                // lesser
                6 => if evaluated.next().unwrap() < evaluated.next().unwrap() { 1 } else { 0 },
                // equal
                7 => if evaluated.next().unwrap() == evaluated.next().unwrap() { 1 } else { 0 },
                _ => panic!("invalid type id"),
            }
        }

        fn evaluated(&self) -> Vec<u128> {
            self.subpackets.iter().map(|sp| sp.evaluate()).collect()
        }
    }
    
    impl Packet for Operator {
        fn versions(&self) -> Vec<u8> {
            vec![self.header.version].into_iter().chain(self.subpackets.iter().flat_map(|p| p.versions())).collect()
        }

        fn values(&self) -> Vec<u128> {
            self.subpackets.iter().flat_map(|p| p.values()).collect()
        }

        fn evaluate(&self) -> u128 {
            self.operate()
        }

        fn eval_string(&self) -> String {
            match self.header.type_id {
                r @ 0..=3 => {
                    let vals: Vec<String> = self.subpackets.iter().map(|sp| sp.eval_string()).collect();
                    match r {
                        0 => format!("({})", vals.join("+")),
                        1 => format!("({})", vals.join("*")),
                        2 => format!("min({})", vals.join(",")),
                        3 => format!("max({})", vals.join(",")),
                        _ => panic!("wish Rust were smarter"),
                    }
                },
                b @ 5..=7 =>{
                    let vals: Vec<String> = self.subpackets.iter().map(|sp| sp.eval_string()).collect();
                    let op = match b {
                        5 => ">",
                        6 => "<",
                        7 => "=",
                        _ => panic!("wish Rust were smarter"),
                    };
                    format!("({}{}{})", vals[0], op, vals[1])
                },
                _ => panic!("invalid type id"),
            }
        }
    }
}