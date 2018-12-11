#[macro_use]
extern crate lazy_static;
extern crate regex;
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::FromIterator;

#[derive(Debug, Clone, Copy)]
struct Pixel {
    x: i32,
    y: i32,
    dx: i32,
    dy: i32,
}

impl Pixel {
    fn from_string(line: String) -> Pixel {
        lazy_static! {
            // position=< 5, -2> velocity=< 1,  2>
            static ref RE: Regex =
                Regex::new(r"position=<\s*(?P<x>-?\d+),\s*(?P<y>-?\d+)> velocity=<\s*(?P<u>-?\d+),\s*(?P<v>-?\d+)>").unwrap();
        }
        let caps = RE.captures(line.as_str()).unwrap();
        let x: i32 = caps["x"].parse().expect("no current step");
        let y: i32 = caps["y"].parse().expect("no previous step");
        let dx: i32 = caps["u"].parse().expect("no previous step");
        let dy: i32 = caps["v"].parse().expect("no previous step");
        Pixel {x, y, dx, dy}
    }
    fn step(&mut self) {
        self.x += self.dx;
        self.y += self.dy;
    }
    fn step_back(&mut self) {
        self.x -= self.dx;
        self.y -= self.dy;
    }
}

fn area(pixels: &[Pixel]) -> u64 {
    let minx: i32 = pixels.iter().map(|p| p.x).min().expect("no min x?");
    let maxx: i32 = pixels.iter().map(|p| p.x).max().expect("no max x?");
    let miny: i32 = pixels.iter().map(|p| p.y).min().expect("no min y?");
    let maxy: i32 = pixels.iter().map(|p| p.y).max().expect("no max y?");
    ((maxx - minx) as u64) * ((maxy - miny) as u64)
}

fn print_solution(pixels: &[Pixel]) {
    let minx: i32 = pixels.iter().map(|p| p.x).min().expect("no min x?");
    let maxx: i32 = pixels.iter().map(|p| p.x).max().expect("no max x?");
    let miny: i32 = pixels.iter().map(|p| p.y).min().expect("no min y?");
    let maxy: i32 = pixels.iter().map(|p| p.y).max().expect("no max y?");
    let mut buffer: String = String::new();
    for y in miny..=maxy {
        for x in minx..=maxx {
            if pixels.iter().any(|p| p.x == x && p.y == y) {
                buffer.push('#');
            } else {
                buffer.push('.');
            }
        }
        buffer.push('\n');
    }
    println!("{}", buffer);
}

fn part1(pixels: &[Pixel]) {
    let mut pixels = pixels.to_owned();
    let mut prev_area: u64 = std::u64::MAX;
    for i in 0.. {
        let new_area = area(&pixels);
        if new_area > prev_area {
            pixels.iter_mut().for_each(|s| s.step_back());
            // -1 since we have to step back.
            println!("{}", i-1);
            break;
        } else {
            prev_area = new_area;
            pixels.iter_mut().for_each(|s| s.step());
        }
    }
    print_solution(pixels.as_slice());
}

fn main() {
    const FNAME: &str = "input.txt";
    let file = File::open(FNAME).expect(&format!("Couldn't open {}", FNAME));
    let reader = BufReader::new(&file);
    let pixels: Vec<Pixel> = Vec::from_iter(reader.lines().filter_map(|l| l.ok()).map(Pixel::from_string));
    part1(pixels.as_slice());
}
