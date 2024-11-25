use std::collections::{BinaryHeap, HashMap};
use std::cmp::Ordering;
use std::fs::{self, File};
use clap::{Arg, ArgAction, Command};
use std::io::{self, Read, Write};
use serde_json;


#[derive(Debug, Eq, PartialEq)]
struct HuffmanNode {
    frequency: usize,
    character: Option<char>,
    left: Option<Box<HuffmanNode>>,
    right: Option<Box<HuffmanNode>>,
}

impl HuffmanNode {
    fn new(frequency: usize, character: Option<char>) -> Self {
        HuffmanNode {
            frequency,
            character,
            left: None,
            right: None,
        }
    }

    // fn is_leaf(&self) -> bool {
    //     self.character.is_some()
    // }
}

impl Ord for HuffmanNode {
    fn cmp(&self, other: &Self) -> Ordering {
        other.frequency.cmp(&self.frequency)
    }
}

impl PartialOrd for HuffmanNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn build_huffman_tree(frequencies: &HashMap<char, usize>) -> HuffmanNode {
    let mut heap = BinaryHeap::new();

    for (&character, &frequency) in frequencies {
        heap.push(HuffmanNode::new(frequency, Some(character)));
    }

    while heap.len() > 1 {
        let left = heap.pop().unwrap();
        let right = heap.pop().unwrap();

        let mut internal_node = HuffmanNode::new(left.frequency + right.frequency, None);
        internal_node.left = Some(Box::new(left));
        internal_node.right = Some(Box::new(right));

        heap.push(internal_node);
    }

    heap.pop().unwrap()
}

fn build_codes(node: &HuffmanNode, prefix: String, codes: &mut HashMap<char, String>) {
    if let Some(character) = node.character {
        codes.insert(character, prefix);
    } else {
        if let Some(ref left) = node.left {
            build_codes(left, format!("{}0", prefix), codes);
        }
        if let Some(ref right) = node.right {
            build_codes(right, format!("{}1", prefix), codes);
        }
    }
}

fn huffman_compress(input: &str) -> (String, HashMap<char, String>) {
    let mut frequencies = HashMap::new();
    for character in input.chars() {
        *frequencies.entry(character).or_insert(0) += 1;
    }

    let root = build_huffman_tree(&frequencies);

    let mut codes = HashMap::new();
    build_codes(&root, String::new(), &mut codes);

    let compressed = input.chars()
        .map(|c| codes[&c].clone())
        .collect::<Vec<String>>()
        .join("");

    (compressed, codes)
}

fn huffman_decompress(compressed: &str, codes: &HashMap<char, String>) -> String {
    let mut reverse_codes = HashMap::new();
    for (char, code) in codes {
        reverse_codes.insert(code.clone(), char);
    }

    let mut result = String::new();

    for bit in compressed.chars() {
         if let Some(&char) = reverse_codes.get(&bit.to_string()) {
            result.push(*char);
        }
    }

    result
}

fn main() -> io::Result<()> {
    let matches = Command::new("Huffman Compressor")
        .version("1.0")
        .author("Anton")
        .about("Compress and decompress files using Huffman algorithm")
        .arg(
            Arg::new("file")
                .short('f')
                .long("file")
                .value_name("INPUT")
                .help("Input file path")
                .required(true),
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .value_name("OUTPUT")
                .help("Output file path")
                .required(true),
        )
        .arg(
            Arg::new("compress")
                .short('c')
                .help("Compress the input file")
                .conflicts_with("decompress")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("decompress")
                .short('u')
                .help("Decompress the input file")
                .conflicts_with("compress")
                .action(ArgAction::SetFalse),
        )
        .get_matches();

    let input = matches.get_one::<String>("file").expect("Input file is required");
    let content = fs::read_to_string(input)?;
    let output = matches.get_one::<String>("output").expect("Output file is required");

    if matches.get_one::<bool>("compress").is_some() {
        let (compressed, codes) = huffman_compress(&content);

        let codes_serialized = serde_json::to_string(&codes)?;
        fs::write(output, format!("{}\n{}", compressed, codes_serialized))?;
    } else if matches.get_one::<bool>("decompress").is_some() {
        let content_parts: Vec<&str> = input.splitn(2, '\n').collect();
        let compressed = content_parts[0].to_string();
        let codes: HashMap<char, String> = serde_json::from_str(content_parts[1])?;
        let decompressed = huffman_decompress(&compressed, &codes);
        fs::write(output, decompressed)?;
    } else {
        eprintln!("Please specify either --compress (-c) or --decompress (-u).");
    }

    Ok(())
}