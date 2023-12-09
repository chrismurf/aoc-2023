use std::{io, fs};

fn extrapolate(values: &Vec<i64>) -> i64 {
    let mut diffs : Vec<Vec<i64>> = vec![values.clone()];

    loop {
        let last = diffs.last().unwrap();
        if last.iter().all(|x| *x == 0) { break; }
        let len = last.len();
        diffs.push(last[1..].iter().zip(last[..len-1].iter()).map(|(a, b)| a - b).collect());
    }

    diffs.reverse();
    diffs.first_mut().unwrap().push(0);
    
    let mut previous_value = 0;
    for diff in diffs.iter_mut().skip(1) {
        previous_value = diff.last().unwrap() + previous_value;
        diff.push(previous_value);
    }
    return *diffs.last().unwrap().last().unwrap() as i64;
}

fn day09() {
    let input = fs::read_to_string("input.txt").unwrap();
    
    let mut part1: i64 = 0;
    let mut part2: i64 = 0;

    for line in input.lines() {
        let mut values: Vec<i64> = line.split_ascii_whitespace().map(|x| x.parse().unwrap()).collect();
        part1 += extrapolate(&values);
        values.reverse();
        part2 += extrapolate(&values);
    }

    println!("{:?}", part1);
    println!("{:?}", part2);
}

fn main() -> io::Result<()> {
    day09();

    Ok(())
}
