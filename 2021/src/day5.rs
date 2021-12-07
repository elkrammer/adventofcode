use std::collections::HashMap;
use std::fs;

pub fn day5() -> String {
    let input = fs::read_to_string("input/day5.txt").unwrap();
    let part1 = part1(&input);
    let part2 = part2(&input);
    format!("Part 1: {}\nPart 2: {}", part1, part2)
}

fn solve(input: &str, check_diagonals: bool, print_grid: bool) -> i32 {
    let mut overlaps: HashMap<(i32, i32), i32> = HashMap::new();

    for vent in input.lines() {
        let mut chunks = vent.split("->").map(|v| v.trim());
        let start: Vec<i32> = chunks
            .next()
            .unwrap()
            .split(',')
            .map(|v| v.parse().unwrap())
            .collect();
        let end: Vec<i32> = chunks
            .next()
            .unwrap()
            .split(',')
            .map(|v| v.parse().unwrap())
            .collect();

        let (x1, y1): (i32, i32) = (start[0], start[1]);
        let (x2, y2): (i32, i32) = (end[0], end[1]);

        if x1 == x2 {
            for y in y1.min(y2)..=y1.max(y2) {
                *overlaps.entry((x1, y)).or_default() += 1;
            }
        } else if y1 == y2 {
            for x in x1.min(x2)..=x1.max(x2) {
                *overlaps.entry((x, y1)).or_default() += 1;
            }
        } else if check_diagonals {
            let x_direction = if x2 - x1 > 0 { 1 } else { -1 };
            let y_direction = if y2 - y1 > 0 { 1 } else { -1 };
            let mut x = x1;
            let mut y = y1;

            *overlaps.entry((x, y)).or_default() += 1;

            while x != x2 {
                x += x_direction;
                y += y_direction;
                *overlaps.entry((x, y)).or_default() += 1;
            }
        }
    }

    // print out the grid
    if print_grid {
        for y in 0..=9 {
            for x in 0..=9 {
                print!("{}", overlaps.get(&(x, y)).unwrap_or(&0));
            }
            println!();
        }
    }

    return overlaps.values().filter(|v| **v >= 2).count() as i32;
}

fn part1(input: &str) -> i32 {
    // let input: String = "0,9 -> 5,9
    //             8,0 -> 0,8
    //             9,4 -> 3,4
    //             2,2 -> 2,1
    //             7,0 -> 7,4
    //             6,4 -> 2,0
    //             0,9 -> 2,9
    //             3,4 -> 1,4
    //             0,0 -> 8,8
    //             5,5 -> 8,2"
    //     .to_string();
    return solve(&input, false, false);
}

fn part2(input: &str) -> i32 {
    return solve(&input, true, false);
}
