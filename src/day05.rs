pub fn day5a() -> String {
    let mut crane = read_data();
    crane.do_work_9000();
    crane.get_stack_face()
}

pub fn day5b() -> String {
    let mut crane = read_data();
    crane.do_work_9001();
    crane.get_stack_face()
}

struct Rearrangement {
    n: usize,
    from: usize,
    to: usize,
}

struct Crane {
    stacks: Vec<Vec<char>>,
    rearrangements: Vec<Rearrangement>,
}

impl Crane {
    fn new(n: usize) -> Self {
        let mut stacks = Vec::new();
        for _ in 0..n {
            stacks.push(Vec::new());
        }
        Self {
            stacks,
            rearrangements: Vec::new(),
        }
    }

    fn add_crate(&mut self, stack: usize, crate_type: char) {
        self.stacks[stack].push(crate_type);
    }

    fn remove_crate(&mut self, stack: usize) -> Option<char> {
        self.stacks[stack].pop()
    }

    fn move_crate_9000(&mut self, n: usize, from: usize, to: usize) {
        for _ in 0..n {
            if let Some(ch) = self.remove_crate(from) {
                self.add_crate(to, ch);
            } else {
                panic!("Can't move crates from empty stack");
            }
        }
    }

    fn move_crate_9001(&mut self, n: usize, from: usize, to: usize) {
        let mut temp = Vec::new();
        for _ in 0..n {
            if let Some(ch) = self.remove_crate(from) {
                temp.push(ch);
            } else {
                panic!("Can't move crates from empty stack");
            }
        }
        while let Some(ch) = temp.pop() {
            self.add_crate(to, ch);
        }
    }

    fn add_rearrangement(&mut self, rearrangement: Rearrangement) {
        self.rearrangements.push(rearrangement);
    }

    fn do_work_9000(&mut self) {
        while let Some(rearrangement) = self.rearrangements.pop() {
            self.move_crate_9000(rearrangement.n, rearrangement.from, rearrangement.to);
        }
    }

    fn do_work_9001(&mut self) {
        while let Some(rearrangement) = self.rearrangements.pop() {
            self.move_crate_9001(rearrangement.n, rearrangement.from, rearrangement.to);
        }
    }

    fn get_stack_face(&self) -> String {
        self.stacks.iter().fold("".to_string(), |acc, stack| {
            if let Some(top) = stack.last() {
                return format!("{acc}{top}");
            }
            acc
        })
    }
}

fn read_data() -> Crane {
    let content = std::fs::read_to_string("inputs/day05.txt").expect("could not read file");
    let mut iter = content.split("\n\n");
    if let (Some(crates), Some(rearrangements), None) = (iter.next(), iter.next(), iter.next()) {
        let number_of_stacks = crates.lines().last().unwrap().split_whitespace().count();
        let mut crane = Crane::new(number_of_stacks);
        crates.lines().rev().skip(1).for_each(|line| {
            for stack in 0..number_of_stacks {
                if let Some(ch) = line.chars().nth(1 + stack * 4) {
                    if ch != ' ' {
                        crane.add_crate(stack, ch);
                    }
                }
            }
        });
        rearrangements.lines().rev().for_each(|line| {
            let commands = line
                .split_whitespace()
                .skip(1)
                .step_by(2)
                .filter_map(|s| s.parse().ok())
                .collect::<Vec<_>>();
            let rearrangement = Rearrangement {
                n: commands[0],
                from: commands[1] - 1,
                to: commands[2] - 1,
            };
            crane.add_rearrangement(rearrangement);
        });
        return crane;
    }
    panic!("input file structure not valid");
}
