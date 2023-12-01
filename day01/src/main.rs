use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn sum_calibration_values() {
    let mut total: u32 = 0;

    let file = File::open("input.txt")
        .unwrap_or_else(|_| panic!("File 'input.txt' not readable.") );

    for line in BufReader::new(file).lines().filter_map(|line| line.ok()) {
        let digits : Vec::<u32> = line.chars()
            .filter_map(|c| match c {
                '0'..='9' => Some(c.to_digit(10).unwrap()),
                _ => None
            })
            .collect();
        total += 10*digits.first().unwrap() + digits.last().unwrap();
    }

    println!("Top value: {}", total);
}

fn main() -> io::Result<()> {
    sum_calibration_values();

    Ok(())
}
