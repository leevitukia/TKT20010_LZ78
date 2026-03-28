use std::{io::Cursor};

#[test]
fn read_10_bits() {
    let bytes: Vec<u8> = vec![255, 3]; // 1023
    let mut reader = super::BitReader::new(Cursor::new(bytes), None);
    let value = reader.read_bits(10).unwrap();
    assert_eq!(value, 1023)
}

#[test]
fn read_20_bits_with_10_bit_chunks_and_continuation_bit() {
    let bytes = vec![255, 247, 63]; //1048575
    let mut reader = super::BitReader::new(Cursor::new(bytes), None);
    let value = reader.read_bits_with_continuation_bit(10).unwrap();
    assert_eq!(value, 1048575)
}

#[test]
fn read_4_bit_integers() {
    let bytes = vec![15, 255]; // 15, 0, 15, 15
    let mut reader = super::BitReader::new(Cursor::new(bytes), None);
    let integers: Vec<u64> = (0..4).map(|_| reader.read_bits(4).unwrap()).collect();

    assert_eq!(integers, [15, 0, 15, 15])
}

/* 

#[test]
fn write_20_bits_with_10_bit_chunks_and_continuation_bit() {
    let mut bytes = Vec::new();
    let mut writer = super::BitWriter::new(&mut  bytes, None);

    writer.write_bits_with_continuation_bit(10, 2_u64.pow(20) - 1).unwrap();
    writer.flush().unwrap();
    drop(writer);
    assert_eq!(bytes, vec![255, 247, 63]);
} */