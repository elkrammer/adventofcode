use std::str::FromStr;
use std::string::ParseError;
use std::{cmp::Ordering, fs};

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Hand {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Self::Rock, Self::Rock) => Some(Ordering::Equal),
            (Self::Paper, Self::Paper) => Some(Ordering::Equal),
            (Self::Scissors, Self::Scissors) => Some(Ordering::Equal),

            (Self::Rock, Self::Scissors) => Some(Ordering::Greater),
            (Self::Scissors, Self::Rock) => Some(Ordering::Less),

            (Self::Scissors, Self::Paper) => Some(Ordering::Greater),
            (Self::Paper, Self::Scissors) => Some(Ordering::Less),

            (Self::Paper, Self::Rock) => Some(Ordering::Greater),
            (Self::Rock, Self::Paper) => Some(Ordering::Less),
        }
    }
}

impl FromStr for Hand {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Self::Rock),
            "B" | "Y" => Ok(Self::Paper),
            "C" | "Z" => Ok(Self::Scissors),
            _ => panic!("invalid input: {s}"),
        }
    }
}
impl Hand {
    fn score(&self) -> usize {
        match self {
            Self::Rock => Self::Rock as usize,
            Self::Paper => Self::Paper as usize,
            Self::Scissors => Self::Scissors as usize,
        }
    }
}

pub fn day2() -> String {
    let input = fs::read_to_string("input/day2.txt").unwrap();
    let part1 = part1(&input);
    let part2 = part2(&input);
    format!("Part 1: {}\nPart 2: {}", part1, part2)
}

fn score(p1: Hand, p2: Hand) -> (usize, usize) {
    let (x, y) = if p1 == p2 {
        (3, 3)
    } else if p1 < p2 {
        (0, 6)
    } else {
        (6, 0)
    };

    (x + p1.score(), y + p2.score())
}

pub fn part1(input: &str) -> usize {
    let mut final_score = 0;

    for play in input.lines() {
        let play: Vec<_> = play.split_ascii_whitespace().collect();
        let p1 = Hand::from_str(&play[0]).unwrap();
        let p2 = Hand::from_str(&play[1]).unwrap();

        let score = score(p1, p2);
        final_score += score.1;
        // println!("{:?}, {:?} ==> {:?}", p1, p2, score);
    }

    final_score
}

#[derive(Debug)]
enum Outcome {
    Win,
    Lose,
    Draw,
}

impl FromStr for Outcome {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "X" => Outcome::Lose,
            "Y" => Outcome::Draw,
            "Z" => Outcome::Win,
            _ => panic!("invalid input: {s}"),
        })
    }
}

pub fn part2(input: &str) -> usize {
    let mut final_score = 0;

    for play in input.lines() {
        let play: Vec<_> = play.split_ascii_whitespace().collect();

        let p1 = Hand::from_str(&play[0]).unwrap();
        let outcome = Outcome::from_str(&play[1]).unwrap();

        let p2 = match (p1, outcome) {
            (Hand::Rock, Outcome::Win) => Hand::Paper,
            (Hand::Rock, Outcome::Lose) => Hand::Scissors,
            (Hand::Rock, Outcome::Draw) => Hand::Rock,
            (Hand::Paper, Outcome::Win) => Hand::Scissors,
            (Hand::Paper, Outcome::Lose) => Hand::Rock,
            (Hand::Paper, Outcome::Draw) => Hand::Paper,
            (Hand::Scissors, Outcome::Win) => Hand::Rock,
            (Hand::Scissors, Outcome::Lose) => Hand::Paper,
            (Hand::Scissors, Outcome::Draw) => Hand::Scissors,
        };

        // println!("Opp: {:?}, Me: {:?}", p1, p2);
        let score = score(p1, p2);
        final_score += score.1;
    }

    final_score
}
