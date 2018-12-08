#[macro_use]
extern crate lazy_static;
extern crate regex;
use regex::Regex;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::FromIterator;

#[derive(Debug, Clone)]
struct Direction {
    step: char,
    before: char,
}
impl Direction {
    fn from_str(line: String) -> Direction {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"^Step (?P<a>.) must .* step (?P<b>.) can begin\.$").unwrap();
        }
        let caps = RE.captures(line.as_str()).unwrap();
        let step: char = caps["a"].parse().expect("no current step");
        let before: char = caps["b"].parse().expect("no previous step");
        Direction { step, before }
    }
}

fn determine_ready_steps(directions: &[Direction]) -> Vec<char> {
    let steps: HashSet<char> = directions.iter().fold(HashSet::new(), |mut acc, d| {
        acc.insert(d.step);
        acc.insert(d.before);
        acc
    });
    let before: HashSet<char> = directions.iter().fold(HashSet::new(), |mut acc, d| {
        acc.insert(d.before);
        acc
    });
    Vec::from_iter(steps.difference(&before).cloned())
}

fn instructions(directions: &[Direction], mut acc: String) -> String {
    if directions.is_empty() {
        return acc;
    }
    let ready: Vec<char> = determine_ready_steps(directions);
    //println!("{}", ready.len());
    let next: char = *ready.iter().min().expect("None left");
    acc.push(next);
    //println!("{:?}", ready);
    let new_directions: Vec<Direction> =
        Vec::from_iter(directions.iter().filter(|d| d.step != next).cloned());
    if new_directions.len() == 0 {
        //println!("{:?}", directions);
        for d in directions {
            acc.push(d.before);
        }
    }
    return instructions(new_directions.as_slice(), acc);
}

#[derive(Debug, Clone, Copy)]
struct Worker {
    on: char,
    busy_unil: usize,
}

fn step_cost(step: char) -> usize {
    ((step as u8) - 4) as usize
}

fn parallel(directions: &[Direction], mut workers: [Worker; 5], curtime: usize) -> usize {
    // completed, so need to update list of directions to follow
    let completed_steps: HashSet<char> = HashSet::from_iter(workers.iter().filter(|w| w.busy_unil <= curtime).map(|w| &w.on).cloned());
    // steps workers are working on, so we don't want anyone else to work on these
    let working_steps: HashSet<char> = HashSet::from_iter(workers.iter().map(|w| &w.on).cloned());
    // These workers are available for a new task
    let ready_workers = workers.iter_mut().filter(|w| w.busy_unil <= curtime);
    // update list of directions that aren't complete.
    let new_directions: Vec<Direction> =
        Vec::from_iter(directions.iter().filter(|d| !completed_steps.contains(&d.step)).cloned());
    // If last direction, simply add the time it takes to complete.
    if new_directions.len() == 0 {
        let mut max_time: usize = 0;
        for d in directions {
            let delta: usize = step_cost(d.before);
            if delta > max_time {
                max_time = delta;
            }
        }
        return curtime + max_time;
    }
    let mut ready_steps: Vec<char> = determine_ready_steps(new_directions.as_slice());
    ready_steps.sort_unstable();
    // remove the steps we are already working on
    let filtered_ready_steps: Vec<char> = Vec::from_iter(ready_steps.iter().filter(|s| !working_steps.contains(&s)).cloned());
    let steps = &mut filtered_ready_steps.iter().cloned();
    // Assign the steps to our idle workers
    for w in ready_workers {
        let next_step_opt: Option<char> = steps.next();
        if next_step_opt.is_some() {
            let next_step: char = next_step_opt.unwrap();
            w.on = next_step;
            w.busy_unil = curtime + step_cost(next_step);
        } else {
            // worker is still idle
            w.on = ' ';
        }
    }
    // next event time
    let next_time: usize = workers.iter().filter(|w| w.on != ' ').map(|w| w.busy_unil).min().unwrap();
    return parallel(new_directions.as_slice(), workers, next_time);
}

fn main() {
    const FNAME: &str = "input.txt";
    let file = File::open(FNAME).expect(&format!("Couldn't open {}", FNAME));
    let reader = BufReader::new(&file);
    let directions: Vec<Direction> = reader
        .lines()
        .filter_map(|l| l.ok())
        .map(Direction::from_str)
        .collect();
    println!("{}", instructions(directions.as_slice(), "".to_string()));
    let workers = [Worker{ on: ' ', busy_unil: 0}; 5];
    let time = parallel(directions.as_slice(), workers, 0);
    println!("{}", time);
}
