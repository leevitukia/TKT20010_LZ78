
#[test]
fn write_10_bits() {
    let mut bytes = Vec::new();
    let mut writer = super::BitWriter::new(&mut  bytes, None);

    writer.write_bits(10, 2_u64.pow(10) - 1).unwrap();
    writer.flush().unwrap();
    drop(writer);
    assert_eq!(bytes, vec![255, 3])
}

#[test]
fn write_4_bit_integers() {
    let mut bytes = Vec::new();
    let mut writer = super::BitWriter::new(&mut  bytes, None);

    writer.write_bits(4, 2_u64.pow(4) - 1).unwrap();
    writer.write_bits(4, 0).unwrap();
    writer.write_bits(4, 2_u64.pow(4) - 1).unwrap();
    writer.write_bits(4, 2_u64.pow(4) - 1).unwrap();
    writer.flush().unwrap();
    drop(writer);
    assert_eq!(bytes, vec![15, 255])
}

#[test]
fn write_20_bits_with_10_bit_chunks_and_continuation_bit() {
    let mut bytes = Vec::new();
    let mut writer = super::BitWriter::new(&mut  bytes, None);

    writer.write_bits_with_continuation_bit(10, 2_u64.pow(20) - 1).unwrap();
    writer.flush().unwrap();
    drop(writer);
    assert_eq!(bytes, vec![255, 247, 63]);
}

// 11_00_00_00

/* 
#[test]
fn decoded_file_matches_input() { // 10 MiB random input
    let mut input = vec![0u8; 1024 * 1024 * 10];
    rand::rng().fill_bytes(&mut input);
    let decoded = decode_to_bytes(&encode_to_bytes(&mut input));

    assert!(input == decoded, "Decoded file does not match the input");
}

#[test]
fn encoded_file_is_smaller_than_input() {
    let mut input = vec![0u8; 1024 * 10];
    let compressed = encode_to_bytes(&mut input);

    assert!(compressed.len() < input.len(), "Encoded file is larger than the input");
} */