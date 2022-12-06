use aoc2022::day1::day1;
use aoc2022::day2::day2;
use aoc2022::day3::day3;
use aoc2022::day4::day4;
use aoc2022::day5::day5;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    let problem = args.get(1).map(|s| s.as_str()).unwrap_or("None");

    let result: String = match problem {
        "day1" => day1(),
        "day2" => day2(),
        "day3" => day3(),
        "day4" => day4(),
        "day5" => day5(),
        _ => "Not solved yet".to_string(),
    };

    println!("{}", result);
}
