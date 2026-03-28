use std::{cmp::{Ordering, Reverse}, collections::BinaryHeap, io::{BufReader, BufWriter, Read, Seek, SeekFrom, Write}};
use ahash::AHashMap;

use crate::{bitreader::BitReader, bitwriter::BitWriter};

#[derive(Debug, Clone, PartialEq, Eq)]
enum Node {
    Leaf { symbol: u8, freq: u64 },
    Internal { freq: u64, left: Box<Node>, right: Box<Node> },
}

impl Node {
    fn freq(&self) -> u64 {
        match self {
            Node::Leaf { freq, .. } => *freq,
            Node::Internal { freq, .. } => *freq,
        }
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.freq().cmp(&other.freq()) 
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn build_codes(node: &Node, prefix: Vec<bool>, codes: &mut AHashMap<u8, Vec<bool>>) {
    match node {
        Node::Leaf { symbol, .. } => { codes.insert(*symbol, prefix); }
        Node::Internal { left, right, .. } => {
            let mut left_prefix = prefix.clone();
            left_prefix.push(false); // 0
            build_codes(left, left_prefix, codes);

            let mut right_prefix = prefix;
            right_prefix.push(true); // 1
            build_codes(right, right_prefix, codes);
        }
    }
}

fn build_tree(frequencies: &mut AHashMap<u8, u64>) -> AHashMap<u8, Vec<bool>>{
    let mut prio_queue = BinaryHeap::with_capacity(frequencies.len());
    let mut sorted_freqs: Vec<(u8, u64)> = frequencies.iter().map(|f|(*f.0, *f.1)).collect();
    sorted_freqs.sort();

    for kvp in sorted_freqs{
        prio_queue.push(Reverse(Node::Leaf { symbol: kvp.0, freq: kvp.1 }));
    }

    while prio_queue.len() > 1 { // combines the 2 least common nodes until there's a single root node
        let left_node = Box::new(prio_queue.pop().unwrap().0);
        let right_node = Box::new(prio_queue.pop().unwrap().0);

        let new_node = Node::Internal { freq: left_node.freq() + right_node.freq(), left: left_node, right: right_node };

        prio_queue.push(Reverse(new_node));
    }

    let root = prio_queue.pop().unwrap().0;
    
    let mut codes: AHashMap<u8, Vec<bool>> = AHashMap::new();

    build_codes(&root, Vec::new(), &mut codes);

    codes
}

pub fn encode<R: Read + Seek, W: Write>(mut input: R, output: W) -> anyhow::Result<()> {
    let reader = BufReader::new(&mut input);
    let mut frequencies: AHashMap<u8, u64> = (0..=255).map(|i| (i, 0)).collect();

    for byte in reader.bytes(){
        let b = byte.expect("Failed to read a byte for some reason");
        *frequencies.entry(b).or_insert(0) += 1;
    }
    
    if frequencies.iter().map(|f| *f.1).sum::<u64>() == 0 {
        return Ok(())
    }

    let codes = build_tree(&mut frequencies);

    // write frequencies so that the decoder can rebuild the tree
    let optimal_chunk_size = (4..=63).min_by_key(|c| { // brute force the optimal chunk size
        let mut bits_written = 0;
        for i in 0..=255 {
            let mut bits_remaining = frequencies[&i].max(1).ilog2() + 1;
            
            while bits_remaining != 0 {
                bits_written += c + 1;
                bits_remaining = bits_remaining.saturating_sub(*c);
            }
        };
        bits_written
    }).unwrap_or(32);

    let mut writer = BitWriter::new(output, None);

    writer.write_bits(6, optimal_chunk_size as u64)?;

    for i in 0..=255 {
        writer.write_bits_with_continuation_bit(optimal_chunk_size, frequencies[&i] as u64)?;
    }

    input.seek(SeekFrom::Start(0))?;
    let reader = BufReader::new(input);

    for byte in reader.bytes(){
        for bit in &codes[&byte?] {
            writer.write(bit)?;
        }
    }

    writer.flush()?;
    Ok(())
}

pub fn decode<R: Read, W: Write>(mut input: R, output: W) -> anyhow::Result<()>{
    let mut reader = BitReader::new(&mut input, None);
    let mut frequencies: AHashMap<u8, u64> = (0..=255).map(|i| (i, 0)).collect();

    let chunk_size = reader.read_bits(6)?;

    for i in 0..=255 {
        frequencies.insert(i, reader.read_bits_with_continuation_bit(chunk_size as _)? as _);
    }

    let codes = build_tree(&mut frequencies);

    let prefix_to_symbol: AHashMap<Vec<bool>, u8> = codes.iter().map(|(k, v)| (v.clone(), *k)).collect();

    let mut prefix: Vec<bool> = Vec::new();

    let mut writer = BufWriter::new(output);

    let total_symbols = frequencies.iter().map(|f|*f.1).sum::<u64>();

    let mut symbols_written = 0;
    while let Ok(bit) = reader.read::<bool>() && symbols_written < total_symbols {
        prefix.push(bit);

        if let Some(symbol) = prefix_to_symbol.get(&prefix) {
            writer.write_all(&[*symbol])?;
            prefix.clear();
            symbols_written += 1;
        }
    }

    writer.flush()?;
    Ok(())
}

#[cfg(test)]
mod tests;