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

/// Here's how the conversion works:
/// 1. Take the binary data and divide it into 8-bit bytes: Our `bytes` input
/// 2. Group the bytes into sets of three bytes (for a total of 24 bits): Line 41
/// 3. Divide those 24 bits into four sets of six bits.
/// 4. Convert each set of six bits into a decimal number. This gives you four decimal numbers for each three-byte group.
/// 5. Map each decimal number to a character in the Base64 alphabet.
pub fn bytes_to_base64(bytes: &[u8]) -> String {
    // This is a string containing all possible characters for Base64 encoding.
    let base64_chars = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

    // This is an empty `String` that will store the Base64-encoded result.
    let mut base64 = String::new();

    // This loop goes over the `bytes` slice in chunks of 3 bytes each.
    // Base64 encoding works on 3 bytes at a time.
    for chunk in bytes.chunks(3) {
        // This `temp` variable is going to hold the combined value of the bytes in the current chunk.
        // It's initialized to 0 for each new chunk.
        let mut temp: u32 = 0;

        // This loop goes over each byte in the current chunk.
        // It shifts the byte to the correct position and adds it to `temp`.
        for (i, &byte) in chunk.iter().enumerate() {
            // Shift the byte value by (16 - 8 * i) bits and bitwise OR with `temp`.
            // For the first byte, it will shift by 16 bits, for the second by 8 and for the third by 0.
            // The bitwise OR operation (|) is used to combine bits together.
            temp |= (byte as u32) << (16 - 8 * i);
        }

        // This loop generates the Base64 characters for the current chunk.
        // Base64 encoding outputs 4 characters for every 3 bytes.
        for i in 0..4 {
            // If we have enough bits in the chunk for another Base64 character
            if i * 6 < chunk.len() * 8 {
                // Extract the next 6 bits from `temp` and use them as an index into `base64_chars`.
                let index = ((temp >> (18 - 6 * i)) & 63) as usize;

                // Get the Base64 character corresponding to `index` and add it to `base64`.
                base64.push(base64_chars.chars().nth(index).unwrap());
            } else {
                // If there aren't enough bits left in `temp` for another Base64 character,
                // add a padding character to `base64`.
                base64.push('=');
            }
        }
    }

    // Return the Base64-encoded string.
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
    const HEX_STRING: &str = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    const BASE_64_RESULT: &str = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";

    let result = bytes_to_base64(&hex_to_bytes(HEX_STRING));
    assert_eq!(result, BASE_64_RESULT);
}
