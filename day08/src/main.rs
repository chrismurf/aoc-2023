use std::{io, fs};
use std::collections::{HashMap, HashSet};
use num::integer::lcm;

fn day08() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut lines = input.lines();

    let instruction_sequence: Vec<char> = lines.next().unwrap().chars().collect();
    lines.next(); // skip blank

    let mut lefts = HashMap::new();
    let mut rights = HashMap::new();

    for line in lines {
        lefts.insert(&line[0..3], &line[7..10]);
        rights.insert(&line[0..3], &line[12..15]);
    }

    // Now solve part 1
    let mut instructions = instruction_sequence.iter().cycle();
    let mut num_steps = 0u32;
    let mut location = "AAA";
    while location != "ZZZ" {
        let instruction = instructions.next().unwrap();
        if *instruction == 'L' {
            location = lefts[location];
        } else {
            location = rights[location];
        }
        num_steps += 1;
    }

    println!("Part 1: {:?}", num_steps);

    // Now solve part 2.  Figure out how long each cycle takes, then find the LCM.
    let start_locations: HashSet<&str> = lefts.keys().filter(|x| x.ends_with("A")).map(|x| *x).collect();
    let mut num_steps = HashMap::new();

    for start_location in start_locations {
        let mut location = start_location;
        instructions = instruction_sequence.iter().cycle();

        while !location.ends_with("Z") {
            let instruction = instructions.next().unwrap();
            if *instruction == 'L' {
                location = lefts[location];
            } else {
                location = rights[location];
            }
            num_steps.entry(start_location).and_modify(|count| *count += 1u64).or_insert(1u64);
        }    
    }

    let part2 = num_steps.values().fold(1, |x, y| lcm(x, *y));
    println!("Part 2: {:?}", part2);

}

fn main() -> io::Result<()> {
    day08();

    Ok(())
}
