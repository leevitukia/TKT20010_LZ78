use std::{cmp::{Ordering, Reverse}, collections::BinaryHeap, io::{BufReader, BufWriter, Read, Seek, SeekFrom, Write}};
use ahash::AHashMap;

use crate::{bitreader::BitReader, bitwriter::BitWriter};

#[derive(Debug, Clone, PartialEq, Eq)]
enum Node {
    Leaf { symbol: u8, freq: usize },
    Internal { freq: usize, left: Box<Node>, right: Box<Node> },
}

impl Node {
    fn freq(&self) -> usize {
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

fn build_tree(frequencies: &mut AHashMap<u8, usize>) -> AHashMap<u8, Vec<bool>>{
    let mut prio_queue = BinaryHeap::with_capacity(frequencies.len());

    for kvp in frequencies{
        prio_queue.push(Reverse(Node::Leaf { symbol: *kvp.0, freq: *kvp.1 }));
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
    let mut frequencies: AHashMap<u8, usize> = AHashMap::with_capacity(256);

    for byte in reader.bytes(){
        let b = byte.expect("Failed to read a byte for some reason");
        *frequencies.entry(b).or_insert(0) += 1;
    }

    let codes = build_tree(&mut frequencies);

    let mut writer = BitWriter::new(output, None);

    // write frequencies so that the decoder can rebuild the tree
    for i in 0..=255 {
        writer.write_bits_with_continuation_bit(32, frequencies[&i] as u64)?;
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



pub fn decode<R: Read + Seek, W: Write>(mut input: R, output: W) -> anyhow::Result<()>{
    let mut reader = BitReader::new(&mut input, None);
    let mut frequencies: AHashMap<u8, usize> = AHashMap::with_capacity(256);

    for i in 0..=255 {
        frequencies.insert(i, reader.read_bits_with_continuation_bit(32)? as usize);
    }

    let codes = build_tree(&mut frequencies);

    let prefix_to_symbol: AHashMap<Vec<bool>, u8> = codes.iter().map(|(k, v)| (v.clone(), *k)).collect();

    let mut prefix: Vec<bool> = Vec::new();

    let mut writer = BufWriter::new(output);
    loop {
        if let Ok(bit) = reader.read::<bool>() {
            prefix.push(bit);

            if let Some(symbol) = prefix_to_symbol.get(&prefix) {
                writer.write_all(&[*symbol])?;
                prefix.clear();
            }
        }
        else {
            break;
        }
    }
    writer.flush()?;
    Ok(())
}
