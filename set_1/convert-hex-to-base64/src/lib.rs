#[allow(dead_code)]
const HEX_STRING: &str = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";

#[allow(dead_code)]
const BASE_64_RESULT: &str = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";

pub fn hex_to_bytes(input: &str) -> Vec<u8> {
    input
        .as_bytes()
        .chunks(2)
        .map(|pair| {
            let pair_str = std::str::from_utf8(pair).expect("Invalid UTF8");
            u8::from_str_radix(pair_str, 16).expect("Decoding failed")
        })
        .collect()
}

#[test]
fn get_bytes_from_hex() {
    // "Hello World"
    let hex_str = "48656c6c6f20576f726c64";
    let bytes = hex_to_bytes(hex_str);
    assert_eq!(bytes, b"Hello World");
}

pub fn bytes_to_base64(bytes: &[u8]) -> String {
    let base64_chars = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut base64 = String::new();

    for chunk in bytes.chunks(3) {
        let mut temp: u32 = 0;
        for (i, &byte) in chunk.iter().enumerate() {
            temp |= (byte as u32) << (16 - 8 * i);
        }

        for i in 0..4 {
            if i * 6 < chunk.len() * 8 {
                let index = ((temp >> (18 - 6 * i)) & 63) as usize;
                base64.push(base64_chars.chars().nth(index).unwrap());
            } else {
                base64.push('=');
            }
        }
    }

    base64
}

#[test]
fn test_bytes_to_base64() {
    let bytes = b"Hello World"; // byte string for "Hello World"
    let base64_str = bytes_to_base64(bytes);
    assert_eq!(base64_str, "SGVsbG8gV29ybGQ=");
}

#[test]
fn convert_hex_to_base64() {
    let result = bytes_to_base64(&hex_to_bytes(HEX_STRING));

    assert_eq!(result, BASE_64_RESULT);
}
