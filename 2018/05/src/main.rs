use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::FromIterator;
use std::time::Instant;

fn is_reacting(a: char, b: char) -> bool {
    // apparently aa doens't react
    a.eq_ignore_ascii_case(&b) && a != b
}

fn process_reaction(chars: &[char]) -> Vec<char> {
    chars.iter().fold(Vec::new(), |mut new_chars, c| {
        if is_reacting(*c, *new_chars.last().unwrap_or(&'!')) {
            new_chars.pop();
        } else {
            new_chars.push(*c);
        }
        new_chars
    })
}

fn collapsed_reaction(chars: &[char]) -> Vec<char> {
    let units: HashSet<char> = HashSet::from_iter(chars.iter().map(|c| c.to_ascii_lowercase()));
    units
        .iter()
        .map(|u| {
            let collapsed: Vec<char> = Vec::from_iter(
                chars
                    .iter()
                    .cloned()
                    .filter(|x| !u.eq_ignore_ascii_case(&x)),
            );
            process_reaction(collapsed.as_slice())
        }).min_by_key(|r| r.len())
        .expect("or not")
}

fn main() {
    const FNAME: &str = "input.txt";
    let file = File::open(FNAME).expect(&format!("Couldn't open {}", FNAME));
    let reader = BufReader::new(&file);
    let line: String = reader.lines().filter_map(|l| l.ok()).next().unwrap();
    let chars: Vec<char> = Vec::from_iter(line.chars());
    let t1 = Instant::now();
    let reacted = process_reaction(chars.as_slice());
    let t2 = Instant::now();
    println!("{}", reacted.len());
    println!("{:?}", t2 - t1);
    let t3 = Instant::now();
    let collapsed = collapsed_reaction(reacted.as_slice());
    let t4 = Instant::now();
    println!("{}", collapsed.len());
    println!("{:?}", t4 - t3);
}
