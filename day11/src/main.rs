use itertools::Itertools;
use std::collections::HashSet;
use std::{fs, io};

fn manhattan_distance(a: &(usize, usize), b: &(usize, usize)) -> usize {
    (a.0.max(b.0) - a.0.min(b.0)) + (a.1.max(b.1) - a.1.min(b.1))
}

fn empty_rows_crossed(
    a: &(usize, usize),
    b: &(usize, usize),
    empty_rows: &HashSet<usize>,
) -> usize {
    HashSet::from_iter(a.0.min(b.0)..a.0.max(b.0))
        .intersection(empty_rows)
        .count()
}

fn empty_cols_crossed(
    a: &(usize, usize),
    b: &(usize, usize),
    empty_cols: &HashSet<usize>,
) -> usize {
    HashSet::from_iter(a.1.min(b.1)..a.1.max(b.1))
        .intersection(empty_cols)
        .count()
}

fn day11() {
    let input = fs::read_to_string("input.txt").unwrap();

    let mut galaxies: Vec<(usize, usize)> = vec![];
    let n_rows = input.lines().count();
    let n_cols = input.lines().next().unwrap().len();
    let mut empty_rows = HashSet::from_iter(0..n_rows);
    let mut empty_cols = HashSet::from_iter(0..n_cols);

    for (row, line) in input.lines().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            if ch == '#' {
                empty_rows.remove(&row);
                empty_cols.remove(&col);
                galaxies.push((row, col));
            }
        }
    }

    let mut part1: usize = 0;
    let mut part2: usize = 0;

    for pair in galaxies.iter().combinations(2) {
        let a = pair.first().unwrap();
        let b = pair.last().unwrap();
        let dist = manhattan_distance(a, b);
        let rows = empty_rows_crossed(a, b, &empty_rows);
        let cols = empty_cols_crossed(a, b, &empty_cols);
        part1 += dist + rows + cols;
        part2 += dist + 999999 * rows + 999999 * cols;
    }
    println!("TOTAL: {:?}", part1); // 9331020
    println!("TOTAL: {:?}", part2); // 411142919886
}

fn main() -> io::Result<()> {
    day11();

    Ok(())
}
