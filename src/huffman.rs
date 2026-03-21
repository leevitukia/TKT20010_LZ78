use std::{cmp::{Ordering, Reverse}, collections::{BinaryHeap, HashMap}, fs::File, io::{BufReader, Read}, path::Path};


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

pub fn encode(input: &Path, output: &Path) {
    let input_file = File::open(input).unwrap();
    let reader = BufReader::new(input_file);
    let mut frequencies: HashMap<u8, usize> = HashMap::with_capacity(256);
    for byte in reader.bytes(){
        let b = byte.expect("Failed to read a byte for some reason");
        *frequencies.entry(b).or_insert(0) += 1;
    }

    let mut prio_queue = BinaryHeap::with_capacity(frequencies.len());

    for kvp in frequencies{
        prio_queue.push(Reverse(Node::Leaf { symbol: kvp.0, freq: kvp.1 }));
    }

    while prio_queue.len() > 1 {
        let left_node = Box::new(prio_queue.pop().unwrap().0);
        let right_node = Box::new(prio_queue.pop().unwrap().0);

        let new_node = Node::Internal { freq: left_node.freq() + right_node.freq(), left: left_node, right: right_node };

        prio_queue.push(Reverse(new_node));
    }

    let root = prio_queue.pop().unwrap().0;

    todo!()
}

pub fn decode(input: &Path, output: &Path) {
    todo!()
}
