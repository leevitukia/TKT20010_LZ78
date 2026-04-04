use std::io::Cursor;

use rand::Rng;

fn encode_to_bytes(input: &mut Vec<u8>) -> Vec<u8> {
    let mut output = Vec::new();
    let mut cursor = Cursor::new(input);
    super::encode(&mut cursor, &mut output).unwrap();
    return output
}

fn decode_to_bytes(input: &[u8]) -> Vec<u8> {
    let mut output = Vec::new();
    super::decode(input, &mut output).unwrap();

    return output;
}

#[test]
fn encode_empty_input() {
    let output = encode_to_bytes(&mut vec![]);
    assert_eq!(output, vec![]);
}

#[test]
fn decoded_file_matches_input() { // 5 MiB random input
    let mut input = vec![0u8; 1024 * 1024 * 5];
    rand::rng().fill_bytes(&mut input);
    let decoded = decode_to_bytes(&encode_to_bytes(&mut input));

    assert!(input == decoded, "Decoded file does not match the input");
}

#[test]
fn encoded_file_is_smaller_than_input() { // 10 KiB
    let mut input = vec![0u8; 1024 * 10];
    let compressed = encode_to_bytes(&mut input);

    assert!(compressed.len() < input.len(), "Encoded file is larger than the input");
}