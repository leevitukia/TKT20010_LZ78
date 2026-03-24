use std::{fs::{File}, io::{BufReader, Read}};

use crate::bit_traits::BitReadable;


pub struct BitReader<R: Read> {
    reader: BufReader<R>,
    pub buffer: u64,
    pub bits_read: u64,
    pub bit_pos: u64,
}


impl<R: Read> BitReader<R> {
    pub fn read<T: BitReadable>(&mut self) -> T {
        T::read(self)
    }

    pub fn new(inner: R) -> Self {
        let mut reader = BufReader::new(inner);
        let mut buf = [0_u8; 8];
        reader.read_exact(&mut buf).unwrap();
        let buffer = u64::from_le_bytes(buf);

        Self { reader, buffer, bits_read: 0, bit_pos: 0 }
    }

    pub fn read_bits(&mut self, bits: u64) -> u64 {
        let mut bits_remaining = bits;

        let mut value = 0;

        while bits_remaining > 0 {
            if self.bit_pos >= 64 {
                let mut buf = [0_u8; 8];
                self.reader.read_exact(&mut buf).unwrap();
                let buffer = u64::from_le_bytes(buf);
                self.buffer = buffer;
                self.bit_pos = 0;
            }

            let bits_to_read = (64 - self.bit_pos).min(bits_remaining);

            let mask: u64 = ((1 << bits_to_read) - 1) << self.bit_pos;
            let extracted = (self.buffer & mask) >> self.bit_pos;

            value |= extracted << (bits - bits_remaining);
            
            bits_remaining -= bits_to_read;
            self.bit_pos += bits_to_read;
            self.bits_read += bits_to_read;
        }

        return value;
    }
}