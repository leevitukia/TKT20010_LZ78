use std::{collections::HashMap, fs::File, io::{BufReader, BufWriter, Read, Write}};

struct Pair {
    index: u64,
    symbol: u8,
}

impl Pair {
    fn encode(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = self.index.to_be_bytes().to_vec();
        bytes.push(self.symbol);
        bytes
    }
}

fn main() {
    let file = File::open("./TestFiles/test2.txt").unwrap();
    let reader = BufReader::new(file);

    let mut dictionary: HashMap<Vec<u8>, u64> = HashMap::new();
    let mut buffer: Vec<u8> = Vec::new();
    let mut index: u64 = 0;
    //let mut output: Vec<Pair> = Vec::new();

    let output_file = File::create("output.lz78").unwrap();
    let mut writer = BufWriter::new(output_file);

    for byte in reader.bytes() {
        let b = byte.unwrap();
        buffer.push(b);
        if !dictionary.contains_key(&buffer) {
            index += 1;
            dictionary.insert(buffer, index);
            buffer = Vec::new();
            let pair = Pair {index: index - 1, symbol: b};
            writer.write_all(&pair.encode()).unwrap();
        }
    }

    writer.flush().unwrap();
}

/* fn encode() {

}

fn decode() {

} */