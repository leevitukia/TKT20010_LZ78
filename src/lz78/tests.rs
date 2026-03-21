use tempfile::NamedTempFile;

use super::*;

use rand::Rng;
// Pair Struct

#[test]
fn length_matches_bit_count() {
    assert_eq!(Pair::new(0, None).encode().len(), 2);
    assert_eq!(Pair::new(255, None).encode().len(), 2);

    assert_eq!(Pair::new(2_u64.pow(8), None).encode().len(), 3);
    assert_eq!(Pair::new(2_u64.pow(16) - 1, None).encode().len(), 3);

    assert_eq!(Pair::new(2_u64.pow(16), None).encode().len(), 5);
    assert_eq!(Pair::new(2_u64.pow(32) - 1, None).encode().len(), 5);

    assert_eq!(Pair::new(2_u64.pow(32), None).encode().len(), 9);
    assert_eq!(Pair::new(u64::MAX, None).encode().len(), 9);
}

// LZ78 Encode & Decode

fn encode_to_bytes(input: &[u8]) -> Vec<u8> {
    let mut input_file = NamedTempFile::new().unwrap();
    input_file.write_all(input).unwrap();

    let output_file = NamedTempFile::new().unwrap();

    super::encode(input_file.path(), output_file.path());

    std::fs::read(output_file.path()).unwrap()
}

fn decode_to_bytes(input: &[u8]) -> Vec<u8> {
    let mut input_file = NamedTempFile::new().unwrap();
    input_file.write_all(input).unwrap();

    let output_file = NamedTempFile::new().unwrap();

    super::decode(input_file.path(), output_file.path());

    std::fs::read(output_file.path()).unwrap()
}

#[test]
fn encode_empty_input() {
    let output = encode_to_bytes(b"");
    assert_eq!(output, vec![]);
}

#[test]
fn decoded_file_matches_input() { // 10 MiB random input
    let mut input = vec![0u8; 1024 * 1024 * 10];
    rand::rng().fill_bytes(&mut input);
    let decoded = decode_to_bytes(&encode_to_bytes(&input));

    assert!(input == decoded, "Decoded file does not match the input");
}

#[test]
fn encoded_file_is_smaller_than_input() {
    let input = vec![0u8; 1024 * 10];
    let compressed = encode_to_bytes(&input);

    assert!(compressed.len() < input.len(), "Encoded file is larger than the input");
}