use std::io::{BufWriter, Write};

use crate::bit_traits::BitWritable;


pub struct BitWriter<W: Write> {
    writer: BufWriter<W>,
    pub bits_written: u64,
    buffer: u64,
    bit_pos: u32,
    pub default_chunk_size: u32, // this is here because I haven't bothered to think of a better way to have somethting like this variable in the BitWritable implementations
}

impl<W: Write> BitWriter<W> {
    pub fn write<T: BitWritable>(&mut self, value: &T) -> anyhow::Result<()> {
        value.write(self)
    }

    /// * `inner` - The inner writer that the bitwriter will write to
    pub fn new(inner: W, default_chunk_size: Option<u32>) -> Self {
        let default_chunk_size = default_chunk_size.unwrap_or(10);
        
        BitWriter { writer: BufWriter::new(inner), bits_written: 0, buffer: 0, bit_pos: 0, default_chunk_size }
    }

    /// Writes an unsigned integer with the amount of bits specified
    /// * `bits` - How many bits to write to the stream
    /// * `value` - The value to write with the amount of bits specified
   pub fn write_bits(&mut self, bits: u32, value: u64) -> anyhow::Result<()> {
        let mut bits_remaining = bits;

        while bits_remaining > 0 {
            let bits_to_write = (u64::BITS - self.bit_pos).min(bits_remaining);

            let offset = bits - bits_remaining;
            let mask: u64 = ((1 << bits_to_write) - 1) << offset; // creates the mask and shifts it to the required position
            let extracted = ((value & mask) >> offset) as u64;

            self.buffer |= extracted << self.bit_pos;
            
            bits_remaining -= bits_to_write;
            self.bits_written += bits_to_write as u64;
            self.bit_pos += bits_to_write;
            if self.bit_pos == 64 {
                self.writer.write_all(&self.buffer.to_le_bytes())?;
                self.buffer = 0;
                self.bit_pos = 0;
            }
        }
        Ok(())
    }

    /// Writes an unsigned integer in a variable length format. 
    /// * `data_bits_per_chunk` - How many bits of data to write before checking for a continuation bit
    /// * `value` - The value to write to the stream
    pub fn write_bits_with_continuation_bit(&mut self, data_bits_per_chunk: u32, value: u64) -> anyhow::Result<()> {
        let mut value = value;
        let mask = (1 << data_bits_per_chunk) - 1;
        loop {
            let chunk = value & mask;
            value >>= data_bits_per_chunk;
            let continuation_bit = value > 0;
            self.write(&continuation_bit)?;
            self.write_bits(data_bits_per_chunk, chunk)?;
            if !continuation_bit {
                break;
            }
        }
        Ok(())
    }

    /// Flushes the bits in the buffer to the output stream
    pub fn flush(&mut self) -> anyhow::Result<()> {
        let byte_count = (self.bits_written % 64).div_ceil(8);
        let bytes: Vec<u8> = self.buffer.to_le_bytes().iter().take(byte_count as usize).map(|b|*b).collect();

        self.writer.write_all(bytes.as_slice())?;
        self.buffer = 0;
        self.bit_pos = 0;

        Ok(self.writer.flush()?)
    }
}

#[cfg(test)]
mod tests;