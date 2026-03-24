use std::io::{BufWriter, Write};

use crate::bit_traits::BitWritable;


pub struct BitWriter<W: Write> {
    writer: BufWriter<W>,
    pub bits_written: u64,
    buffer: u8,
}

impl<W: Write> BitWriter<W> {
    pub fn write<T: BitWritable>(&mut self, value: &T) -> anyhow::Result<()> {
        value.write(self)
    }

    pub fn new(inner: W) -> Self {
        BitWriter { writer: BufWriter::new(inner), bits_written: 0, buffer: 0 }
    }

    /* pub fn write_bits(&mut self, bits: u64, value: u64) {
        while self.buffer.len() < ((self.bits_written + bits) as usize).div_ceil(8) {
            self.buffer.push(0);
        }
        let mut bits_remaining = bits;

        while bits_remaining > 0 {
            let index = (self.bits_written / 8) as usize;
            let bit_pos = self.bits_written % 8;

            let offset = bits - bits_remaining;

            let extracted_bits = (8-bit_pos).min(bits_remaining);

            let mask: u64 = ((1 << extracted_bits) - 1) << offset;
            let extracted = ((value & mask) >> offset) as u8;

            self.buffer[index] |= extracted << bit_pos;
            
            //println!("Bits Written: {}  Bits Remaining: {}  Bit Pos {}", self.bits_written, bits_remaining, bit_pos);
            
            bits_remaining -= extracted_bits;
            self.bits_written += extracted_bits;
        }

    } */

   pub fn write_bits(&mut self, bits: u64, value: u64) -> anyhow::Result<()> {
        let mut bits_remaining = bits;

        while bits_remaining > 0 {
            let bit_pos = self.bits_written % 8;
            let extracted_bits = (8-bit_pos).min(bits_remaining);

            let offset = bits - bits_remaining;
            let mask: u64 = ((1 << extracted_bits) - 1) << offset;
            let extracted = ((value & mask) >> offset) as u8;

            self.buffer |= extracted << bit_pos;
            
            //println!("Bits Written: {}  Bits Remaining: {}  Bit Pos {}", self.bits_written, bits_remaining, bit_pos);
            
            bits_remaining -= extracted_bits;
            self.bits_written += extracted_bits;

            if self.bits_written % 8 == 0 {
                self.writer.write_all(&[self.buffer])?;
                self.buffer = 0;
            }
        }
        Ok(())
    }

    pub fn flush(&mut self) -> anyhow::Result<()> {
        if self.bits_written % 8 != 0 {
            self.writer.write_all(&[self.buffer])?;
            self.buffer = 0;
        }
        let padding = 8 - self.writer.buffer().len() % 8;
        for _ in 0..padding {
            self.writer.write_all(&[0u8])?;
        }

        Ok(self.writer.flush()?)
    }
}