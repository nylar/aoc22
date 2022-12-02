use std::str::FromStr;

fn main() {
    let rounds = include_str!("../input")
        .lines()
        .filter_map(|l| str::split_once(l, ' '));

    let total = rounds
        .clone()
        .map(|(f, s)| (f.parse::<Move>().unwrap(), s.parse::<Move>().unwrap()))
        .map(|(cpu, human)| human.score1(cpu))
        .sum::<i64>();

    println!("Part 1: {}", total);

    let total = rounds
        .clone()
        .map(|(f, s)| (f.parse::<Move>().unwrap(), s.parse::<Outcome>().unwrap()))
        .map(|(cpu, outcome)| cpu.score2(outcome))
        .sum::<i64>();

    println!("Part 2: {}", total);
}

#[derive(Clone, Copy, Debug)]
enum Move {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl Move {
    fn play1(&self, other: Move) -> Outcome {
        use Move::*;

        match (*self, other) {
            (Paper, Rock) | (Scissors, Paper) | (Rock, Scissors) => Outcome::Win,
            (Rock, Rock) | (Paper, Paper) | (Scissors, Scissors) => Outcome::Draw,
            (Rock, Paper) | (Paper, Scissors) | (Scissors, Rock) => Outcome::Loss,
        }
    }

    fn score1(&self, other: Move) -> i64 {
        *self as i64 + self.play1(other) as i64
    }

    fn play2(&self, outcome: Outcome) -> Move {
        use {Move::*, Outcome::*};

        match (*self, outcome) {
            (Rock, Win) => Paper,
            (Paper, Win) => Scissors,
            (Scissors, Win) => Rock,
            (Rock, Loss) => Scissors,
            (Paper, Loss) => Rock,
            (Scissors, Loss) => Paper,
            (r#move, Draw) => r#move,
        }
    }

    fn score2(&self, outcome: Outcome) -> i64 {
        let r#move = self.play2(outcome);

        r#move as i64 + r#move.play1(*self) as i64
    }
}

impl FromStr for Move {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Move::Rock),
            "B" | "Y" => Ok(Move::Paper),
            "C" | "Z" => Ok(Move::Scissors),
            _ => Err(format!("unhandled move: '{}'", s)),
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Outcome {
    Loss = 0,
    Draw = 3,
    Win = 6,
}

impl FromStr for Outcome {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Outcome::Loss),
            "Y" => Ok(Outcome::Draw),
            "Z" => Ok(Outcome::Win),
            _ => Err(format!("unhandled outcome: '{}'", s)),
        }
    }
}
