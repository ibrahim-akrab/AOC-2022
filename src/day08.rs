pub fn day8a() -> String {
    let height_map = read_data();
    count_visible_trees(&height_map).to_string()
}

pub fn day8b() -> String {
    "".to_string()
}

fn read_data() -> Vec<Vec<u8>> {
    std::fs::read_to_string("inputs/day08.txt")
        .expect("could not read file")
        .lines()
        .map(|line| line.bytes().map(|b| b - b'0').collect::<Vec<u8>>())
        .collect()
}

enum Direction {
    Left,
    Right,
    Up,
    Down,
}

fn count_visible_trees(map: &[Vec<u8>]) -> usize {
    const W: usize = 99;
    let mut lowest_neighbor = [[i8::MAX; W]; W];
    use Direction::*;
    for direction in vec![Left, Right, Up, Down] {
        match direction {
            Left => {
                for row_idx in 0..W {
                    let mut tallest_tree = i8::MIN;
                    for col_idx in 0..W {
                        let h = map[row_idx][col_idx];
                        lowest_neighbor[row_idx][col_idx] =
                            lowest_neighbor[row_idx][col_idx].min(tallest_tree);
                        tallest_tree = tallest_tree.max(h as i8);
                        if h == 9 {
                            break;
                        }
                    }
                }
            }
            Right => {
                for row_idx in 0..W {
                    let mut tallest_tree = i8::MIN;
                    for col_idx in (0..W).rev() {
                        let h = map[row_idx][col_idx];
                        lowest_neighbor[row_idx][col_idx] =
                            lowest_neighbor[row_idx][col_idx].min(tallest_tree);
                        tallest_tree = tallest_tree.max(h as i8);
                        if h == 9 {
                            break;
                        }
                    }
                }
            }
            Up => {
                for col_idx in 0..W {
                    let mut tallest_tree = i8::MIN;
                    for row_idx in 0..W {
                        let h = map[row_idx][col_idx];
                        lowest_neighbor[row_idx][col_idx] =
                            lowest_neighbor[row_idx][col_idx].min(tallest_tree);
                        tallest_tree = tallest_tree.max(h as i8);
                        if h == 9 {
                            break;
                        }
                    }
                }
            }
            Down => {
                for col_idx in 0..W {
                    let mut tallest_tree = i8::MIN;
                    for row_idx in (0..W).rev() {
                        let h = map[row_idx][col_idx];
                        lowest_neighbor[row_idx][col_idx] =
                            lowest_neighbor[row_idx][col_idx].min(tallest_tree);
                        tallest_tree = tallest_tree.max(h as i8);
                        if h == 9 {
                            break;
                        }
                    }
                }
            }
        }
    }

    // compare tree height with the lowest visibility height
    map.iter()
        .flatten()
        .zip(lowest_neighbor.iter().flatten())
        .filter(|(&tree, &l)| tree as i8 > l)
        .count()
}
