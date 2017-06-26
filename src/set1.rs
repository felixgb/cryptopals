pub fn hex2base64(inp: &str) -> &str {
    let foo = inp
        .chars()
        .map(|c| c.to_digit(16).map(|n| dec2bits(n as u8)))
        .collect::<Vec<_>>();

        // convert each digit to four bits
    print!("{:?}", foo);
    "fartass"
}

//pub fn traverse<T>(xs: Vec<Option<T>>) -> Option<Vec<T>> {
//    let out = Vec::new();
//    for x in xs {
//        if let Some(ok) = x {
//            out.push(ok);
//        }
//    }
//}

pub fn dec2bits(n: u8) -> String {
    format!("{:04b}", n).to_string()
}

#[cfg(test)]
mod test_set1 {
    use set1;

    #[test]
    fn test_dec2bits() {
        assert!(set1::dec2bits(2) == "0010".to_string())
    }

    #[test]
    fn test_hex2base64() {
        let input = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
        let expected = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";
        assert!(set1::hex2base64(input) == expected)
    }
}
