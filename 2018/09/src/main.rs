#[macro_use]
extern crate lazy_static;
extern crate regex;
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::usize;

#[derive(Debug, Clone, Copy)]
struct Configuration {
    num_players: u16,
    last_marble: u64,
}
impl Configuration {
    fn from_line(line: &str) -> Configuration {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"^(?P<p>\d+) players; last marble is worth (?P<m>\d+) points").unwrap();
        }
        let caps = RE.captures(line).unwrap();
        let num_players: u16 = caps["p"].parse().expect("no current step");
        let last_marble: u64 = caps["m"].parse().expect("no previous step");
        Configuration {
            num_players,
            last_marble,
        }
    }
}

#[derive(Clone, Copy)]
struct Marble {
    value: u64,
    next: usize,
    prev: usize,
}

fn play(conf: Configuration) -> u64 {
    let mut players: Vec<u64> = vec![0; conf.num_players as usize];
    let mut circle: Vec<Marble> = vec![Marble{value: 0, next: 0, prev: 0}; conf.last_marble as usize];
    let mut curidx: usize = 0;
    let mut nextidx: usize = 1;

    for marble in 0..=conf.last_marble {
        if marble % 23 == 0 {
            // move next
            // 0 -> 0 first, next one we'll have initialized
            curidx = circle[curidx].next;
            // where current marble points
            let next = circle[curidx].next;
            // set new marble value
            circle[nextidx].value = marble;
            circle[nextidx].next = next;
            circle[nextidx].prev = curidx;
            // fix pointers to prev/next of new marble
            // now curidx is prev, and next is still next
            circle[curidx].next = nextidx;
            circle[next].prev = nextidx;
            curidx = nextidx;
            nextidx += 1;
        } else {
            // score player
            players[(marble % u64::from(conf.num_players)) as usize] += marble;
            // move back 7 times...
            for _ in 0..7 {
                curidx = circle[curidx].prev;
            }
            // add vale
            players[(marble % u64::from(conf.num_players)) as usize] += circle[curidx].value;
            // now "delete" by linking the prev/next marbles to each other
            let prev = circle[curidx].prev;
            let next = circle[curidx].next;
            circle[prev].next = next;
            circle[next].prev = prev;
            curidx = next;
        }
    }

    players.iter().cloned().max().unwrap()
}

fn main() {
    const FNAME: &str = "input.txt";
    let file = File::open(FNAME).expect(&format!("Couldn't open {}", FNAME));
    let reader = BufReader::new(&file);
    let conf = reader
        .lines()
        .filter_map(|l| l.ok())
        .next()
        .map(|l| Configuration::from_line(l.as_str()))
        .expect("No lines?");
    // part 1
    println!("{}", play(conf));
    // part 2
    println!(
        "{}",
        play(Configuration {
            num_players: conf.num_players,
            last_marble: conf.last_marble * 100,
        })
    );
}
