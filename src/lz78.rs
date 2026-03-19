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
        let bit_count: u8 = match self.index {
            0..U8MAX => 8,
            U8MAX..U16MAX => 16,
            U16MAX..U32MAX => 32,
            U32MAX..=u64::MAX => 64,
        };
        let mut encoded_bytes = Vec::with_capacity((bit_count / 8 + 2) as usize);
        encoded_bytes.push(bit_count);

        let bytes = match self.index {
            0..U8MAX => (self.index as u8).to_be_bytes().to_vec(),
            U8MAX..U16MAX => (self.index as u16).to_be_bytes().to_vec(),
            U16MAX..U32MAX => (self.index as u32).to_be_bytes().to_vec(),
            U32MAX..=u64::MAX => self.index.to_be_bytes().to_vec(),
        };

        encoded_bytes.extend(bytes);

        if let Some(byte) = self.symbol {
            encoded_bytes.push(byte);
        }
        encoded_bytes
    }
}


pub fn encode(input: &Path, output: &Path) {
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
        let pair = Pair { index, symbol: None };
        writer.write_all(&pair.encode()).unwrap();
    }

    writer.flush().unwrap();
}



pub fn decode(input: &Path, output: &Path) {
    let input_file = File::open(input).unwrap();
    let mut reader = BufReader::new(input_file);

    let output_file = File::create(output).unwrap();
    let mut writer = BufWriter::new(output_file);

    let mut dictionary: Vec<Vec<u8>> = Vec::new();

    loop {
        let mut buf = [0u8; 1];
        if let Err(_) = reader.read_exact(&mut buf) {
            break;
        };
        let bit_count = buf[0];
        let mut buf = vec![0u8; (bit_count / 8) as usize];
        reader.read_exact(&mut buf).expect("Failed to read the bit count header");

        let index = match bit_count {
            8 => u8::from_be_bytes(buf[..].try_into().unwrap()) as u64,
            16 => u16::from_be_bytes(buf[..].try_into().unwrap()) as u64,
            32 => u32::from_be_bytes(buf[..].try_into().unwrap()) as u64,
            64 => u64::from_be_bytes(buf[..].try_into().unwrap()),
            _ => panic!("Something went horribly wrong {}", bit_count),
        };
        let mut buf = [0u8; 1];
        let symbol = if let Ok(_) = reader.read_exact(&mut buf) { Some(buf[0]) } else { None };

        let mut buffer = Vec::new();
        if index > 0 {
            buffer.extend(&dictionary[index as usize - 1]);
        }
        
        if let Some(symbol) = symbol {
            buffer.push(symbol);
        }

        writer.write_all(&buffer).expect("Failed to write buffer to file");

        dictionary.push(buffer);
    }

    writer.flush().expect("Failed to finish writing the output file");

}