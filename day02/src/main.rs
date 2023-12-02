use std::str::FromStr;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

#[derive(Debug, Clone)]
pub struct Error;

#[derive(Debug, Default)]
pub struct Round {
    red: u32,
    green: u32,
    blue: u32
}

impl FromStr for Round {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (mut red, mut green, mut blue) = (0, 0, 0);
        for color_count_str in s.split(",") {
            let Some((count_str, color_name)) = color_count_str.trim().split_once(" ") else { continue; };
            let count: u32 = count_str.parse().unwrap();
            match color_name {
                "red" => red += count,
                "green" => green += count,
                "blue" => blue += count,
                _ => unreachable!("Only red / green/ blue should exist.")
            }
        }
        Ok(Self { red, green, blue })
    }
}


#[derive(Debug, Default)]
pub struct Game {
    id: u32,
    rounds: Vec<Round>
}

impl Game {
    fn possible(&self, max_red: u32, max_green: u32, max_blue: u32) -> bool {
        for round in &self.rounds {
            if round.red > max_red || round.blue > max_blue || round.green > max_green { return false; }
        }
        true
    }

    fn power_min_cubes(&self) -> u32 {
        let (mut min_red, mut min_green, mut min_blue) = (0, 0, 0);
        for round in &self.rounds {
            if round.red > min_red { min_red = round.red; }
            if round.green > min_green { min_green = round.green; }
            if round.blue > min_blue { min_blue = round.blue; }
        }

        min_red * min_green * min_blue
    }
}

impl FromStr for Game {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut id = 0;
        let mut rounds = Vec::new();

        for line in s.lines() {
            let Some((game_id_str, rounds_str)) = line.split_once(":") else { continue; };
            let Some((_, id_str)) = game_id_str.split_once(" ") else { continue; };
            id = id_str.parse().unwrap();

            for round_str in rounds_str.split(";") {
                rounds.push(Round::from_str(round_str).unwrap());
            }
        }

        Ok(Self { id, rounds })
    }
}

fn part1() {
    let file = File::open("input.txt")
        .unwrap_or_else(|_| panic!("File 'input.txt' not readable.") );

    let mut part1_total = 0;
    let mut part2_total = 0;
    let mut games = Vec::new();
    
    for line in BufReader::new(file).lines().filter_map(|line| line.ok()) {
        let game = Game::from_str(&line).unwrap();
        if game.possible(12, 13, 14) { part1_total += game.id; }
        part2_total += game.power_min_cubes();
        games.push(game);
    }

    println!("Part 1: {:?}", part1_total);
    println!("Part 1: {:?}", part2_total);
}

fn main() -> io::Result<()> {
    part1();

    Ok(())
}
