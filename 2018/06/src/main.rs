use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::FromIterator;

#[derive(Debug, Clone)]
struct Coordinate {
    id: u32,
    x: u32,
    y: u32,
}

impl Coordinate {
    fn distance(&self, i: u32, j: u32) -> u32 {
        (((self.x as i32) - (i as i32)).abs() + ((self.y as i32) - (j as i32)).abs()) as u32
    }
}

fn compute_size(input: &[Coordinate]) -> (u32, u32) {
    let x = input.iter().map(|x| x.x).max().expect("No inputs?");
    let y = input.iter().map(|x| x.y).max().expect("No inputs?");
    (x, y)
}

fn initialize_board(width: u32, height: u32) -> Vec<Vec<u32>> {
    let mut vecs = Vec::with_capacity(height as usize);
    for _ in 0..height {
        vecs.push(vec![0; width as usize]);
    }
    vecs
}

fn nearest(i: usize, j: usize, coords: &[Coordinate]) -> u32 {
    let dists: Vec<(u32, Coordinate)> = coords
        .iter()
        .map(|c| (c.distance(i as u32, j as u32), c.clone()))
        .collect();
    let mindist = dists
        .iter()
        .min_by_key(|(d, _)| d)
        .map(|(d, _)| d)
        .expect("no min");
    if dists.iter().filter(|(d, _)| d == mindist).count() > 1 {
        0u32
    } else {
        dists
            .iter()
            .min_by_key(|(d, _)| d)
            .map(|(_, v)| v.id)
            .expect("no min2")
    }
}

fn total(i: usize, j: usize, coords: &[Coordinate]) -> u32 {
    let dists: Vec<(u32, Coordinate)> = coords
        .iter()
        .map(|c| (c.distance(i as u32, j as u32), c.clone()))
        .collect();
    dists.iter().map(|(d, _)| d).sum::<u32>()
}

fn ids_on_edge(board: &[Vec<u32>]) -> HashSet<u32> {
    // if an edge is closest, it'll be infinite since the nearest will be the same going in that
    // direction.
    let first_row: HashSet<u32> = HashSet::from_iter(board.first().expect("k").iter().cloned());
    let last_row: HashSet<u32> = HashSet::from_iter(board.last().expect("k").iter().cloned());
    let first_col: HashSet<u32> =
        HashSet::from_iter(board.iter().map(|r| r.first().expect("K")).cloned());
    let last_col: HashSet<u32> =
        HashSet::from_iter(board.iter().map(|r| r.last().expect("K")).cloned());
    first_row
        .union(&last_row)
        .cloned()
        .collect::<HashSet<_>>()
        .union(&first_col)
        .cloned()
        .collect::<HashSet<_>>()
        .union(&last_col)
        .cloned()
        .collect()
}

fn counts(board: &[Vec<u32>], infinite: &HashSet<u32>) -> HashMap<u32, u32> {
    let mut count: HashMap<u32, u32> = HashMap::new();
    let vals = board
        .iter()
        .flat_map(|r| r.iter())
        .filter(|v| !infinite.contains(v));
    for v in vals {
        *count.entry(*v).or_insert(0) += 1;
    }
    count
}

fn main() {
    const FNAME: &str = "input.txt";
    let file = File::open(FNAME).expect("Couldn't open input.txt");
    let reader = BufReader::new(&file);
    let coords: Vec<Coordinate> = Vec::from_iter(
        reader
            .lines()
            .filter_map(|l| l.ok())
            .enumerate()
            .map(|(idx, l)| {
                // let id=0 indicate no closest coordinate
                let id = (idx + 1) as u32;
                let mut itr = l.split(", ");
                let x: u32 = itr
                    .next()
                    .expect("No x value")
                    .parse()
                    .expect("couldn't parse x");
                let y: u32 = itr
                    .next()
                    .expect("No y value")
                    .parse()
                    .expect("couldn't parse y");
                Coordinate { id, x, y }
            }),
    );
    let (width, height) = compute_size(coords.as_slice());
    let mut board: Vec<Vec<u32>> = initialize_board(width, height);
    for (i, row) in board.iter_mut().enumerate() {
        for (j, mut cell) in row.iter_mut().enumerate() {
            *cell = nearest(i, j, coords.as_slice());
        }
    }
    let infinite: HashSet<u32> = ids_on_edge(board.as_slice());
    let cnts = counts(board.as_slice(), &infinite);
    let m = cnts.iter().max_by_key(|&(_, v)| v).expect("k");
    println!("{:?}", m.1);
    let mut board2: Vec<Vec<u32>> = initialize_board(width, height);
    for (i, row) in board2.iter_mut().enumerate() {
        for (j, mut cell) in row.iter_mut().enumerate() {
            let t = total(i, j, coords.as_slice());
            if t < 10000 {
                *cell = 1;
            } else {
                *cell = 0;
            }
        }
    }
    let s = board2.iter().flat_map(|r| r.iter()).sum::<u32>();
    println!("{}", s);
}
