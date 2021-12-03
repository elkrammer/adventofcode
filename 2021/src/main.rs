use aoc2021::day1::day1;
use aoc2021::day2::day2;
use aoc2021::day3::day3;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    let problem = args.get(1).map(|s| s.as_str()).unwrap_or("None");

    let result: String = match problem {
        "day1" => day1(),
        "day2" => day2(),
        "day3" => day3(),
        _ => "Not solved yet".to_string(),
    };

    println!("{}", result);
}
