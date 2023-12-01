use regex::Regex;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn as_digit(v : &str) -> u32 {
    match v {
        "zero" | "0" => 0,
        "one" | "1" => 1,
        "two" | "2" => 2,
        "three" | "3" => 3,
        "four" | "4" => 4,
        "five" | "5" => 5,
        "six" | "6" => 6,
        "seven" | "7" => 7,
        "eight" | "8" => 8,
        "nine" | "9" => 9,
        _ => unimplemented!()
    }
}

fn sum_calibration_values() {
    let mut part1_total: u32 = 0;
    let mut part2_total: u32 = 0;

    let re_p1 = Regex::new(r"[0-9]").unwrap();
    let re = Regex::new(r"(zero|one|two|three|four|five|six|seven|eight|nine|[0-9])").unwrap();
    let rev_re = Regex::new(r"(orez|eno|owt|eerht|ruof|evif|xis|neves|thgie|enin|[0-9])").unwrap();

    let file = File::open("input.txt")
        .unwrap_or_else(|_| panic!("File 'input.txt' not readable.") );

    for line in BufReader::new(file).lines().filter_map(|line| line.ok()) {
        // Part 1
        let first_p1 = as_digit(re_p1.find(&line).unwrap().as_str());
        let reversed_line: String = line.chars().rev().collect();
        let last_p1 = as_digit(re_p1.find(&reversed_line).unwrap().as_str());
        part1_total += 10 * first_p1 + last_p1;

        // Part 2
        // There's no "rfind" for regex, so just reverse everything (including the regex...)
        let first = as_digit(re.find(&line).unwrap().as_str());
        let reversed_line: String = line.chars().rev().collect();
        let reversed_last: String = rev_re.find(&reversed_line).unwrap().as_str().chars().rev().collect();
        let last = as_digit(&reversed_last);
        part2_total += 10*first + last;
    }

    println!("Part 1: {}", part1_total);
    println!("Part 2: {}", part2_total);
}

fn main() -> io::Result<()> {
    sum_calibration_values();

    Ok(())
}
