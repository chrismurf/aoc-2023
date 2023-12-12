use std::str::FromStr;
use std::{fs, io};

#[derive(Debug, Clone)]
pub struct Error;

#[derive(Debug, Clone)]
enum Direction { North, East, South, West }

#[derive(Debug, Clone)]
pub struct Map {
    start: (usize, usize), // row, col
    cells: Vec<Vec<Cell>>,
}

impl FromStr for Map {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut start = (0usize, 0usize);
        let cells = s
            .lines()
            .enumerate()
            .map(|(row, line)| {
                line.chars()
                    .enumerate()
                    .map(|(col, ch)| {
                        let cell = Cell::from_char((row, col), ch);
                        if cell.start_distance == Some(0) {
                            // Side effect in interation ... little shady.
                            start = (row, col);
                        }
                        return cell;
                    })
                    .collect()
            })
            .collect();

        Ok(Self { start, cells })
    }
}

impl Map {
    fn start(&self) -> &Cell { return &self.cells[self.start.0][self.start.1] }
    fn get(&self, rc: (usize, usize)) -> &Cell { return &self.cells[rc.0][rc.1] }
    fn get_mut(&mut self, rc: (usize, usize)) -> &mut Cell { return &mut self.cells[rc.0][rc.1] }

    fn direction_of(&self, dir: Direction, rc : (usize, usize)) -> (usize, usize) {
        match dir {
            Direction::North => self.north_of(rc),
            Direction::East => self.east_of(rc),
            Direction::South => self.south_of(rc),
            Direction::West => self.west_of(rc),
        }
    }

    fn east_of(&self, rc : (usize, usize)) -> (usize, usize) { return (rc.0, rc.1+1) }
    fn west_of(&self, rc : (usize, usize)) -> (usize, usize) { return (rc.0, rc.1-1) }
    fn north_of(&self, rc : (usize, usize)) -> (usize, usize) { return (rc.0-1, rc.1) }
    fn south_of(&self, rc : (usize, usize)) -> (usize, usize) { return (rc.0+1, rc.1) }
}

#[derive(PartialEq, Eq, Debug, Clone, Hash)]
pub struct Cell {
    rc: (usize, usize), // Row, Column
    start_distance: Option<u16>,
    pipe: PipeDir,
}

impl Cell {
    fn from_char(rc: (usize, usize), ch: char) -> Cell {
        return Cell {
            rc,
            start_distance: if ch == 'S' { Some(0) } else { None },
            pipe: PipeDir::from_char(ch),
        };
    }

    fn traverse(&self, from_rc: (usize, usize)) -> Direction {
        let dy = self.rc.0 as isize - from_rc.0 as isize;
        let dx = self.rc.1 as isize - from_rc.1 as isize;

        match (dy, dx, &self.pipe) {
            (0, 1, PipeDir::EastWest) => Direction::East,
            (0, 1, PipeDir::NorthWest) => Direction::North,
            (0, 1, PipeDir::SouthWest) => Direction::South,
            (0, -1, PipeDir::EastWest) => Direction::West,
            (0, -1, PipeDir::NorthEast) => Direction::North,
            (0, -1, PipeDir::SouthEast) => Direction::South,
            (1, 0, PipeDir::NorthSouth) => Direction::South,
            (1, 0, PipeDir::NorthEast) => Direction::East,
            (1, 0, PipeDir::NorthWest) => Direction::West,
            (-1, 0, PipeDir::NorthSouth) => Direction::North,
            (-1, 0, PipeDir::SouthEast) => Direction::East,
            (-1, 0, PipeDir::SouthWest) => Direction::West,
            _ => unreachable!(),
        }
    }

    fn goes_north(&self) -> bool {
        return self.pipe == PipeDir::NorthSouth
            || self.pipe == PipeDir::NorthEast
            || self.pipe == PipeDir::NorthWest;
    }
    fn goes_south(&self) -> bool {
        return self.pipe == PipeDir::NorthSouth
            || self.pipe == PipeDir::SouthEast
            || self.pipe == PipeDir::SouthWest;
    }
    fn goes_east(&self) -> bool {
        return self.pipe == PipeDir::EastWest
            || self.pipe == PipeDir::NorthEast
            || self.pipe == PipeDir::SouthEast;
    }
    fn goes_west(&self) -> bool {
        return self.pipe == PipeDir::EastWest
            || self.pipe == PipeDir::NorthWest
            || self.pipe == PipeDir::SouthWest;
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Hash)]
enum PipeDir {
    NorthSouth,
    EastWest,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
    None,
}

impl PipeDir {
    fn from_char(ch: char) -> Self {
        return match ch {
            '-' => Self::EastWest,
            '|' => Self::NorthSouth,
            'L' => Self::NorthEast,
            'J' => Self::NorthWest,
            'F' => Self::SouthEast,
            '7' => Self::SouthWest,
            'S' | '.' => Self::None,
            _ => unreachable!(),
        };
    }
}

fn day10() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut map: Map = input.parse().unwrap();

    let mut prev_locations : Vec<(usize, usize)> = vec![];
    let mut locations: Vec<(usize, usize)> = vec![];

    if map.get(map.north_of(map.start)).goes_south() {
        let cell = map.get_mut(map.north_of(map.start));
        cell.start_distance = Some(1);
        locations.push(cell.rc);
        prev_locations.push(map.start);
    }
    if map.get(map.south_of(map.start)).goes_north() {
        let cell = map.get_mut(map.south_of(map.start));
        cell.start_distance = Some(1);
        locations.push(cell.rc);
        prev_locations.push(map.start);
    }
    if map.get(map.east_of(map.start)).goes_west() {
        let cell = map.get_mut(map.east_of(map.start));
        cell.start_distance = Some(1);
        locations.push(cell.rc);
        prev_locations.push(map.start);
    }
    if map.get(map.west_of(map.start)).goes_east() {
        let cell = map.get_mut(map.west_of(map.start));
        cell.start_distance = Some(1);
        locations.push(cell.rc);
        prev_locations.push(map.start);
    }

    let mut current_cost = 2u16;
    loop {
        let mut new_locations = vec![];
        let mut new_prev_locations = vec![];
        for (rc, prev_rc) in locations.iter().zip(prev_locations.iter()) {
            let new_rc = map.direction_of(
                map.get(*rc).traverse(*prev_rc),
                *rc);

            if map.get(new_rc).start_distance.is_none() {
                map.get_mut(new_rc).start_distance = Some(current_cost);
                new_prev_locations.push(*rc);
                new_locations.push(new_rc);
            }
        }
        if new_locations.is_empty() {
            // Minus one because we *didn't* find (and therefore set cost on) any cells this round.
            current_cost -= 1;
            break;
        }

        current_cost += 1;
        prev_locations = new_prev_locations;
        locations = new_locations;
    }
    
    println!("{:?}", current_cost);

    let printable : Vec<String> = map.cells.iter().map(|row| row.iter().map(
        |x| if let Some(dist) = x.start_distance { char::from_digit((dist % 10) as u32, 10).unwrap()  } else { '.' }
    ).collect() ).collect();

    //println!("{:}", input);
    println!("{}", printable.join("\n"));

}

fn main() -> io::Result<()> {
    day10();

    Ok(())
}
