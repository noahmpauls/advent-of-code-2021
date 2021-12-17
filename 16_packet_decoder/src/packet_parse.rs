use literal_body::LiteralBody;

const VERSION: usize = 3;
const TYPE_ID: usize = 3;
const LEN_ID: usize = 1;
const LEN_BITS: usize = 15;
const LEN_PACKETS: usize = 11;
const HEADER: usize = VERSION + TYPE_ID;

/// Get the version of a packet.
pub fn version(packet: &str) -> u8 {
    assert!(packet.len() >= VERSION, "packet too short");
    u8::from_str_radix(&packet[..VERSION], 2).unwrap()
}

/// Get the type ID of a packet.
pub fn type_id(packet: &str) -> u8 {
    assert!(packet.len() >= HEADER, "packet too short");
    u8::from_str_radix(&packet[VERSION..HEADER], 2).unwrap()
}

/// Get the header of a literal packet.
pub fn _literal_header(packet: &str) -> &str {
    assert!(packet.len() >= HEADER, "packet too short");
    &packet[..HEADER]
}

/// Get the body of a literal packet, including any extra trailing 0's.
pub fn literal_body(packet: &str) -> &str {
    assert!(packet.len() >= HEADER, "packet too short");
    &packet[HEADER..]
}

/// Get the parsed value from a literal packet.
pub fn literal_value(packet_fragment: &str) -> String {
    LiteralBody::from(packet_fragment).value()
}

const HEADER_LENID: usize = HEADER + LEN_ID;

/// Get the length type ID of an operator packet.
pub fn len_type_id(packet: &str) -> u8 {
    assert!(packet.len() >= HEADER_LENID, "packet too short");
    packet.chars().nth(HEADER_LENID - 1).unwrap().to_digit(2).unwrap().try_into().unwrap()
}

/// Get the subpacket length field of an operator packet, either a bit count or 
/// a packet count depending on the packet's length type ID.
pub fn subpackets_len(packet: &str) -> usize {
    match len_type_id(packet) {
        0 => subpackets_len_bits(packet),
        1 => subpackets_len_packets(packet),
        _ => panic!("invalid length id; should not get here"),
    }
}

/// Get the subpacket length of an operator packet as a bit count.
pub fn subpackets_len_bits(packet: &str) -> usize {
    assert!(packet.len() >= HEADER + LEN_BITS, "packet too short");
    usize::from_str_radix(&packet[HEADER..HEADER_LENID + LEN_BITS], 2).unwrap()
}

/// Get the subpacket length of an operator packet as a packet count.
pub fn subpackets_len_packets(packet: &str) -> usize {
    assert!(packet.len() >= HEADER_LENID + LEN_PACKETS, "packet too short");
    usize::from_str_radix(&packet[HEADER_LENID..HEADER_LENID + LEN_PACKETS], 2).unwrap()
}

/// Get the header of an operator packet, including length metadata.
pub fn operator_header(packet: &str) -> &str {
    let len_type_id = match len_type_id(packet) {
        0 => LEN_BITS,
        1 => LEN_PACKETS,
        _ => panic!("invalid length id; should not get here"),
    };

    &packet[..HEADER_LENID + len_type_id]
}

/// Get the body of an operator packet, excluding length metadata.
pub fn subpackets_body(packet: &str) -> &str {
    match len_type_id(packet) {
        0 => &packet[HEADER_LENID + LEN_BITS..],
        1 => &packet[HEADER_LENID + LEN_PACKETS..],
        _ => panic!("invalid length id; should not get here"),
    }
}

/// Find all the subpackets of the packet that leads a fragment.
pub fn find_subpackets(packet_fragment: &str) -> Vec<&str> {
    if type_id(packet_fragment) == 4 {
        return vec![];
    }

    match len_type_id(packet_fragment) {
        0 => find_subpackets_bits(packet_fragment),
        1 => find_subpackets_count(packet_fragment),
        _ => panic!("should never get here"),
    }
}

/// Find the first packet of any kind that leads a fragment.
fn lead_packet(packet_fragment: &str) -> &str {
    match type_id(packet_fragment) {
        4 => {
            let len = find_literal(packet_fragment).len();
            &packet_fragment[..len]
        },
        _ => {
            let len = find_operator(packet_fragment).len();
            &packet_fragment[..len]
        }
    }
}

/// Find first literal packet that leads a fragment.
fn find_literal(packet_fragment: &str) -> &str {
    assert!(type_id(packet_fragment) == 4, "not a literal");
    let body = LiteralBody::from(packet_fragment).body();
    &packet_fragment[..HEADER + body.len()]
}

/// Find the first operator packet that leads a fragment.
fn find_operator(packet_fragment: &str) -> &str {
    let header_len = operator_header(packet_fragment).len();
    let body_len: usize = find_subpackets(packet_fragment).iter().map(|sp| sp.len()).sum();

    &packet_fragment[..header_len + body_len]
}

/// Find the subpackets of an operator packet where the length is given as a 
/// bit count.
fn find_subpackets_bits(packet_fragment: &str) -> Vec<&str> {
    let subpackets_len = subpackets_len(packet_fragment);
    let subpackets_body = subpackets_body(packet_fragment);

    let mut result = Vec::new();
    let mut bits = subpackets_len;
    let mut remaining = subpackets_body;

    while bits > 0 {
        let next_packet = lead_packet(remaining);
        assert!(next_packet.len() <= bits, "subpackets not parseable in {} bits", subpackets_len);
        bits -= next_packet.len();
        remaining = &remaining[next_packet.len()..];
        result.push(next_packet);
    }

    result
}

