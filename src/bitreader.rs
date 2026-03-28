use std::{fs::File, io::{BufRead, BufReader, Read}};

use anyhow::Error;
use rand::random;

use crate::bit_traits::BitReadable;


pub struct BitReader<R: Read> {
    reader: BufReader<R>,
    pub buffer: u64,
    pub bits_read: u64,
    pub bit_pos: u32,
    pub default_chunk_size: u32
}


impl<R: Read> BitReader<R> {
    pub fn read<T: BitReadable>(&mut self) -> anyhow::Result<T> {
        T::read(self)
    }

    pub fn new(inner: R, default_chunk_size: Option<u32>) -> Self {
        let mut reader = BufReader::new(inner);
        let mut buf = Vec::with_capacity(8);
        (&mut reader).take(8).read_to_end(&mut buf).unwrap();
        if buf.len() < 8 {
            buf.resize(8, 0);
        }
        let buffer = u64::from_le_bytes(buf.try_into().unwrap());

        let default_chunk_size = default_chunk_size.unwrap_or(10);
        Self { reader, buffer, bits_read: 0, bit_pos: 0, default_chunk_size }
    }

    pub fn read_bits(&mut self, bits: u32) -> anyhow::Result<u64> {
        let mut bits_remaining = bits;

        let mut value = 0;

        while bits_remaining > 0 {
            if self.bit_pos >= 64 {
                let mut buf = Vec::with_capacity(8);
                
                (&mut self.reader).take(8).read_to_end(&mut buf)?;

                if buf.len() < 8 {
                    if buf.len() == 0{
                        anyhow::bail!("EOF");
                    }
                    buf.resize(8, 0);
                }

                let buffer = u64::from_le_bytes(buf.try_into().unwrap());
                self.buffer = buffer;
                self.bit_pos = 0;
            }

            let bits_to_read = (64 - self.bit_pos).min(bits_remaining);

            let mask: u64 = ((1 << bits_to_read) - 1) << self.bit_pos;
            let extracted = (self.buffer & mask) >> self.bit_pos;

            value |= extracted << (bits - bits_remaining);
            
            bits_remaining -= bits_to_read;
            self.bit_pos += bits_to_read;
            self.bits_read += bits_to_read as u64;
        }

        return Ok(value);
    }

    pub fn read_bits_with_continuation_bit(&mut self, data_bits_per_chunk: u32) -> anyhow::Result<u64> {
        let mut return_value = 0;
        let mut bits_read = 0;
        let mask = (1 << data_bits_per_chunk) - 1;

        loop {
            let continuation_bit: bool = self.read()?;
            let value = self.read_bits(data_bits_per_chunk);
            
            return_value |= (value? & mask) << bits_read;
            bits_read += data_bits_per_chunk;
            if !continuation_bit {
                return Ok(return_value);
            }
        }
    }

}

#[cfg(test)]
mod tests;