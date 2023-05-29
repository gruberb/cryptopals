pub fn decode(input: &str) -> Result<Vec<u8>, std::num::ParseIntError> {
    (0..input.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&input[i..i + 2], 16))
        .collect()
}

pub fn xor(first: Vec<u8>, second: Vec<u8>) -> Vec<u8> {
    if first.len() != second.len() {
        panic!("Lengths of the two vectors are not equal");
    }

    first
        .iter()
        .zip(second.iter())
        .map(|(a, b)| a ^ b)
        .collect()
}

#[test]
fn test_xor() {
    let first = decode("1c0111001f010100061a024b53535009181c").unwrap();
    let second = decode("686974207468652062756c6c277320657965").unwrap();
    let expected = decode("746865206b696420646f6e277420706c6179").unwrap();
    assert_eq!(xor(first, second), expected);
}