/// Find the subpackets of an operator packet where the length is given as a 
/// packet count.
fn find_subpackets_count(packet_fragment: &str) -> Vec<&str> {
    let subpackets_len = subpackets_len(packet_fragment);
    let subpackets_body = subpackets_body(packet_fragment);

    let mut result = Vec::new();
    let mut count = subpackets_len;
    let mut remaining = subpackets_body;
    
    while count > 0 {
        let next_packet = lead_packet(remaining);
        count -= 1;
        remaining = &remaining[next_packet.len()..];
        result.push(next_packet);
    }

    result
}

mod literal_body {
    use super::{literal_body};
    const CHUNK: usize = 5;

    pub struct LiteralBody<'a> {
        binary: &'a str,
        i: usize,
        complete: bool,
    }
    
    impl<'a> LiteralBody<'a> {
        pub fn from(packet: &str) -> LiteralBody  {
            LiteralBody {
                binary: literal_body(packet),
                i: 0,
                complete: false,
            }
        }
    
        pub fn body(self) -> String {
            self.collect::<String>()
        }
    
        pub fn value(self) -> String {
            self.map(|chunk| &chunk[1..]).collect::<String>()
        }
    }
    
    impl<'a> Iterator for LiteralBody<'a> {
        type Item = &'a str;
    
        fn next(&mut self) -> Option<Self::Item> {
            if self.complete {
                None
            } else if self.i + CHUNK > self.binary.len() {
                self.complete = true;
                None
            } else {
                let chunk = &self.binary[self.i..self.i+CHUNK];
                let lead = chunk.chars().next().unwrap().to_digit(2).unwrap();
                
                if lead == 0 {
                    self.complete = true;
                }
                self.i += CHUNK;
                Some(chunk)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // { 110 100 } [ {1 0111} {1 1110} {0 0101} ] 000
    const LITERAL: &str = "110100101111111000101000";
    // { 001 110 0 000000000011011} [
    //     { 110 100 } [ {0 1010} ]
    //     { 010 100 } [ {1 0001} {0 0100} ]
    // ] 0000000
    const OP_BITS: &str = "00111000000000000110111101000101001010010001001000000000";
    // { 111 011 1 00000000011 } [
    //     { 010 100 } [ {0 0001} ] 
    //     { 100 100 } [ {0 0010} ]
    //     { 001 100 } [ {0 0011} ]
    // ] 00000
    const OP_COUNT: &str = "11101110000000001101010000001100100000100011000001100000";
    // { 100 010 1 00000000001 } [
    //     { 001 010 1 00000000001 } [
    //         { 101 010 0 000000000001011 } [
    //             { 110 100 } [ {0 1111} ]
    //         ]
    //     ]
    // ] 000
    const NESTED: &str = "100010100000000001001010100000000001101010000000000000101111010001111000";

    #[test]
    fn test_version() {
        let bin = LITERAL;
        let version = version(bin);
        let expected = 6;

        assert_eq!(expected, version);
    }

    #[test]
    fn test_typeid() {
        let bin = LITERAL;
        let type_id = type_id(bin);
        let expected = 4;

        assert_eq!(expected, type_id);
    }

    #[test]
    fn test_literal_body() {
        let bin = LITERAL;
        let body = literal_body(bin);
        let expected = "101111111000101000";

        assert_eq!(expected, body);
    }

    #[test]
    fn test_literal_value() {
        let bin = LITERAL;
        let body = literal_value(bin);
        let expected = "011111100101";

        assert_eq!(expected, body);
    }

    #[test]
    fn test_operator_bits_typeid() {
        let expected = 0;

        assert_eq!(expected, len_type_id(OP_BITS));
    }

    #[test]
    fn test_operator_count_typeid() {
        let expected = 1;

        assert_eq!(expected, len_type_id(OP_COUNT));
    }

    #[test]
    fn test_operator_bits_len() {
        let expected = 27;

        assert_eq!(expected, subpackets_len(OP_BITS));
    }

    #[test]
    fn test_operator_count_len() {
        let expected = 3;

        assert_eq!(expected, subpackets_len(OP_COUNT));
    }

    #[test]
    fn test_operator_bits_body() {
        let expected = "1101000101001010010001001000000000";

        assert_eq!(expected, subpackets_body(OP_BITS));
    }

    #[test]
    fn test_operator_count_body() {
        let expected = "01010000001100100000100011000001100000";

        assert_eq!(expected, subpackets_body(OP_COUNT));
    }

    #[test]
    fn test_operator_count_body_2() {
        let bin = NESTED;
        let expected = "001010100000000001101010000000000000101111010001111000";

        assert_eq!(expected, subpackets_body(bin));
    }

    #[test]
    fn test_lead_packet_1() {
        let bin = NESTED;
        let expected = "100010100000000001001010100000000001101010000000000000101111010001111";
        assert_eq!(expected, lead_packet(bin));
    }

    #[test]
    fn test_find_subpackets_bits1() {
        let expected = vec![
            "11010001010",
            "0101001000100100",
        ];

        assert_eq!(expected, find_subpackets(OP_BITS));
    }

    #[test]
    fn test_find_subpackets_bits2() {
        let bin = NESTED;
        let expected = vec![
            "001010100000000001101010000000000000101111010001111"
        ];

        assert_eq!(expected, find_subpackets(bin));
    }

    #[test]
    fn test_find_subpackets_count1() {
        let expected = vec![
            "01010000001",
            "10010000010",
            "00110000011",
        ];

        assert_eq!(expected, find_subpackets(OP_COUNT));
    }
}
