const BASE58_ALPHABET: &'static [u8] =
    b"123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";

pub fn encode(input: &str) -> String {
    let bytes = input.as_bytes().to_vec();
    let mut result = Vec::new();

    let mut decimal = 0u128;

    for byte in bytes.iter() {
        decimal = decimal * 256 + *byte as u128;
    }

    while decimal > 0 {
        let i = (decimal % 58) as usize;
        decimal /= 58;
        result.push(BASE58_ALPHABET[i]);
    }

    for &byte in &bytes {
        if byte == 0 {
            result.push(BASE58_ALPHABET[0]);
        } else {
            break;
        }
    }
    result.reverse();
    String::from_utf8(result).unwrap()
}

pub fn decode(input: &str) -> String {
    let bytes = input.as_bytes().to_vec();
    let mut result = Vec::new();

    let mut decimal = 0u128;

    for byte in bytes.iter() {
        let value = BASE58_ALPHABET.iter().position(|&x| x == *byte).unwrap();
        decimal = decimal * 58 + value as u128;
    }

    while decimal > 0 {
        let byte = (decimal % 256) as u8;
        decimal /= 256;
        result.push(byte);
    }

    for &byte in &bytes {
        if byte == BASE58_ALPHABET[0] {
            result.push(0);
        } else {
            break;
        }
    }
    result.reverse();
    String::from_utf8(result).unwrap()
}
