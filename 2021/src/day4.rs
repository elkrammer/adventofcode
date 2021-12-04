use std::convert::TryInto;
use std::fs;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct Board {
    numbers: [[u8; 5]; 5],
    marked: [[bool; 5]; 5],
}

impl Board {
    fn mark(&mut self, number: u8) {
        for (y, row) in self.numbers.iter().enumerate() {
            if let Some(x) = row.iter().position(|&n| n == number) {
                self.marked[y][x] = true;
                return;
            }
        }
    }

    fn is_winner(&self) -> bool {
        // check rows
        let mut rows = self.marked.iter();
        if rows.any(|row| row.iter().all(|mark| *mark)) {
            return true;
        }

        // check columns
        for x in 0..self.marked.len() {
            let mut column = self.marked.iter().map(|row| row[x]);
            if column.all(|mark| mark) {
                return true;
            }
        }
        false
    }

    fn sum_unmarked(&self) -> u32 {
        let mut sum = 0u32;
        for (y, row) in self.numbers.iter().enumerate() {
            for (x, number) in row.iter().enumerate() {
                if !self.marked[y][x] {
                    sum += *number as u32;
                }
            }
        }
        sum
    }
}

impl FromStr for Board {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let numbers = s
            .lines()
            .take(5)
            .map(|line| {
                line.split_whitespace()
                    .filter(|n| !n.is_empty())
                    .map(|n| n.parse::<u8>().unwrap())
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap()
            })
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        Ok(Board {
            numbers,
            marked: Default::default(),
        })
    }
}

pub fn day4() -> String {
    format!("Part 1: {}\nPart 2: {}", part1(), part2())
}

fn part1() -> u32 {
    let f = fs::read_to_string("input/day4.txt").unwrap();
    let mut lines = f.split("\n\n");

    let draw_numbers: Vec<u8> = lines
        .next()
        .unwrap()
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect();
    let mut boards: Vec<Board> = lines.map(|x| x.parse().unwrap()).collect();

    for num in draw_numbers {
        boards.iter_mut().for_each(|board| board.mark(num));
        if let Some(board) = boards.iter().find(|board| board.is_winner()) {
            // println!("Part 1 Winning Board: \n{:?}", board);
            return (num as u32) * board.sum_unmarked();
        }
    }

    panic!("There was no winner")
}

fn part2() -> u32 {
    let f = fs::read_to_string("input/day4.txt").unwrap();
    let mut lines = f.split("\n\n");

    let draw_numbers: Vec<u8> = lines
        .next()
        .unwrap()
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect();
    let mut boards: Vec<Board> = lines.map(|x| x.parse().unwrap()).collect();

    for num in draw_numbers {
        boards.iter_mut().for_each(|board| board.mark(num));

        // remove winners until last
        while let Some(pos) = boards.iter().position(|board| board.is_winner()) {
            let board = boards.remove(pos);
            if boards.is_empty() {
                // println!("Part 2 Last Winning Board: \n{:?}", board);
                return (num as u32) * board.sum_unmarked();
            }
        }
    }
    panic!("There was no last winning board")
}
