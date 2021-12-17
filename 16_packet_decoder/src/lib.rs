mod packet_parse;
mod packet_types;

pub mod packet {
    use crate::packet_types::literal::Literal;
    use crate::packet_types::operator::Operator;
    use crate::packet_parse::{type_id};

    pub trait Packet {
        fn versions(&self) -> Vec<u8>;

        fn values(&self) -> Vec<u128>;

        fn evaluate(&self) -> u128;

        fn eval_string(&self) -> String;
    }

    pub fn parse_hex(hex: &str) -> Box<dyn Packet> {
        let binary = hex_to_bin(hex);
        parse_bin(&binary)
    }
    
    fn hex_to_bin(hex: &str) -> String {
        hex.chars().map(|c| {
            let num = c.to_digit(16).expect(&format!("char `{}` is not a hex digit", c));
            format!("{:04b}", num)
        }).collect()
    }

    pub fn parse_bin(bin: &str) -> Box<dyn Packet> {
        match type_id(bin) {
            4 => literal(bin),
            _id => operator(bin),
        }
    }

    pub fn literal(bin: &str) -> Box<dyn Packet> {
        Box::new(Literal::new(bin))
    }

    pub fn operator(bin: &str) -> Box<dyn Packet> {
        Box::new(Operator::new(bin))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_packet_literal() {
        let bin = "110100101111111000101000";
        let literal = packet::parse_bin(bin);
        let expected = vec![2021];

        assert_eq!(expected, literal.values());
    }

    #[test]
    fn test_parse_packet_operator_bits() {
        let bin = "00111000000000000110111101000101001010010001001000000000";
        let operator = packet::parse_bin(bin);
        let expected = vec![10, 20];

        assert_eq!(expected, operator.values());
    }

    #[test]
    fn test_parse_packet_operator_count() {
        let bin = "11101110000000001101010000001100100000100011000001100000";
        let operator = packet::parse_bin(bin);
        let expected = vec![1, 2, 3];

        assert_eq!(expected, operator.values());
    }

    #[test]
    fn test_versions_example_0() {
        let hex = "D2FE28";
        let packet = packet::parse_hex(hex);
        let versions = vec![6];

        assert_eq!(versions, packet.versions());
    }

    #[test]
    fn test_versions_example_1() {
        let hex = "38006F45291200";
        let packet = packet::parse_hex(hex);
        let versions = vec![1, 6, 2];

        assert_eq!(versions, packet.versions());
    }

    #[test]
    fn test_versions_example_2() {
        let hex = "EE00D40C823060";
        let packet = packet::parse_hex(hex);
        let versions = vec![7, 2, 4, 1];

        assert_eq!(versions, packet.versions());
    }

    // 8A004A801A8002F478 represents an operator packet (version 4) which 
    // contains an operator packet (version 1) which contains an operator packet
    // (version 5) which contains a literal value (version 6); this packet has a
    // version sum of 16.
    #[test]
    fn test_versions_example_3() {
        let hex = "8A004A801A8002F478";
        let packet = packet::parse_hex(hex);
        let versions = vec![4, 1, 5, 6];

        assert_eq!(versions, packet.versions());
    }

    // 620080001611562C8802118E34 represents an operator packet (version 3) 
    // which contains two sub-packets; each sub-packet is an operator packet 
    // that contains two literal values. This packet has a version sum of 12.
    #[test]
    fn test_versions_example_4() {
        let hex = "620080001611562C8802118E34";
        let packet = packet::parse_hex(hex);

        assert_eq!(12_u128, packet.versions().iter().map(|&v| v as u128).sum());
    }

    // C0015000016115A2E0802F182340 has the same structure as the previous 
    // example, but the outermost packet uses a different length type ID. 
    // This packet has a version sum of 23.
    #[test]
    fn test_versions_example_5() {
        let hex = "C0015000016115A2E0802F182340";
        let packet = packet::parse_hex(hex);

        assert_eq!(23_u128, packet.versions().iter().map(|&v| v as u128).sum());
    }

    // A0016C880162017C3686B18A3D4780 is an operator packet that contains an 
    // operator packet that contains an operator packet that contains five 
    // literal values; it has a version sum of 31.
    #[test]
    fn test_versions_example_6() {
        let hex = "A0016C880162017C3686B18A3D4780";
        
        let packet = packet::parse_hex(hex);

        assert_eq!(31_u128, packet.versions().iter().map(|&v| v as u128).sum());
    }


    // C200B40A82 finds the sum of 1 and 2, resulting in the value 3.
    #[test]
    fn test_eval_example_1() {
        let hex = "C200B40A82";
        let packet = packet::parse_hex(hex);

        assert_eq!(3, packet.evaluate());
    }

    // 04005AC33890 finds the product of 6 and 9, resulting in the value 54.
    #[test]
    fn test_eval_example_2() {
        let hex = "04005AC33890";
        let packet = packet::parse_hex(hex);

        assert_eq!(54, packet.evaluate());
    }

    // 880086C3E88112 finds the minimum of 7, 8, and 9, resulting in the value 7.
    #[test]
    fn test_eval_example_3() {
        let hex = "880086C3E88112";
        let packet = packet::parse_hex(hex);

        assert_eq!(7, packet.evaluate());
    }

    // CE00C43D881120 finds the maximum of 7, 8, and 9, resulting in the value 9.
    #[test]
    fn test_eval_example_4() {
        let hex = "CE00C43D881120";
        let packet = packet::parse_hex(hex);

        assert_eq!(9, packet.evaluate());
    }

    // D8005AC2A8F0 produces 1, because 5 is less than 15.
    #[test]
    fn test_eval_example_5() {
        let hex = "D8005AC2A8F0";
        let packet = packet::parse_hex(hex);

        assert_eq!(1, packet.evaluate());
    }

    // F600BC2D8F produces 0, because 5 is not greater than 15.
    #[test]
    fn test_eval_example_6() {
        let hex = "F600BC2D8F";
        let packet = packet::parse_hex(hex);

        assert_eq!(0, packet.evaluate());
    }

    // 9C005AC2F8F0 produces 0, because 5 is not equal to 15.
    #[test]
    fn test_eval_example_7() {
        let hex = "9C005AC2F8F0";
        let packet = packet::parse_hex(hex);

        assert_eq!(0, packet.evaluate());
    }

    // 9C0141080250320F1802104A08 produces 1, because 1 + 3 = 2 * 2.
    #[test]
    fn test_eval_example_8() {
        let hex = "9C0141080250320F1802104A08";
        let packet = packet::parse_hex(hex);

        assert_eq!(1, packet.evaluate());
    }

    #[test]
    fn test_huge() {
        let hex = "E20D7880532D4E551A5791BD7B8C964C1548CB3EC1FCA41CC00C6D50024400C202A65C00C20257C008AF70024C00810039C00C3002D400A300258040F200D6040093002CC0084003FA52DB8134DE620EC01DECC4C8A5B55E204B6610189F87BDD3B30052C01493E2DC9F1724B3C1F8DC801E249E8D66C564715589BCCF08B23CA1A00039D35FD6AC5727801500260B8801F253D467BFF99C40182004223B4458D2600E42C82D07CC01D83F0521C180273D5C8EE802B29F7C9DA1DCACD1D802469FF57558D6A65372113005E4DB25CF8C0209B329D0D996C92605009A637D299AEF06622CE4F1D7560141A52BC6D91C73CD732153BF862F39BA49E6BA8C438C010E009AA6B75EF7EE53BBAC244933A48600B025AD7C074FEB901599A49808008398142013426BD06FA00D540010C87F0CA29880370E21D42294A6E3BCF0A080324A006824E3FCBE4A782E7F356A5006A587A56D3699CF2F4FD6DF60862600BF802F25B4E96BDD26049802333EB7DDB401795FC36BD26A860094E176006A0200FC4B8790B4001098A50A61748D2DEDDF4C6200F4B6FE1F1665BED44015ACC055802B23BD87C8EF61E600B4D6BAD5800AA4E5C8672E4E401D0CC89F802D298F6A317894C7B518BE4772013C2803710004261EC318B800084C7288509E56FD6430052482340128FB37286F9194EE3D31FA43BACAF2802B12A7B83E4017E4E755E801A2942A9FCE757093005A6D1F803561007A17C3B8EE0008442085D1E8C0109E3BC00CDE4BFED737A90DC97FDAE6F521B97B4619BE17CC01D94489E1C9623000F924A7C8C77EA61E6679F7398159DE7D84C015A0040670765D5A52D060200C92801CA8A531194E98DA3CCF8C8C017C00416703665A2141008CF34EF8019A080390962841C1007217C5587E60164F81C9A5CE0E4AA549223002E32BDCEA36B2E100A160008747D8B705C001098DB13A388803F1AE304600";
        let packet = packet::parse_hex(hex);

        assert_eq!(979_u128, packet.versions().iter().map(|&v| v as u128).sum());
        assert_eq!(277110354175_u128, packet.evaluate());
    }
}
