use camino::Utf8PathBuf;
use id_tree::{Node, Tree};
use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    combinator::{all_consuming, map},
    sequence::{preceded, separated_pair},
    Finish, IResult,
};

pub fn day7a() -> String {
    let lines = read_data();

    let tree = construct_tree(&lines);
    // let mut s = String::new();
    // tree.write_formatted(&mut s).unwrap();
    // println!("{s}");

    let sum = tree
        .traverse_pre_order(tree.root_node_id().unwrap())
        .unwrap()
        .filter(|node| !node.children().is_empty())
        .map(|node| total_size(&tree, node))
        // .inspect(|size| {
        //     dbg!(size);
        // })
        .filter(|&size| size <= 100_000)
        .sum::<u64>();

    sum.to_string()
}

pub fn day7b() -> String {
    let lines = read_data();

    let tree = construct_tree(&lines);
    // let mut s = String::new();
    // tree.write_formatted(&mut s).unwrap();
    // println!("{s}");

    let mut sorted_sizes = tree
        .traverse_pre_order(tree.root_node_id().unwrap())
        .unwrap()
        .filter(|node| !node.children().is_empty())
        .map(|node| total_size(&tree, node))
        // .inspect(|size| {
        //     dbg!(size);
        // })
        .sorted_unstable()
        .rev();
    let root_size = sorted_sizes.next().unwrap();
    let required_free = 30_000_000 + root_size - 70_000_000;
    let directory_size = sorted_sizes
        .rev()
        .find(|&size| size >= required_free)
        .unwrap();

    directory_size.to_string()
}

#[derive(Debug)]
struct Ls;

impl Ls {
    fn parse(input: &str) -> IResult<&str, Self> {
        map(tag("ls"), |_| Self)(input)
    }
}

#[derive(Debug)]
struct Cd(Utf8PathBuf);

impl Cd {
    fn parse(input: &str) -> IResult<&str, Cd> {
        map(preceded(tag("cd "), parse_path), Self)(input)
    }
}

fn parse_path(input: &str) -> IResult<&str, Utf8PathBuf> {
    map(
        take_while1(|c: char| "abcdefghijklmnopqrstuvwxyz./".contains(c)),
        Into::into,
    )(input)
}

#[derive(Debug)]
enum Command {
    Ls,
    Cd(Utf8PathBuf),
}

impl Command {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (input, _) = tag("$ ")(input)?;
        alt((map(Ls::parse, Into::into), map(Cd::parse, Into::into)))(input)
    }
}

impl From<Ls> for Command {
    fn from(_: Ls) -> Self {
        Self::Ls
    }
}

impl From<Cd> for Command {
    fn from(cd: Cd) -> Self {
        Self::Cd(cd.0)
    }
}

#[derive(Debug)]
enum Entry {
    Dir(Utf8PathBuf),
    File(u64, Utf8PathBuf),
}

impl Entry {
    fn parse(input: &str) -> IResult<&str, Self> {
        let parse_file = map(
            separated_pair(nom::character::complete::u64, tag(" "), parse_path),
            |(size, path)| Self::File(size, path),
        );
        let parse_dir = map(preceded(tag("dir "), parse_path), Self::Dir);

        alt((parse_file, parse_dir))(input)
    }
}

#[derive(Debug)]
enum Line {
    Command(Command),
    Entry(Entry),
}

impl Line {
    fn parse(input: &str) -> IResult<&str, Self> {
        alt((
            map(Command::parse, Self::Command),
            map(Entry::parse, Self::Entry),
        ))(input)
    }
}

fn read_data() -> Vec<Line> {
    std::fs::read_to_string("inputs/day07.txt")
        .expect("could not read file")
        .lines()
        .map(|line| all_consuming(Line::parse)(line).finish().unwrap().1)
        .collect()
}

#[derive(Debug)]
struct FsEntry {
    _path: Utf8PathBuf,
    size: u64,
}

fn construct_tree(lines: &Vec<Line>) -> Tree<FsEntry> {
    let mut tree = Tree::<FsEntry>::new();
    let root = tree
        .insert(
            Node::new(FsEntry {
                _path: "/".into(),
                size: 0,
            }),
            id_tree::InsertBehavior::AsRoot,
        )
        .unwrap();

    let mut curr = root;

    for line in lines {
        match line {
            Line::Command(cmd) => match cmd {
                Command::Ls => {
                    // ignore
                }
                Command::Cd(path) => match path.as_str() {
                    "/" => {
                        // ignore since we're root
                    }
                    ".." => {
                        curr = tree.get(&curr).unwrap().parent().unwrap().clone();
                    }
                    _ => {
                        let node = Node::new(FsEntry {
                            _path: path.clone(),
                            size: 0,
                        });
                        curr = tree
                            .insert(node, id_tree::InsertBehavior::UnderNode(&curr))
                            .unwrap();
                    }
                },
            },
            Line::Entry(entry) => match entry {
                Entry::Dir(_) => {
                    // ignore for now
                }
                Entry::File(size, path) => {
                    let node = Node::new(FsEntry {
                        size: *size,
                        _path: path.clone(),
                    });
                    tree.insert(node, id_tree::InsertBehavior::UnderNode(&curr))
                        .unwrap();
                }
            },
        }
    }
    tree
}

fn total_size(tree: &Tree<FsEntry>, node: &Node<FsEntry>) -> u64 {
    let mut total = node.data().size;
    for child in node.children() {
        total += total_size(tree, tree.get(child).unwrap());
    }
    total
}
