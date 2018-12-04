#[macro_use]
extern crate lazy_static;

extern crate regex;
use regex::Regex;

use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct Claim {
    id: u16,
    left: usize,
    top: usize,
    width: usize,
    height: usize,
}
impl Claim {
    fn xrange(&self) -> std::ops::Range<usize> {
        (self.left)..(self.left + self.width)
    }
    fn yrange(&self) -> std::ops::Range<usize> {
        (self.top)..(self.top + self.height)
    }
}
fn parse_line(line: &str) -> Claim {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"#(?P<id>\d+) @ (?P<l>\d+),(?P<t>\d+): (?P<w>\d+)x(?P<h>\d+)").unwrap();
    }
    let caps = RE.captures(line).unwrap();
    let id: u16 = caps["id"].parse().expect("Couldn't parse id");
    let left: usize = caps["l"].parse().expect("Couldn't parse left");
    let top: usize = caps["t"].parse().expect("Couldn't parse top");
    let width: usize = caps["w"].parse().expect("Couldn't parse width");
    let height: usize = caps["h"].parse().expect("Couldn't parse height");
    Claim {
        id: id,
        left: left,
        top: top,
        width: width,
        height: height,
    }
}
fn main() {
    const FNAME: &str = "input.txt";
    let file = File::open(FNAME).expect(&format!("Couldn't open {}", FNAME));
    let reader = BufReader::new(&file);
    let claims: Vec<Claim> = reader
        .lines()
        .filter_map(|l| l.ok())
        .map(|l| parse_line(l.as_str()))
        .collect();
    // static array since we were given a max size
    let mut mat = [[0usize; 1000]; 1000];
    for c in &claims {
        for i in c.xrange() {
            for j in c.yrange() {
                mat[i][j] += 1;
            }
        }
    }
    let mut n: u32 = 0;
    // probably a better way to do this sum.
    for i in 0..1000 {
        for j in 0..1000 {
            if mat[i][j] > 1 {
                n += 1;
            }
        }
    }

    println!("{}", n);

    'outer: for c in &claims {
        // increment again
        for i in c.xrange() {
            for j in c.yrange() {
                mat[i][j] += 1;
            }
        }
        // sum again
        let mut n: usize = 0;
        for i in c.xrange() {
            for j in c.yrange() {
                n += mat[i][j];
            }
        }
        // the only one with no overlap should now be 2 for every cell
        let exp = 2 * c.width * c.height;
        if exp == n {
            println!("{:?}", c);
            break 'outer;
        }
        // reset the matrix
        for i in c.xrange() {
            for j in c.yrange() {
                mat[i][j] -= 1;
            }
        }
    }
}
