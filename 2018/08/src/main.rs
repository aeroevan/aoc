use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::FromIterator;

#[derive(Debug)]
struct Node {
    children: Vec<Node>,
    metadata: Vec<u8>,
}

impl Node {
    fn metadata_sum(&self) -> u64 {
        self.metadata.iter().map(|m| u64::from(*m)).sum::<u64>()
    }
    fn total_metadata(&self) -> u64 {
        let mut total: u64 = self.metadata_sum();
        for n in &self.children {
            total += n.total_metadata();
        }
        total
    }
    fn value(&self) -> u64 {
        if self.children.is_empty() {
            self.metadata_sum()
        } else {
            let mut total: u64 = 0;
            for idx in &self.metadata {
                let i: usize = *idx as usize;
                if i > 0 && i < self.children.len() + 1 {
                    total += &self.children[i - 1].value();
                }
            }
            total
        }
    }
}

fn build_node(numbers: &[u8]) -> (Node, Vec<u8>) {
    let mut header_iter = numbers.iter().cloned();
    let num_children: u8 = header_iter.next().unwrap();
    let num_metadata: u8 = header_iter.next().unwrap();
    let mut new_numbers: Vec<u8> = Vec::from_iter(header_iter);
    let mut children: Vec<Node> = Vec::new();
    for _ in 0..num_children {
        let (node, updated_numbers) = build_node(new_numbers.as_slice());
        new_numbers = updated_numbers;
        children.push(node);
    }
    let mut metadata_iter = new_numbers.iter().cloned();
    let mut metadata: Vec<u8> = Vec::new();
    for _ in 0..num_metadata {
        metadata.push(metadata_iter.next().unwrap());
    }
    new_numbers = Vec::from_iter(metadata_iter);
    (Node { children, metadata }, new_numbers)
}

fn main() {
    const FNAME: &str = "input.txt";
    let file = File::open(FNAME).expect(&format!("Couldn't open {}", FNAME));
    let reader = BufReader::new(&file);
    let line: String = reader.lines().filter_map(|l| l.ok()).next().unwrap();
    let numbers: Vec<u8> =
        Vec::from_iter(line.split(' ').map(|v| v.parse().expect("not a number?")));
    let (node, _) = build_node(numbers.as_slice());
    println!("{}", node.total_metadata());
    println!("{}", node.value());
}
