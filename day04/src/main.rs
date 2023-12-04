use std::collections::{HashSet, HashMap};
use std::str::FromStr;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

#[derive(Debug, Clone)]
pub struct Error;

#[derive(Debug, Default, Clone)]
pub struct GameCard {
    id: u32,
    winners: HashSet<u32>,
    mine: HashSet<u32>
}

impl FromStr for GameCard {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts : Vec<&str> = s.split_ascii_whitespace().collect();
        let id : u32 = parts[1].strip_suffix(":").unwrap().parse().unwrap();
        let winners: HashSet<u32> = parts[2..=11].iter().map(|x| x.parse().unwrap()).collect();
        let mine: HashSet<u32> = parts[13..].iter().map(|x| x.parse().unwrap()).collect();
        Ok(GameCard {id, winners, mine})
    }
}

impl GameCard {
    fn matches(&self) -> u32 {
        self.winners.intersection(&self.mine).count() as u32
    }

    fn score(&self) -> u32 {
        let count = self.matches();
        if count > 0 { return 1 << (count - 1); } else { return 0; }
    }

    fn score_and_redeem(&self, gamecards: &HashMap<u32, GameCard>) -> Vec<GameCard> {
        (self.id+1 ..= self.id + self.matches()).map(|id| gamecards[&id].clone()).collect()
    }
}

fn day04() {
    let file = File::open("input.txt")
        .unwrap_or_else(|_| panic!("File 'input.txt' not readable.") );

    // Collect a HashMap of all the gamecards
    let gamecards : HashMap<u32, GameCard> = BufReader::new(file).lines()
        .filter_map(|line| (line.ok()))
        .map(|line| GameCard::from_str(&line).unwrap())
        .map(|card| (card.id, card))
        .collect();
    
    // Iterate through game cards, and sum up their scores.
    let part1_total : u32 = gamecards.iter()
        .map(|(_, card)| card.score())
        .sum();

    // Output part 1 answer
    println!("Part 1: {:?}", part1_total);

    // Create a vector of the cards in order (since order matters for the first part)
    let mut remaining_cards : Vec<GameCard> = (1..=gamecards.len() as u32).map(|id| gamecards[&id].clone()).collect();
    let mut total_scratchcards = 0u32;

    // As long as we have cards remaining, play them, keeping track of how many we create each round
    while !remaining_cards.is_empty() {
        total_scratchcards += remaining_cards.len() as u32;

        remaining_cards = remaining_cards.iter()
            .flat_map(|card| card.score_and_redeem(&gamecards))
            .collect();
    }
    println!("Part 2: {:?}", total_scratchcards);
}

fn main() -> io::Result<()> {
    day04();

    Ok(())
}
