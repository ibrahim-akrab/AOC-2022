use std::collections::HashSet;

pub fn day9a() -> String {
    let moves = read_data();
    let mut grid = Grid::default();
    moves.iter().for_each(|m| grid.apply(m));
    // println!("{moves:#?}");
    grid.count_visited().to_string()
}

pub fn day9b() -> String {
    "".to_string()
}

fn read_data() -> Vec<Move> {
    // std::fs::read_to_string("test.txt")
    std::fs::read_to_string("inputs/day09.txt")
        .expect("could not read file")
        .lines()
        .map(Into::into)
        .collect()
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
struct Move {
    direction: Direction,
    steps: usize,
}

impl From<&str> for Move {
    fn from(s: &str) -> Self {
        let first = s.chars().next().expect("Empty input line");
        let (_, second) = s.split_at(2);
        let steps: usize = second
            .parse::<usize>()
            .expect("steps have to be unsigned integer");

        match first {
            'U' => Self {
                direction: Direction::Up,
                steps,
            },
            'D' => Self {
                direction: Direction::Down,
                steps,
            },
            'L' => Self {
                direction: Direction::Left,
                steps,
            },
            'R' => Self {
                direction: Direction::Right,
                steps,
            },
            s => {
                panic!("unknown direction {}", s);
            }
        }
    }
}

#[derive(Default)]
struct Pointer {
    x: isize, // postive x is the Right direction
    y: isize, // postive y is the Up direction
}

impl Pointer {
    fn apply(&mut self, r#move: &Move) {
        use Direction::*;
        match r#move {
            Move {
                direction: Up,
                steps,
            } => self.y += *steps as isize,
            Move {
                direction: Down,
                steps,
            } => self.y -= *steps as isize,
            Move {
                direction: Right,
                steps,
            } => self.x += *steps as isize,
            Move {
                direction: Left,
                steps,
            } => self.x -= *steps as isize,
        }
    }

    fn follow(&mut self, other: &Self, dir: &Direction) -> Vec<(isize, isize)> {
        // let mut diagonal_push = false;
        let mut visited: Vec<(isize, isize)> = vec![];
        while !self.touching(other) {
            use Direction::*;
            // if !diagonal_push {
            //     match dir {
            //         Up | Down => {
            //             diagonal_push = true;
            //             self.x = other.x;
            //         }
            //         Left | Right => {
            //             diagonal_push = true;
            //             self.y = other.y;
            //         }
            //     }
            // }
            match dir {
                Up => {
                    self.x = other.x;
                    self.y += 1;
                }
                Down => {
                    self.x = other.x;
                    self.y -= 1;
                }
                Right => {
                    self.y = other.y;
                    self.x += 1;
                }
                Left => {
                    self.y = other.y;
                    self.x -= 1;
                }
            }
            // match dir {
            //     Up => self.y += 1,
            //     Down => self.y -= 1,
            //     Right => self.x += 1,
            //     Left => self.x -= 1,
            // }
            visited.push((self.x, self.y));
            // println!("tail pos: {},{}", self.x, self.y);
        }
        visited
    }

    // check if touching even diagonally
    fn touching(&self, other: &Self) -> bool {
        self.x.abs_diff(other.x) <= 1 && self.y.abs_diff(other.y) <= 1
    }
}

struct Grid {
    head: Pointer,
    tail: Pointer,
    visited: HashSet<(isize, isize)>,
}

impl Default for Grid {
    fn default() -> Self {
        // inserting the initial starting position into the hashset
        Self {
            head: Default::default(),
            tail: Default::default(),
            visited: vec![(0, 0)].into_iter().collect(),
        }
    }
}

impl Grid {
    fn apply(&mut self, r#move: &Move) {
        self.head.apply(r#move);

        // println!("{:#?}", r#move);
        // println!("head pos: {}, {}", self.head.x, self.head.y);
        let visited = self.tail.follow(&self.head, &r#move.direction);
        visited.iter().for_each(|pos| {
            self.visited.insert(*pos);
        });
    }

    fn count_visited(&self) -> usize {
        // the 1 is for the initial starting position
        self.visited.len()
    }
}
