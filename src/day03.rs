pub fn day3a() -> String {
    let rucksacks = read_data();
    fn get_lookup_index(ch: char) -> usize {
        match ch {
            small if ch as usize >= 'a' as usize && ch as usize <= 'z' as usize => {
                small as usize - 'a' as usize
            }
            capital if ch as usize >= 'A' as usize && ch as usize <= 'Z' as usize => {
                capital as usize - 'A' as usize + 26
            }
            _ => unreachable!(),
        }
    }
    rucksacks
        .iter()
        .map(|rucksack| {
            let mut exists = [false; 52];
            let (left, right) = rucksack.split_at(rucksack.len() / 2);
            left.chars()
                .for_each(|item| exists[get_lookup_index(item)] = true);
            if let Some(shared_type) = right.chars().find(|&ch| exists[get_lookup_index(ch)]) {
                return get_lookup_index(shared_type) + 1;
            }
            0usize
        })
        .sum::<usize>()
        .to_string()
}

pub fn day3b() -> String {
    "".to_string()
}

fn read_data() -> Vec<String> {
    std::fs::read_to_string("inputs/day03.txt")
        .expect("could not read file")
        .lines()
        .map(String::from)
        .collect()
}
