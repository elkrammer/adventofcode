use std::fs;

pub fn day6() -> String {
    format!("Part 1: {}\nPart 2: {}", part1(), part2())
}

fn get_total_fish(input: &Vec<usize>, days: u32) -> usize {
    let mut totals = [0usize; 9];
    input.iter().for_each(|i| totals[*i] += 1);

    for _ in 1..=days {
        let babys = totals[0];
        for _ in 1..totals.len() {
            totals.rotate_right(1);
        }

        totals[6] += babys;
        totals[8] = babys;
        // println!("Day {}\t: {:?}", day, totals);
    }

    totals.iter().sum()
}

fn part1() -> usize {
    // let input = "3,4,3,1,2";
    let input = fs::read_to_string("input/day6.txt").unwrap();
    let fish: Vec<usize> = input
        .trim()
        .split(',')
        .filter(|f| !f.is_empty())
        .map(|f| f.parse().unwrap())
        .collect();

    get_total_fish(&fish, 80)
}

fn part2() -> usize {
    let input = fs::read_to_string("input/day6.txt").unwrap();
    let fish: Vec<usize> = input
        .trim()
        .split(',')
        .filter(|f| !f.is_empty())
        .map(|f| f.parse().unwrap())
        .collect();

    get_total_fish(&fish, 256)
}
