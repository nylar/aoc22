use std::{
    ops::{Index, IndexMut},
    str::FromStr,
};

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref MOVE_RE: Regex = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
}

fn main() {
    let input = include_str!("../input");

    let diagram: Vec<_> = input.lines().take_while(|l| !l.is_empty()).collect();

    let moves: Vec<_> = input
        .lines()
        .skip(diagram.len() + 1)
        .filter_map(|l| l.parse::<Move>().ok())
        .collect();

    let mut stacks = Stacks::new(&diagram);
    let mut stacks2 = stacks.clone();

    for mov in &moves {
        stacks.move9000(mov);
    }

    println!("Part 1: {:?}", stacks.top());

    for mov in &moves {
        stacks2.move9001(mov);
    }

    println!("Part 2: {:?}", stacks2.top());
}

#[derive(Clone, Debug)]
struct Stacks {
    stacks: Vec<Vec<char>>,
}

impl Index<usize> for Stacks {
    type Output = Vec<char>;

    fn index(&self, index: usize) -> &Self::Output {
        if index > self.stacks.len() {
            panic!(
                "index out of bounds {} for len {}",
                index,
                self.stacks.len()
            );
        }

        self.stacks.get(index - 1).unwrap()
    }
}

impl IndexMut<usize> for Stacks {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if index > self.stacks.len() {
            panic!(
                "index out of bounds {} for len {}",
                index,
                self.stacks.len()
            );
        }

        self.stacks.get_mut(index - 1).unwrap()
    }
}

impl Stacks {
    fn new(diagram: &[&str]) -> Self {
        let columns = (diagram.iter().map(|l| l.len()).max().unwrap() + 1) / 4;

        let mut stacks: Vec<Vec<char>> = Vec::with_capacity(columns);
        for _ in 0..columns {
            stacks.push(vec![]);
        }

        for line in diagram.iter().rev().skip(1) {
            for (stack, chunk) in line.as_bytes().chunks(4).enumerate() {
                if chunk[0] as char == '[' {
                    stacks[stack].push(chunk[1] as char);
                }
            }
        }

        Self { stacks }
    }

    fn move9000(&mut self, mov: &Move) {
        for _ in 0..mov.count {
            let v = self[mov.source].pop().unwrap();
            self[mov.destination].push(v);
        }
    }

    fn move9001(&mut self, mov: &Move) {
        if mov.count == 1 {
            return self.move9000(mov);
        }

        let mut s = Vec::with_capacity(mov.count as usize);
        for _ in 0..mov.count {
            let v = self[mov.source].pop().unwrap();
            s.push(v);
        }
        s.reverse();

        self[mov.destination].extend_from_slice(&s);
    }

    fn top(&self) -> String {
        self.stacks.iter().filter_map(|s| s.last()).collect()
    }
}

#[derive(Debug)]
struct Move {
    count: u8,
    source: usize,
    destination: usize,
}

impl FromStr for Move {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let captures = MOVE_RE.captures(s).unwrap();

        let count = captures.get(1).unwrap().as_str().parse::<u8>().unwrap();
        let source = captures.get(2).unwrap().as_str().parse::<usize>().unwrap();
        let destination = captures.get(3).unwrap().as_str().parse::<usize>().unwrap();

        Ok(Self {
            count,
            source,
            destination,
        })
    }
}
