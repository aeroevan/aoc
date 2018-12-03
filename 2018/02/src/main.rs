use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn charmap(code: &str) -> HashMap<char, u8> {
    let mut freq: HashMap<char, u8> = HashMap::new();
    for c in code.chars() {
        *freq.entry(c).or_insert(0) += 1;
    }
    freq
}

fn twochars(code: &str) -> bool {
    let freq = charmap(code);
    freq.values().cloned().filter(|x| *x == 2).count() > 0
}

fn threechars(code: &str) -> bool {
    let freq = charmap(code);
    freq.values().cloned().filter(|x| *x == 3).count() > 0
}

fn diff(code1: &str, code2: &str) -> usize {
    code1
        .chars()
        .zip(code2.chars())
        .filter(|(c1, c2)| c1 != c2)
        .count()
}

fn main() {
    const FNAME: &str = "input.txt";
    let file = File::open(FNAME).expect(&format!("Couldn't open {}", FNAME));
    let reader = BufReader::new(&file);
    let codes: Vec<String> = reader.lines().filter_map(|l| l.ok()).collect();
    // part 1
    let n2 = codes.iter().filter(|c| twochars(c)).count();
    let n3 = codes.iter().filter(|c| threechars(c)).count();
    println!("{}", n2 * n3);
    // part 2
    'outer: for code1 in &codes {
        for code2 in &codes {
            if code1 != code2 {
                if diff(code1, code2) == 1 {
                    // apparently the order mattered, so couldn't use set intersection
                    let inter = code1
                        .chars()
                        .zip(code2.chars())
                        .filter(|(c1, c2)| c1 == c2)
                        .map(|(c1, _c2)| c1)
                        .fold(String::new(), |mut acc, c| {
                            acc.push(c);
                            acc
                        });
                    println!("{}", inter);
                    break 'outer;
                }
            }
        }
    }
}
