use std::cmp::{min,max};
use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::collections::HashMap;

pub enum SchematicEntry {
    Digit(u8),
    Symbol(char),
    Empty
}

impl From<char> for SchematicEntry {
    fn from(c: char) -> Self {
        match c {
            '0'..='9' => Self::Digit(c.to_digit(10).unwrap().try_into().unwrap()),
            '.' => Self::Empty,
            _ => Self::Symbol(c)
        }
    }
}

pub struct Schematic {
    data : Vec<Vec<SchematicEntry>>
}

impl Schematic {
    fn new() -> Self {
        let data = vec![];
        Schematic { data }
    }

    fn from_file(path: &str) -> Self {
        let file = File::open(path)
            .unwrap_or_else(|_| panic!("File 'input.txt' not readable.") );

        let mut schematic = Schematic::new();

        for line in BufReader::new(file).lines().filter_map(|line| line.ok()) {
            schematic.append_row(&line);
        }
        schematic
    }

    fn append_row(&mut self, s: &str) {
        self.data.push(s.chars().map(|c| c.into()).collect());
    }

    fn adjacent_symbol(&self, r: i32, c: i32) -> Option<(char, i32, i32)> {
        let last_row = (self.data.len() - 1) as i32;
        let last_col = (self.data[0].len() - 1) as i32;
    
        for i in max(0, r-1) ..= min(last_row, r+1) {
            for j in max(0, c-1) ..= min(last_col, c+1) {
                if let SchematicEntry::Symbol(char) = self.data[i as usize][j as usize] {
                    return Some((char, i, j));
                }
            }
        }
        None    
    }
}

fn part1(schematic: &Schematic) -> u32 {
    let mut sum: u32 = 0;
    // Iterate through each cell.  If we finish a number, check whether it was 
    // ever next to a symbol.  If so, add it to the sum.
    for (r, row) in schematic.data.iter().enumerate() {
        let mut current_part_id: u32 = 0;
        let mut has_adjacent_symbol : bool = false;
        for (c, entry) in row.iter().enumerate() {
            match entry {
                SchematicEntry::Digit(d) => {
                    current_part_id *= 10;
                    current_part_id += *d as u32;
                    has_adjacent_symbol |= schematic.adjacent_symbol(r as i32, c as i32).is_some();
                },
                _ => {
                    if current_part_id != 0 {
                        if has_adjacent_symbol { sum += current_part_id; }
                        current_part_id = 0;
                    }
                    has_adjacent_symbol = false;
                }
            }
        }

        if current_part_id != 0 {
            if has_adjacent_symbol { sum += current_part_id; }
        }
    }
    sum
}

fn part2(schematic: &Schematic) -> u32 {
    let mut gears = HashMap::new();

    // In this case, we keep a list of which numbers were next to each gear.  When we're done scanning
    // the schematic, we then sum up any "gear ratios" that were next to exactly two gears.
    for (r, row) in schematic.data.iter().enumerate() {
        let mut current_part_id: u32 = 0;
        let mut adjacent_gear : Option<(i32, i32)> = None;
        for (c, entry) in row.iter().enumerate() {
            match entry {
                SchematicEntry::Digit(d) => {
                    current_part_id *= 10;
                    current_part_id += *d as u32;
                    if let Some(('*', r, c)) = schematic.adjacent_symbol(r as i32, c as i32) {
                        adjacent_gear = Some((r, c));

                    }
                },
                _ => {
                    if current_part_id != 0 {
                        if let Some(rc) = adjacent_gear {
                            gears.entry(rc).or_insert(vec![]).push(current_part_id);
                        }
                        current_part_id = 0;
                    }
                    adjacent_gear = None;
                }
            }
        }

        if current_part_id != 0 {
            if let Some(rc) = adjacent_gear {
                gears.entry(rc).or_insert(vec![]).push(current_part_id);
            }
        }
    }

    let mut sum = 0;
    for (_, values) in &gears {
        if values.len() == 2 {
            sum += values[0] * values[1];
        }
    }

    sum
}

fn main() -> io::Result<()> {
    let schematic = Schematic::from_file("input.txt");
    println!("Part 1 sum: {}", part1(&schematic));
    println!("Part 2 sum: {}", part2(&schematic));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum() {
        let mut example = Schematic::new();
        example.append_row("467..114.");
        example.append_row("...*.....");
        example.append_row("..35..633");
        example.append_row("......#..");
        example.append_row("617*.....");
        example.append_row(".....+.58");
        example.append_row("..592....");
        example.append_row("......755");
        example.append_row("...$.*...");
        example.append_row(".664.598.");

        assert_eq!(part1(&example), 4361);
        assert_eq!(part2(&example), 467835);
    }
}