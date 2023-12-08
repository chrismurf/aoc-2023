use std::collections::HashMap;
use std::{io, fs};

#[derive(Debug, Clone)]
pub struct Error;

#[derive(PartialEq, PartialOrd, Eq, Ord, Debug, Clone, Hash)]
enum Card { Joker, Two, Three, Four, Five, Six, Seven, Eight, Nine, Ten, Jack, Queen, King, Ace }

impl Card {
    fn from_char(ch: char, j_is_joker: bool) -> Result<Self, Error> {
        match ch {
            '2' => Ok(Card::Two),
            '3' => Ok(Card::Three),
            '4' => Ok(Card::Four),
            '5' => Ok(Card::Five),
            '6' => Ok(Card::Six),
            '7' => Ok(Card::Seven),
            '8' => Ok(Card::Eight),
            '9' => Ok(Card::Nine),
            'T' => Ok(Card::Ten),
            'J' => if j_is_joker { Ok(Card::Joker) } else { Ok(Card::Jack) },
            'Q' => Ok(Card::Queen),
            'K' => Ok(Card::King),
            'A' => Ok(Card::Ace),
            _ => Err(Error { })
        }
    }
}


#[derive(PartialEq, PartialOrd, Eq, Ord, Debug, Clone, Hash)]
pub enum HandType { HighCard, Pair, TwoPair, ThreeKind, FullHouse, FourKind, FiveKind }

impl HandType {
    fn from_cards(cards: &Vec<Card>) -> Result<Self, Error> {
        let mut distribution = HashMap::new();
        for ch in cards {
            distribution.entry(ch).and_modify(|count| *count += 1).or_insert(1);
        }

        // If we have any jokers, pull them out, and we'll make them whatever card we have the most of
        let joker_count = distribution.remove(&Card::Joker).or(Some(0)).unwrap();

        let mut counts : Vec<u8> = distribution.values().map(|x| *x).collect();
        counts.sort();

        // Add the jokers to whatever we have the most of... if we have nothing, then 
        if counts.is_empty() {
            counts = vec![joker_count];
        } else {
            *counts.last_mut().unwrap() += joker_count;
        }

        return match counts[..] {
            [5] => Ok(HandType::FiveKind),
            [1, 4] => Ok(HandType::FourKind),
            [2, 3] => Ok(HandType::FullHouse),
            [1, 1, 3] => Ok(HandType::ThreeKind),
            [1, 2, 2] => Ok(HandType::TwoPair),
            [1, 1, 1, 2] => Ok(HandType::Pair),
            [1, 1, 1, 1, 1] => Ok(HandType::HighCard),
            _ => {
                println!("Died with {:?}", counts);
                return Err(Error { });
            }
        }
    }
}

#[derive(PartialEq, PartialOrd, Eq, Ord, Debug, Clone)]
pub struct Hand {
    hand_type: HandType,
    cards: Vec<Card>,
}

impl Hand {
    fn from_str(s: &str, j_is_joker: bool) -> Result<Self, Error> {
        let cards = s.chars().map(|ch| Card::from_char(ch, j_is_joker).unwrap()).collect();
        let hand_type = HandType::from_cards(&cards).unwrap();
        Ok(Hand { hand_type, cards })
    }
}

#[derive(Debug, Clone)]
pub struct Play {
    hand: Hand,
    bid: u64,
}

impl Play {
    fn from_str(s: &str, j_is_joker: bool) -> Result<Self, Error> {
        let parts : Vec<&str> = s.split_ascii_whitespace().collect();
        Ok(Play {
            hand: Hand::from_str(parts[0], j_is_joker).unwrap(),
            bid: parts[1].parse().unwrap()
        })
    }
}

fn compute_total(lines: &Vec<String>, j_is_joker: bool) -> u64 {
    // Parse
    let mut plays : Vec<Play> = lines.iter().map(|s| Play::from_str(s, j_is_joker).unwrap()).collect();

    plays.sort_by(|a, b| a.hand.partial_cmp(&b.hand).unwrap());

    return plays.iter()
        .enumerate()
        .fold(0, |total, (i, play)| total + ((i as u64+1) * play.bid));
}

fn day07() {
    let lines : Vec<String> = fs::read_to_string("input.txt").unwrap().lines().map(|x| x.to_owned()).collect();
    println!("Part 1: {}", compute_total(&lines, false));
    println!("Part 2: {}", compute_total(&lines, true));
}

fn main() -> io::Result<()> {
    day07();

    Ok(())
}