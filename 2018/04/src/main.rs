#[macro_use]
extern crate lazy_static;

extern crate regex;
use regex::Regex;

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Range;

fn parse_minute(line: &str) -> u8 {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^\[\d{4}-\d{2}-\d{2} \d{2}:(?P<m>\d{2})\].*").unwrap();
    }
    let caps = RE.captures(line).unwrap();
    let minute: u8 = caps["m"].parse().expect("Couldn't parse top");
    minute
}

#[derive(Debug, Clone)]
struct Shift {
    id: u32,
    sleeping: Vec<Range<u8>>,
}

fn parse_shifts(lines: Vec<String>) -> Vec<Shift> {
    let mut shifts: Vec<Shift> = Vec::new();
    let mut guard: u32 = 0;
    let mut start: u8 = 0;
    let mut sleeping: Vec<Range<u8>> = Vec::new();
    for line in &lines {
        if line.contains("Guard") {
            lazy_static! {
                static ref GRE: Regex = Regex::new(r".*Guard #(?P<id>\d+) begins shift").unwrap();
            }
            let caps = GRE.captures(line).unwrap();
            if sleeping.len() > 0 {
                shifts.push(Shift {
                    id: guard,
                    sleeping: sleeping.clone(),
                });
                sleeping.clear();
            }
            guard = caps["id"].parse().expect("couldn't find guard id");
        } else if line.contains("falls asleep") {
            start = parse_minute(line);
        } else if line.contains("wakes up") {
            let end = parse_minute(line);
            sleeping.push(start..end);
        } else {
            eprintln!("Unknown line type: {}", line);
        }
    }
    shifts
}

#[derive(Debug, Clone)]
struct Guard {
    id: u32,
    total: u32,
    time_asleep: HashMap<u8, u32>,
}
impl Guard {
    fn most_asleep(&self) -> (u8, u32) {
        let mut val: u32 = 0;
        let mut min: u8 = 0;
        for (m, v) in &self.time_asleep {
            if v > &val {
                val = *v;
                min = *m;
            }
        }
        (min, val)
    }
}

fn combine_shifts(shifts: Vec<Shift>) -> Vec<Guard> {
    let mut guards: Vec<Guard> = Vec::new();
    let mut shifts_per_guard: HashMap<u32, Vec<Shift>> = HashMap::new();
    for shift in &shifts {
        let id: u32 = shift.id;
        shifts_per_guard
            .entry(id)
            .or_insert(Vec::new())
            .push(shift.clone());
    }
    for (id, id_shifts) in &shifts_per_guard {
        let mut total: u32 = 0;
        let mut time_asleep: HashMap<u8, u32> = HashMap::new();
        for shift in id_shifts {
            for sleep in &shift.sleeping {
                for m in sleep.clone() {
                    total += 1;
                    *time_asleep.entry(m).or_insert(0) += 1;
                }
            }
        }
        guards.push(Guard {
            id: id.clone(),
            total: total,
            time_asleep: time_asleep,
        });
    }
    guards
}

fn main() {
    const FNAME: &str = "input.txt";
    let file = File::open(FNAME).expect(&format!("Couldn't open {}", FNAME));
    let reader = BufReader::new(&file);
    let mut lines: Vec<String> = reader.lines().filter_map(|l| l.ok()).collect();
    lines.sort_unstable();
    let shifts: Vec<Shift> = parse_shifts(lines);
    let guards: Vec<Guard> = combine_shifts(shifts);
    let max_time: Guard = guards.iter().max_by_key(|x| x.total).unwrap().clone();
    //println!("{:?}", max_time);
    println!("{}", max_time.id * (max_time.most_asleep().0 as u32));
    let max_minute: Guard = guards
        .iter()
        .max_by_key(|x| x.most_asleep().1)
        .unwrap()
        .clone();
    //println!("{:?}", max_minute);
    println!("{}", max_minute.id * (max_minute.most_asleep().0 as u32));
}
