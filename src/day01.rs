use std::collections::BinaryHeap;

pub fn day1a() -> String {
    let elves = read_data();
    sum_of_top_n(&elves, 1).to_string()
}

pub fn day1b() -> String {
    let elves = read_data();
    sum_of_top_n(&elves, 3).to_string()
}

fn sum_of_top_n(elves: &[Elf], n: usize) -> usize {
    let mut elf_calories = elves
        .iter()
        .map(|elf| elf.iter().sum::<usize>())
        .collect::<BinaryHeap<usize>>();
    let mut total = 0usize;
    for _ in 0..n {
        if let Some(calories) = elf_calories.pop() {
            total += calories;
        }
    }
    total
}

type Elf = Vec<usize>;

fn read_data() -> Vec<Elf> {
    std::fs::read_to_string("inputs/day01.txt")
        .expect("could not read file")
        .split("\n\n")
        .map(|s| {
            s.split('\n')
                .filter_map(|v| v.parse().ok())
                .collect::<Elf>()
        })
        .collect()
}
