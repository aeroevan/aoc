use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    const FNAME: &str = "input.txt";
    let file = File::open(FNAME).expect(&format!("Couldn't open {}", FNAME));
    let reader = BufReader::new(&file);
    let vals: Vec<i32> = reader
        .lines()
        .map(|l| {
            l.expect("Couldn't read line")
                .parse::<i32>()
                .expect("Unknown input string")
        }).collect();
    // part 1
    println!("{}", vals.iter().sum::<i32>());
    // part 2
    let mut set: HashSet<i32> = HashSet::new();
    set.insert(0);
    let part2: Option<i32> = vals
        .iter()
        .cycle()
        .scan(0, |c, v| {
            *c += v;
            Some(*c)
        }).filter(|f| !set.insert(*f))
        .next();
    println!("{}", part2.expect("No second element found"));
}
