#![warn(clippy::pedantic)]
#![warn(clippy::style)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]
#![warn(clippy::use_self)]

use std::{
    collections::{HashMap, HashSet},
    ops::RangeBounds,
    usize,
};

use itertools::Itertools;

fn grid_coords<'a>(lines: impl Iterator<Item = &'a str>, target: char) -> Vec<(usize, usize)> {
    lines
        .enumerate()
        .flat_map(|(y, l)| {
            l.chars()
                .enumerate()
                .filter_map(move |(x, c)| (c == target).then_some((x, y)))
        })
        .collect_vec()
}

#[must_use]
pub fn solve_a(input: &str) -> usize {
    let y_len = input.lines().count();
    let x_len = input.lines().next().unwrap().len();

    let round = grid_coords(input.lines(), 'O');
    let cube = grid_coords(input.lines(), '#');

    // group by column

    let mut round: HashMap<usize, HashSet<_>> = round
        .into_iter()
        .sorted()
        .group_by(|&(x, _)| x)
        .into_iter()
        .map(|(x, g)| (x, g.collect()))
        .collect();

    let cube: HashMap<usize, Vec<_>> = cube
        .into_iter()
        .sorted()
        .group_by(|&(x, _)| x)
        .into_iter()
        .map(|(x, g)| (x, g.collect()))
        .collect();

    for x in 0..x_len {
        let empty_r: &mut HashSet<(usize, usize)> = &mut HashSet::new();
        let empty_c: &Vec<(usize, usize)> = &vec![];
        let round = round.get_mut(&x).unwrap_or(empty_r);
        let cube = cube.get(&x).unwrap_or(empty_c);

        if cube.is_empty() {
            roll(round, .., x);
        } else {
            //do the first #
            let &(_, cy) = cube.first().unwrap();
            roll(round, ..cy, x);

            //do every pair of #
            for (&(_, cy1), &(_, cy2)) in cube.iter().tuple_windows() {
                roll(round, (cy1 + 1)..cy2, x);
            }

            //do the last #
            let &(_, cy) = cube.last().unwrap();
            roll(round, (cy + 1).., x);
        }
    }

    round
        .into_iter()
        .flat_map(|(_, row)| row.into_iter().map(|(_, y)| y_len - y))
        .sum()
}

fn roll(round: &mut HashSet<(usize, usize)>, range: impl RangeBounds<usize>, x: usize) {
    // remove all the stones that are in the range and count them
    let removed_len = round.iter().filter(|(_, y)| range.contains(y)).count();
    round.retain(|(_, y)| !range.contains(y));

    // insert that many at the end or the range
    match range.start_bound() {
        std::ops::Bound::Included(&n) => round.extend((n..(n + removed_len)).map(|n| (x, n))),
        std::ops::Bound::Excluded(&n) => {
            round.extend(((n + 1)..=(n + removed_len)).map(|n| (x, n)));
        }
        std::ops::Bound::Unbounded => round.extend((0..removed_len).map(|n| (x, n))),
    }
}

#[must_use]
pub fn solve_b(input: &str) -> u32 {
    input.len().try_into().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";
    #[test]
    fn example_a() {
        assert_eq!(solve_a(INPUT), 136);
    }

    #[test]
    fn example_b() {
        assert_eq!(solve_b(INPUT), 0);
    }

    #[test]
    fn solution_a() {
        assert_eq!(solve_a(include_str!("input.txt")), 106_997);
    }

    #[test]
    fn solution_b() {
        assert_eq!(solve_b(include_str!("input.txt")), 0);
    }
}
