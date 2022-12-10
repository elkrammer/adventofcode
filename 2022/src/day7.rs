use std::{fs, str::Lines};

#[derive(Debug)]
pub struct Dir {
    file_sizes: Vec<i32>,
    dirs: Vec<Dir>,
}

impl Dir {
    fn new() -> Self {
        Self {
            file_sizes: Vec::new(),
            dirs: Vec::new(),
        }
    }

    fn get_file_size(&self) -> i32 {
        self.file_sizes.iter().sum()
    }

    fn get_dir_sizes(&self, limit: i32, result: &mut Vec<i32>) -> i32 {
        let size = self.get_file_size()
            + self
                .dirs
                .iter()
                .map(|dir| dir.get_dir_sizes(limit, result))
                .sum::<i32>();
        if size < limit {
            result.push(size);
        }
        size
    }
}

pub fn parse(fs: String) -> Dir {
    let mut lines = fs.lines();
    lines.next();

    fn parse_dir(lines: &mut Lines) -> Dir {
        let mut dir = Dir::new();
        while let Some(line) = lines.next() {
            let line: Vec<&str> = line.split(' ').collect();

            if line[0] == "$" {
                if line[1] == "cd" {
                    if line[2] == ".." {
                        return dir;
                    }
                } else {
                    let child = parse_dir(lines);
                    dir.dirs.push(child);
                }
            } else if line[0] != "dir" {
                dir.file_sizes.push(line[0].parse().unwrap());
            }
        }
        dir
    }

    return parse_dir(&mut lines);
}

pub fn day7() -> String {
    let input = fs::read_to_string("input/day7.txt").unwrap();
    let part1 = part1(&input);
    let part2 = part2(&input);
    format!("Part 1: {}\nPart 2: {}", part1, part2)
}

pub fn part1(input: &str) -> i32 {
    let fs = parse(input.to_string());
    let mut sizes: Vec<i32> = Vec::new();
    fs.get_dir_sizes(100_000, &mut sizes);
    sizes.iter().sum::<i32>()
}

pub fn part2(input: &str) -> i32 {
    let fs = parse(input.to_string());
    let root_size: i32 = 70_000_000;
    let update_size: i32 = 30_000_000;
    let used_space = fs.get_dir_sizes(100_000, &mut Vec::new());
    let to_free = used_space - (root_size - update_size);
    let mut sizes: Vec<i32> = Vec::new();
    fs.get_dir_sizes(root_size, &mut sizes);
    *sizes.iter().filter(|&s| s >= &to_free).min().unwrap()
}
