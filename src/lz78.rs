use std::{io::{BufReader, BufWriter, Read, Seek, SeekFrom, Write}};

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
        writer.write_bits_with_continuation_bit(writer.default_chunk_size, self.index as u64)?;
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
        Ok(Pair::new(index, symbol))
    }
}

/* fn chunk_size_heuristic(bytes: u64) -> u32 {
    if bytes < 2 {
        1
    }
    else {
        let bits = bytes.ilog2() + 1;
        (bits as f64 / 4.5 * 2.0) as _// replace with something smart
    }
} */

#[derive(Debug, Clone, PartialEq, Eq)]
struct TrieNode {
    pub children: [u64; 256],
    pub index: u64,
}

const MAX_DICT_SIZE: usize = 1 << 20;

/// Takes the input and writes an LZ78 encoded version to the output
/// * `input` - Any readable & seekable type, such as a File or a byte vector wrapped in a Cursor
/// * `output` - Any writable type, such as a File or a byte vector
pub fn encode<R: Read + Seek, W: Write>(mut input: R, output: W) -> anyhow::Result<()> {
    let len = input.seek(SeekFrom::End(0))?;
    input.seek(SeekFrom::Start(0))?;

    //let optimal_chunk_size: u32 = chunk_size_heuristic(len);

    let optimal_chunk_size: u32 = ((MAX_DICT_SIZE.ilog2() + 1) as f64 / 4.5 * 2.0) as u32;

    let mut reader = BufReader::new(input);
    let mut writer = BitWriter::new(output, Some(optimal_chunk_size));
    if len == 0 {
        writer.flush()?; //empty file
        return Ok(())
    }
    writer.write_bits(6, optimal_chunk_size as u64)?;

    //let mut dictionary: HashMap<Box<[u8]>, u64> = HashMap::new();
    //let mut buffer: Vec<u8> = Vec::new();
    //let mut index: u64 = 0;

    let mut nodes: Vec<TrieNode> = vec![TrieNode { children: [0;256], index: 0 }];
    let mut curr_index = 0;
    let mut prev_index = 0;
    let mut index = 0;

    let mut byte_buffer = [0u8; 8192];
    loop {
        let n = reader.read(&mut byte_buffer)?;
        if n == 0 {
            break;
        }
        for b in &byte_buffer[..n] {
            if nodes[curr_index].children[*b as usize] != 0 {
                curr_index = nodes[curr_index].children[*b as usize] as usize;
                prev_index = nodes[curr_index].index;
            } else {
                index += 1;
                let next_index = nodes.len() as u64;
                nodes[curr_index].children[*b as usize] = next_index;
                //nodes[curr_index].children.insert(*b, next_index);
                nodes.push(TrieNode { children: [0;256], index });
                curr_index = 0;

                let pair = Pair { index: prev_index, symbol: Some(*b) };
                writer.write(&pair)?;
                prev_index = 0;

                if nodes.len() >= MAX_DICT_SIZE { //dictionary reset
                    nodes.clear();
                    nodes.push(TrieNode { children: [0;256], index: 0 });
                    index = 0;
                    prev_index = 0;
                }
            }
        }
    }

    if curr_index != 0 {
        let pair = Pair { index: prev_index, symbol: None };
        writer.write(&pair)?;
    }

    writer.flush()
}

/// Takes the input and writes the decoded version to the output
/// * `input` - Any readable & seekable type, such as a File or a byte vector wrapped in a Cursor
/// * `output` - Any writable type, such as a File or a byte vector
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

    while let Ok(pair) = reader.read::<Pair>() {
        let mut buffer = Vec::new();
        if pair.index > 0 {
            buffer.extend(&dictionary[pair.index as usize - 1]);
        }

        if let Some(symbol) = pair.symbol {
            buffer.push(symbol);
        }
        writer.write_all(&buffer)?;

        dictionary.push(buffer);

        if dictionary.len() >= MAX_DICT_SIZE - 1 { //dictionary reset
            dictionary.clear();
        }
    }
    
    writer.flush()?;
    Ok(())
}

#[cfg(test)]
mod tests;