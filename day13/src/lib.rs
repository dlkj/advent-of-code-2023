#![warn(clippy::pedantic)]
#![warn(clippy::style)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]
#![warn(clippy::use_self)]

use winnow::{
    ascii::{line_ending, not_line_ending},
    combinator::separated,
    PResult, Parser,
};

fn parse_map<'a>(input: &mut &'a str) -> PResult<Vec<&'a str>> {
    separated(1.., not_line_ending, line_ending).parse_next(input)
}

fn parse_maps<'a>(input: &mut &'a str) -> PResult<Vec<Vec<&'a str>>> {
    separated(1.., parse_map, (line_ending, line_ending)).parse_next(input)
}

#[must_use]
pub fn solve_a(input: &str) -> usize {
    let maps = parse_maps.parse(input).unwrap();

    for m in maps {
        if let Some(value) = find_mirror_y(&m) {
            return value;
        }

        // transpose and try again
    }

    input.len().try_into().unwrap()
}

fn find_mirror_y(m: &[&str]) -> Option<usize> {
    for i in 0..(m.len() - 1) {
        //then check outwards
        if m[i] == m[i + 1] {
            return Some(i);
        }
    }
    None
}

#[must_use]
pub fn solve_b(input: &str) -> usize {
    input.len().try_into().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";
    #[test]
    fn example_a() {
        assert_eq!(solve_a(INPUT), 0);
    }

    #[test]
    fn example_b() {
        assert_eq!(solve_b(INPUT), 0);
    }

    #[test]
    fn solution_a() {
        assert_eq!(solve_a(include_str!("input.txt")), 0);
    }

    #[test]
    fn solution_b() {
        assert_eq!(solve_b(include_str!("input.txt")), 0);
    }
}
