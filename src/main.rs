use std::{collections::HashMap, fs::File, io::{BufReader, BufWriter, Read, Write}, path::Path};

struct Pair {
    index: u64,
    symbol: Option<u8>,
}

const U8MAX: u64 = 2_u64.pow(8);
const U16MAX: u64 = 2_u64.pow(16);
const U32MAX: u64 = 2_u64.pow(32);

impl Pair {
    fn encode(&self) -> Vec<u8> {
        /* let mut bytes: Vec<u8> = match self.index {
            0..U8MAX => (self.index as u8).to_be_bytes().to_vec(),
            U8MAX..U16MAX => (self.index as u16).to_be_bytes().to_vec(),
            U16MAX..U32MAX => (self.index as u32).to_be_bytes().to_vec(),
            U32MAX..=u64::MAX => self.index.to_be_bytes().to_vec(),
        }; */

        let mut bytes: Vec<u8> = self.index.to_be_bytes().to_vec();
        if let Some(byte) = self.symbol {
            bytes.push(byte);
        }
        bytes
    }
}

fn main() {
    encode(Path::new("./TestFiles/test2.txt"), Path::new("output.lz78"));
}

fn encode(input: &Path, output: &Path) {
    let input_file = File::open(input).unwrap();
    let reader = BufReader::new(input_file);

    let output_file = File::create(output).unwrap();
    let mut writer = BufWriter::new(output_file);

    let mut dictionary: HashMap<Vec<u8>, u64> = HashMap::new();
    let mut buffer: Vec<u8> = Vec::new();
    let mut index: u64 = 0;

    for byte in reader.bytes() {
        let b = byte.unwrap();
        let prev_index = if buffer.len() > 0 {dictionary[&buffer]} else {0};
        buffer.push(b);
        if !dictionary.contains_key(&buffer) {
            index += 1;
            dictionary.insert(buffer, index);
            buffer = Vec::new();
            let pair = Pair {index: prev_index, symbol: Some(b)};
            writer.write_all(&pair.encode()).unwrap();
        }
    }

    if buffer.len() > 0 { //EOF
        let index = dictionary[&buffer];
        writer.write_all(&index.to_be_bytes()).unwrap();
    }

    writer.flush().unwrap();
}

fn decode() {

}