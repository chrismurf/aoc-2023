use std::collections::HashSet;
use std::str::FromStr;
use std::{io, fs, cmp};

#[derive(Debug, Clone)]
pub struct Error;

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct Range {
    start: u64,
    length: u64,
    end: u64
}

impl Range {
    fn new(start: u64, length: u64) -> Self {
        let end = start + length - 1;
        Self { start, length, end }
    }
}

#[derive(Debug, Default, Clone)]
pub struct RangedLookup {
    destination: u64,
    source: Range,
}

impl FromStr for RangedLookup {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts : Vec<&str> = s.split_ascii_whitespace().collect();
        let destination = parts[0].parse().unwrap();
        let source = Range::new(parts[1].parse().unwrap(), parts[2].parse().unwrap());
        Ok(RangedLookup {destination, source})
    }
}

impl RangedLookup {
    fn lookup(&self, value: u64) -> Option<u64> {
        if value >= self.source.start && value <= self.source.end {
            return Some(value - self.source.start + self.destination);
        }
        None
    }

    // Returns tuple of (mutated_range, unprocessed_ranges)
    fn range_lookup(&self, range: Range) -> (Option<Range>, HashSet<Range>) {
        let overlap_start = cmp::max(self.source.start, range.start);
        let overlap_end = cmp::min(self.source.end, range.end);
        if overlap_end < overlap_start { return (None, HashSet::from([range])) }
        let overlap_length = overlap_end - overlap_start + 1;

        let mutated = Some(
            Range::new(overlap_start - self.source.start + self.destination, overlap_length)
        );
        let mut unprocessed = HashSet::new();
        if overlap_start > range.start { unprocessed.insert(
            Range::new(range.start, overlap_start-range.start)
        );}
        if overlap_end < range.end { unprocessed.insert(
            Range::new(overlap_end+1, range.end - overlap_end)
        );}

        return (mutated, unprocessed)
    }
}

#[derive(Debug, Default, Clone)]
pub struct AlmanacSection {
    lookups: Vec<RangedLookup>,
}

impl FromStr for AlmanacSection {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lookups: Vec<RangedLookup> = s.lines()
            .skip(1)
            .map(|x| x.parse().unwrap())
            .collect();

            lookups.sort_by_key(|x| x.source.start);

        Ok(AlmanacSection{lookups})
    }
}

impl AlmanacSection {
    fn lookup(&self, value: u64) -> u64 {
        for lookup in &self.lookups {
            let result = lookup.lookup(value);
            if result.is_some() { return result.unwrap(); }
        }
        value // Unmapped values get passed straight through
    }

    // This should really be a set, since ordering doesn't matter...  But this works.
    fn range_lookup(&self, to_lookup : Range) -> HashSet<Range> {
        let mut mutated_ranges: HashSet<Range> = HashSet::new();
        let mut remainder = HashSet::from([to_lookup]);
        for lookup in &self.lookups {
            let mut new_remainders = HashSet::new();
            for range in remainder {
                let (mutated_range, mut new_remainder) = lookup.range_lookup(range);
                if let Some(r) = mutated_range { mutated_ranges.insert(r); }
                new_remainders.extend(new_remainder);
            }
            remainder = new_remainders;
        }
        mutated_ranges.extend(remainder);// Unmapped values get passed straight through
        return mutated_ranges;
    }
}

#[derive(Debug, Default, Clone)]
pub struct Almanac {
    seeds: Vec<u64>,
    almanac_sections: Vec<AlmanacSection>
}

impl FromStr for Almanac {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let str_sections: Vec<&str> = s.split("\n\n").collect();

        let seeds = str_sections[0]
            .split_ascii_whitespace()
            .filter_map(|x| x.parse().ok())
            .collect();

        let almanac_sections = str_sections[1..].iter().map(|x| x.parse().unwrap()).collect();        
        Ok(Almanac {seeds, almanac_sections})
    }
}

impl Almanac {
    fn location_for_seed(&self, seed: u64) -> u64 {
        let mut current_value = seed;
        for section in &self.almanac_sections {
            current_value = section.lookup(current_value);
        }
        current_value
    }

    fn locations_for_range(&self, range: Range) -> HashSet<Range> {
        let mut ranges = HashSet::from([range]);
        for section in &self.almanac_sections {
            let mut next_ranges = HashSet::new();
            for range in ranges {
                next_ranges.extend(section.range_lookup(range) );
            }
            ranges = next_ranges;
        }
        ranges
    }
}

fn day05() {
    // Parse
    let almanac: Almanac = fs::read_to_string("input.txt").unwrap().parse().unwrap();

    // Part 1
    let mut min_location : u64 = u64::MAX;
    for seed in &almanac.seeds {
        let location = almanac.location_for_seed(*seed);
        if location < min_location {
            min_location = location;
        }
    }
    println!("Part 1: {:?}", min_location);

    // Part 2
    let seed_ranges: Vec<Range> = almanac.seeds.chunks(2).map(|r| Range::new(r[0], r[1])).collect();
    let mut min_location : u64 = u64::MAX;
    for range in seed_ranges {
        let locations = almanac.locations_for_range(range);
        for location in locations {
            if location.start < min_location { min_location = location.start; }
        }
    }
    println!("Part 2: {:?}", min_location);

}

fn main() -> io::Result<()> {
    day05();

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_tests() {
        let almanac: Almanac = fs::read_to_string("test_input.txt").unwrap().parse().unwrap();
        assert_eq!(almanac.seeds, vec![79, 14, 55, 13]);
        assert_eq!(almanac.location_for_seed(79), 82);
    }

    #[test]
    fn range_math_tests() {
        let lookup : RangedLookup = RangedLookup { destination: 50, source: Range::new(3, 4) };
        let (mutated, leftover) = lookup.range_lookup( Range::new(2, 3) );
        assert_eq!(mutated, Some(Range::new(50, 2)));
        assert_eq!(leftover, HashSet::from([Range::new(2, 1)]));

        let (mutated, leftover) = lookup.range_lookup( Range::new(5, 4) );
        assert_eq!(mutated, Some(Range::new(52, 2)));
        assert_eq!(leftover, HashSet::from([Range::new(7, 2)]));

        let (mutated, leftover) = lookup.range_lookup( Range::new(5, 1) );
        assert_eq!(mutated, Some(Range::new(52, 1)));
        assert_eq!(leftover, HashSet::from([]));

        let (mutated, leftover) = lookup.range_lookup( Range::new(1, 10) );
        assert_eq!(mutated, Some(Range::new(50, 4)));
        assert_eq!(leftover, HashSet::from([Range::new(1, 2), Range::new(7, 4)]));
    }

    #[test]
    fn almanac_section_math_tests() {
        let section = AlmanacSection { lookups: vec![
            RangedLookup { destination: 10, source: Range::new(3, 2) },
            RangedLookup { destination: 50, source: Range::new(7, 3) }
        ] };

        let mutated = section.range_lookup(Range::new(1, 12));
        assert_eq!(mutated, HashSet::from([Range::new(1, 2), Range::new(5, 2), Range::new(10, 3), Range::new(10, 2), Range::new(50, 3)]));
    }
}
