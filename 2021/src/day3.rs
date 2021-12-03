use std::cmp::Ordering;
use std::fs;

pub fn day3() -> String {
    format!("Part 1: {}\nPart 2: {}", part1(), part2())
}

fn part1() -> u64 {
    let f = fs::read_to_string("input/day3.txt").unwrap();
    let len = f.split_whitespace().next().unwrap().len();
    let mut zeros = vec![0; len];
    let mut ones = vec![0; len];
    let mut gamma = String::new();

    for num in f.split_whitespace() {
        for (i, bit) in num.chars().enumerate() {
            match bit {
                '0' => zeros[i] += 1,
                '1' => ones[i] += 1,
                _ => println!("Unexpected char"),
            }
        }
    }

    for i in 0..len {
        if zeros[i] > ones[i] {
            gamma.push('0');
        } else {
            gamma.push('1');
        }
    }

    let gamma = u64::from_str_radix(&gamma, 2).unwrap();
    let bitmask = (1 << len) - 1;
    let epsilon = !gamma & bitmask;

    gamma * epsilon
}

fn find_most_common_in_bit(input: &[&str], bit: usize) -> char {
    let mut ones = 0;
    let mut zeros = 0;

    for num in input {
        match num.as_bytes()[bit] as char {
            '0' => zeros += 1,
            '1' => ones += 1,
            _ => println!("Unexpected char"),
        }
    }

    match zeros.cmp(&ones) {
        Ordering::Less => '1',
        Ordering::Greater => '0',
        Ordering::Equal => '=',
    }
}

fn part2() -> u64 {
    let f = fs::read_to_string("input/day3.txt").unwrap();

    let mut input_vec: Vec<&str> = f.split_whitespace().collect();
    let mut current_bit = 0;

    // calc oxygen
    while input_vec.len() > 1 {
        let result = find_most_common_in_bit(&input_vec, current_bit);
        let keep_char = match result {
            '0' => '0',
            _ => '1',
        };
        input_vec.retain(|&num| num.as_bytes()[current_bit] as char == keep_char);
        current_bit += 1;
    }

    let oxygen = u64::from_str_radix(input_vec[0], 2).unwrap();

    // calc co2
    let mut input_vec: Vec<&str> = f.split_whitespace().collect();
    let mut current_bit = 0;
    while input_vec.len() > 1 {
        let result = find_most_common_in_bit(&input_vec, current_bit);
        let keep_char = match result {
            '0' => '1',
            _ => '0',
        };
        input_vec.retain(|&num| num.as_bytes()[current_bit] as char == keep_char);
        current_bit += 1;
    }

    let co2 = u64::from_str_radix(input_vec[0], 2).unwrap();

    oxygen * co2
}
