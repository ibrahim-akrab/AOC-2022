use itertools::Itertools;

pub fn day2a() -> String {
    let hands = read_data();
    hands
        .iter()
        .map(|(elf_hand, my_hand)| my_hand.against(elf_hand) + my_hand.value())
        .sum::<usize>()
        .to_string()
}

pub fn day2b() -> String {
    "".to_string()
}

#[derive(PartialEq)]
enum Hand {
    Rock,
    Paper,
    Scissors,
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        use std::cmp::Ordering;
        if self.value() == other.value() {
            return Some(Ordering::Equal);
        }
        if self.value() % 3 == (other.value() + 1) % 3 {
            return Some(Ordering::Greater);
        }
        Some(Ordering::Less)
    }
}

impl Hand {
    fn new(symbol: &str) -> Self {
        match symbol {
            "A" | "X" => Self::Rock,
            "B" | "Y" => Self::Paper,
            "C" | "Z" => Self::Scissors,
            _ => unreachable!(),
        }
    }

    fn value(&self) -> usize {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }

    fn against(&self, other_hand: &Self) -> usize {
        use std::cmp::Ordering;
        if let Some(order) = self.partial_cmp(other_hand) {
            match order {
                Ordering::Less => {
                    return 0;
                }
                Ordering::Equal => {
                    return 3;
                }
                Ordering::Greater => {
                    return 6;
                }
            }
        }
        unreachable!()
    }
}

fn read_data() -> Vec<(Hand, Hand)> {
    std::fs::read_to_string("inputs/day02.txt")
        .expect("could not read file")
        .lines()
        .map(|line| line.split_whitespace().map(Hand::new).next_tuple().unwrap())
        .collect()
}
