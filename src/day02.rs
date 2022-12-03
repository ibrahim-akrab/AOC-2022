use itertools::Itertools;

pub fn day2a() -> String {
    let hands = read_data();
    hands
        .iter()
        .map(|line| {
            line.split_whitespace()
                .map(|s| Hand::new(s, None))
                .next_tuple()
                .unwrap()
        })
        .map(|(elf_hand, my_hand)| my_hand.against(&elf_hand) + my_hand.value() + 1)
        .sum::<usize>()
        .to_string()
}

pub fn day2b() -> String {
    let hands = read_data();
    hands
        .iter()
        .map(|line| {
            let hands: Vec<_> = line.split_whitespace().collect();
            let first_hand = Hand::new(hands[0], None);
            let second_hand = Hand::new(hands[1], Some(&first_hand));
            (first_hand, second_hand)
        })
        .map(|(elf_hand, my_hand)| my_hand.against(&elf_hand) + my_hand.value() + 1)
        .sum::<usize>()
        .to_string()
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
    fn new(symbol: &str, other_hand: Option<&Hand>) -> Self {
        match other_hand {
            None => match symbol {
                "A" | "X" => Self::Rock,
                "B" | "Y" => Self::Paper,
                "C" | "Z" => Self::Scissors,
                _ => unreachable!(),
            },
            Some(hand) => match symbol {
                "X" => Hand::with_value((hand.value() + 2) % 3),
                "Y" => Hand::with_value(hand.value()),
                "Z" => Hand::with_value((hand.value() + 1) % 3),
                _ => unreachable!(),
            },
        }
    }

    fn value(&self) -> usize {
        match self {
            Self::Rock => 0,
            Self::Paper => 1,
            Self::Scissors => 2,
        }
    }

    fn with_value(val: usize) -> Self {
        match val {
            0 => Self::Rock,
            1 => Self::Paper,
            2 => Self::Scissors,
            _ => unreachable!(),
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

fn read_data() -> Vec<String> {
    std::fs::read_to_string("inputs/day02.txt")
        .expect("could not read file")
        .lines()
        .map(String::from)
        .collect()
}
