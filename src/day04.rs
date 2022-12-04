use std::{num::ParseIntError, str::FromStr};

pub fn day4a() -> String {
    let pairs = read_data();
    pairs
        .iter()
        .filter(|&pair| pair.fully_contains())
        .count()
        .to_string()
}

pub fn day4b() -> String {
    let pairs = read_data();
    pairs
        .iter()
        .filter(|&pair| pair.overlaps())
        .count()
        .to_string()
}

struct Section {
    start: usize,
    end: usize,
}

impl FromStr for Section {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut it = s.split('-');
        if let (Some(start), Some(end), None) = (it.next(), it.next(), it.next()) {
            return Ok(Self {
                start: start.parse()?,
                end: end.parse()?,
            });
        }
        unreachable!()
    }
}

struct Pair {
    first: Section,
    second: Section,
}

impl FromStr for Pair {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut it = s.split(',');
        if let (Some(first), Some(second), None) = (it.next(), it.next(), it.next()) {
            return Ok(Self {
                first: first.parse()?,
                second: second.parse()?,
            });
        }
        unreachable!()
    }
}

impl Pair {
    fn fully_contains(&self) -> bool {
        (self.first.start >= self.second.start && self.first.end <= self.second.end)
            || (self.second.start >= self.first.start && self.second.end <= self.first.end)
    }

    fn overlaps(&self) -> bool {
        self.first.start <= self.second.end && self.second.start <= self.first.end
    }
}

fn read_data() -> Vec<Pair> {
    std::fs::read_to_string("inputs/day04.txt")
        .expect("could not read file")
        .lines()
        .filter_map(|line| line.parse().ok())
        .collect()
}
