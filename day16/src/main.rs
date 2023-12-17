use std::collections::HashSet;
use std::{fs, io};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum NESW {
    North,
    East,
    South,
    West,
}

#[derive(Debug, Copy, Clone)]
pub enum TileType {
    Empty,
    MirrorSWNE,
    MirrorNWSE,
    SplitVert,
    SplitHoriz,
}

#[derive(Debug, Clone)]
pub struct Tile {
    tile_type: TileType,
    active_inputs: HashSet<NESW>,
}

impl Tile {
    fn from_char(ch: &char) -> Tile {
        Tile {
            tile_type: match ch {
                '.' => TileType::Empty,
                '/' => TileType::MirrorSWNE,
                '\\' => TileType::MirrorNWSE,
                '|' => TileType::SplitVert,
                '-' => TileType::SplitHoriz,
                _ => unreachable!(),
            },
            active_inputs: HashSet::new(),
        }
    }

    fn to_char(&self) -> char {
        match self.tile_type {
            TileType::Empty => match self.active_inputs.len() {
                0 => '.',
                1 => match self.active_inputs.iter().next().unwrap() {
                    NESW::North => '^',
                    NESW::East => '>',
                    NESW::South => 'v',
                    NESW::West => '<',
                },
                2 => '2',
                3 => '3',
                4 => '4',
                _ => '?',
            },
            TileType::MirrorSWNE => '/',
            TileType::MirrorNWSE => '\\',
            TileType::SplitVert => '|',
            TileType::SplitHoriz => '-',
        }
    }

    fn activate(&mut self, light_dir: &NESW) -> Vec<NESW> {
        // Store input light direction
        self.active_inputs.insert(*light_dir);

        // Return the output directions
        match self.tile_type {
            TileType::Empty => match light_dir {
                NESW::North => vec![NESW::North],
                NESW::East => vec![NESW::East],
                NESW::South => vec![NESW::South],
                NESW::West => vec![NESW::West],
            },

            TileType::MirrorSWNE => match light_dir {
                NESW::North => vec![NESW::East],
                NESW::East => vec![NESW::North],
                NESW::South => vec![NESW::West],
                NESW::West => vec![NESW::South],
            },

            TileType::MirrorNWSE => match light_dir {
                NESW::North => vec![NESW::West],
                NESW::East => vec![NESW::South],
                NESW::South => vec![NESW::East],
                NESW::West => vec![NESW::North],
            },

            TileType::SplitVert => match light_dir {
                NESW::North => vec![NESW::North],
                NESW::East => vec![NESW::North, NESW::South],
                NESW::South => vec![NESW::South],
                NESW::West => vec![NESW::North, NESW::South],
            },

            TileType::SplitHoriz => match light_dir {
                NESW::North => vec![NESW::East, NESW::West],
                NESW::East => vec![NESW::East],
                NESW::South => vec![NESW::East, NESW::West],
                NESW::West => vec![NESW::West],
            },
        }
    }
}

struct Map {
    tiles: Vec<Vec<Tile>>,
    wavefronts: HashSet<(usize, usize, NESW)>,
}

impl Map {
    fn from_str(s: &str, start: (usize, usize, NESW)) -> Self {
        Self {
            tiles: s
                .lines()
                .map(|l| l.chars().map(|ch| Tile::from_char(&ch)).collect())
                .collect(),

            wavefronts: HashSet::from([start]),
        }
    }

    fn to_str(&self) -> String {
        let lines: Vec<String> = self
            .tiles
            .iter()
            .map(|line| line.iter().map(|t| t.to_char()).collect())
            .collect();

        lines.join("\n")
    }

    fn to_energized_str(&self) -> String {
        let lines: Vec<String> = self
            .tiles
            .iter()
            .map(|line| {
                line.iter()
                    .map(|t| if t.active_inputs.is_empty() { '.' } else { '#' })
                    .collect()
            })
            .collect();

        lines.join("\n")
    }

    fn advance(&mut self) {
        let mut new_wf = HashSet::new();
        for (r, c, nesw) in &self.wavefronts {
            for out_dir in self.tiles[*r][*c].activate(&nesw) {
                if matches!(out_dir, NESW::North) && *r > 0 {
                    if !self.tiles[*r - 1][*c].active_inputs.contains(&out_dir) {
                        new_wf.insert((*r - 1, *c, out_dir));
                    }
                } else if matches!(out_dir, NESW::East) && *c < self.tiles[0].len() - 1 {
                    if !self.tiles[*r][*c + 1].active_inputs.contains(&out_dir) {
                        new_wf.insert((*r, *c + 1, out_dir));
                    }
                } else if matches!(out_dir, NESW::South) && *r < self.tiles.len() - 1 {
                    if !self.tiles[*r + 1][*c].active_inputs.contains(&out_dir) {
                        new_wf.insert((*r + 1, *c, out_dir));
                    }
                } else if matches!(out_dir, NESW::West) && *c > 0 {
                    if !self.tiles[*r][*c - 1].active_inputs.contains(&out_dir) {
                        new_wf.insert((*r, *c - 1, out_dir));
                    }
                }
            }
        }
        self.wavefronts = new_wf;
    }

    fn energize(&mut self) -> usize {
        while !self.wavefronts.is_empty() {
            self.advance();
        }
        self.tiles
            .iter()
            .map(|row| row.iter().filter(|t| !t.active_inputs.is_empty()).count())
            .sum()
    }
}

fn day16() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut map = Map::from_str(&input, (0usize, 0usize, NESW::East));

    let part1 = map.energize();
    println!("Part 1: {}", part1);

    let mut start_points: Vec<(usize, usize, NESW)> = vec![];
    start_points.append(
        &mut (0..map.tiles.len())
            .map(|r| (r, 0usize, NESW::East))
            .collect(),
    );
    start_points.append(
        &mut (0..map.tiles.len())
            .map(|r| (r, map.tiles[0].len() - 1, NESW::West))
            .collect(),
    );
    start_points.append(
        &mut (0..map.tiles[0].len())
            .map(|c| (0usize, c, NESW::South))
            .collect(),
    );
    start_points.append(
        &mut (0..map.tiles[0].len())
            .map(|c| (map.tiles.len() - 1, c, NESW::North))
            .collect(),
    );

    let part2 = start_points
        .iter()
        .map(|start| Map::from_str(&input, *start).energize())
        .max()
        .unwrap();
    println!("Part 2: {}", part2);
}

fn main() -> io::Result<()> {
    day16();

    Ok(())
}
