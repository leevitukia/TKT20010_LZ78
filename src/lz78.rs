use std::io::{BufReader, BufWriter, Read, Seek, SeekFrom, Write};

use ahash::AHashMap;

use crate::{bit_traits::{BitReadable, BitWritable}, bitreader::BitReader, bitwriter::BitWriter};

#[derive(Debug, Clone)]
pub struct Pair {
    /// The index of the longest matching prefix in the dictionary
    index: u64,
    /// The symbol that broke the buffer, only None if the last buffer was in the dictionary
    symbol: Option<u8>,
}

impl Pair {
    pub fn new(index: u64, symbol: Option<u8>) -> Self {
        Self {index, symbol}
    }
}
impl BitWritable for Pair {
    fn write<W: Write>(&self, writer: &mut BitWriter<W>) -> anyhow::Result<()> {
        writer.write_bits_with_continuation_bit(writer.default_chunk_size, self.index)?;
        //println!("Wrote: {}", self.index);
        if let Some(symbol) = self.symbol {
            writer.write(&true)?;
            writer.write_bits(8, symbol as u64)?;
        }
        else {
            writer.write(&false)?;
        }

        Ok(())
    }
}

impl BitReadable for Pair {
    fn read<R: Read>(reader: &mut BitReader<R>) -> anyhow::Result<Self> {
        let index = reader.read_bits_with_continuation_bit(reader.default_chunk_size)?;
        let mut symbol = None;
        if reader.read::<bool>()? {
            symbol = Some(reader.read_bits(8)? as u8);
        }
        return Ok(Pair::new(index, symbol));
    }
}

fn chunk_size_heuristic(bytes: u64) -> u32 {
    if bytes < 2 {
        return 1;
    }
    else {
        let bits = bytes.ilog2() + 1;
        return (bits as f64 / 4.5 * 2.0) as _; // replace with something smart
    }
}

/// This method takes the input file and saves a version compressed with the LZ78 algorithm to the output path
pub fn encode<R: Read + Seek, W: Write>(mut input: R, output: W) -> anyhow::Result<()> {
    let len = input.seek(SeekFrom::End(0))?;
    input.seek(SeekFrom::Start(0))?;

    let optimal_chunk_size: u32 = chunk_size_heuristic(len);

    let reader = BufReader::new(input);
    let mut writer = BitWriter::new(output, Some(optimal_chunk_size));
    if len == 0 {
        writer.flush()?; //empty file
        return Ok(())
    }
    writer.write_bits(6, optimal_chunk_size as u64)?;

    let mut dictionary: AHashMap<Box<[u8]>, u64> = AHashMap::new();
    let mut buffer: Vec<u8> = Vec::new();
    let mut index: u64 = 0;

    for byte in reader.bytes() {
        let b = byte.unwrap();
        let prev_index = if buffer.len() > 0 {dictionary[buffer.as_slice()]} else {0};
        buffer.push(b);
        if !dictionary.contains_key(buffer.as_slice()) {
            index += 1;
            dictionary.insert(buffer.clone().into_boxed_slice(), index);
            buffer.clear();
            let pair = Pair {index: prev_index, symbol: Some(b)};
            writer.write(&pair)?;
        }
    }

    if buffer.len() > 0 { //EOF
        let index = dictionary[buffer.as_slice()];
        let pair = Pair { index, symbol: None };
        writer.write(&pair)?;
    }

    Ok(writer.flush()?)
}

/// This method takes a file that was compressed with the LZ78 algorithm and decodes it to the output path
pub fn decode<R: Read, W: Write>(input: R, output: W) -> anyhow::Result<()> {
    let mut reader = BitReader::new(input, None);

    if let Ok(chunk_size ) = reader.read_bits(6){
        reader.default_chunk_size = chunk_size as u32;
    }
    else {
        return Ok(())
    };
    
    let mut writer = BufWriter::new(output);
    let mut dictionary: Vec<Vec<u8>> = Vec::new();

    loop {
        if let Ok(pair) = reader.read::<Pair>() {
            let mut buffer = Vec::new();
            if pair.index > 0 {
                buffer.extend(&dictionary[pair.index as usize - 1]);
            }

            if let Some(symbol) = pair.symbol {
                buffer.push(symbol);
            }
            writer.write_all(&buffer)?;

            dictionary.push(buffer);
        }
        else {
            break;
        }
    }

    writer.flush()?;
    Ok(())
}

#[cfg(test)]
mod tests;