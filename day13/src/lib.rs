#![warn(clippy::pedantic)]
#![warn(clippy::style)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]
#![warn(clippy::use_self)]

use winnow::{ascii::line_ending, combinator::separated, token::take_while, PResult, Parser};

fn parse_map(input: &mut &str) -> PResult<Vec<String>> {
    separated(
        1..,
        take_while(1.., |c: char| !c.is_whitespace()).map(&str::to_owned),
        line_ending,
    )
    .parse_next(input)
}

fn parse_maps(input: &mut &str) -> PResult<Vec<Vec<String>>> {
    separated(1.., parse_map, (line_ending, line_ending)).parse_next(input)
}

#[must_use]
pub fn solve_a(input: &str) -> usize {
    let maps = parse_maps.parse(input).unwrap();

    maps.into_iter()
        .map(|m| {
            if let Some(value) = find_mirror_rows(&m) {
                return (value + 1) * 100;
            }

            // transpose
            let mut transpose: Vec<String> = vec![];
            for s in &m {
                for (x, c) in s.chars().enumerate() {
                    if let Some(s) = transpose.get_mut(x) {
                        s.push(c);
                    } else {
                        transpose.push(c.to_string());
                    }
                }
            }

            // try again
            if let Some(value) = find_mirror_rows(&transpose) {
                return value + 1;
            }
            panic!("no mirror found for {m:?}")
        })
        .sum()
}

fn find_mirror_rows(rows: &[String]) -> Option<usize> {
    (0..(rows.len() - 1)).find(|&i| {
        (0..=i).all(|j| {
            let mirror_row = i * 2 - j + 1;
            if let Some(right) = rows.get(mirror_row) {
                right == &rows[j]
            } else {
                true
            }
        })
    })
}

#[must_use]
pub fn solve_b(input: &str) -> usize {
    let maps = parse_maps.parse(input).unwrap();

    maps.into_iter()
        .map(|m| {
            if let Some(value) = find_smudge_rows(&m) {
                return (value + 1) * 100;
            }

            // transpose
            let mut transpose: Vec<String> = vec![];
            for s in &m {
                for (x, c) in s.chars().enumerate() {
                    if let Some(s) = transpose.get_mut(x) {
                        s.push(c);
                    } else {
                        transpose.push(c.to_string());
                    }
                }
            }

            // try again
            if let Some(value) = find_smudge_rows(&transpose) {
                return value + 1;
            }
            panic!("no mirror found for {m:?}")
        })
        .sum()
}

fn find_smudge_rows(rows: &[String]) -> Option<usize> {
    (0..(rows.len() - 1)).find(|&i| {
        let mut single_error = false;
        (0..=i).all(|j| {
            let mirror_row = i * 2 - j + 1;
            if let Some(right) = rows.get(mirror_row) {
                if right == &rows[j] {
                    true
                } else if !single_error && single_difference(right, &rows[j]) {
                    single_error = true;
                    true
                } else {
                    false
                }
            } else {
                true
            }
        }) && single_error
    })
}

fn single_difference(right: &str, left: &str) -> bool {
    if right.len() != left.len() {
        return false;
    }

    let mut single_error = false;
    for (r, l) in right.chars().zip(left.chars()) {
        if r != l && !single_error {
            single_error = true;
        } else if r != l && single_error {
            return false;
        }
    }
    single_error
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
        assert_eq!(solve_a(INPUT), 405);
    }

    #[test]
    fn example_b() {
        assert_eq!(solve_b(INPUT), 400);
    }

    #[test]
    fn solution_a() {
        assert_eq!(solve_a(include_str!("input.txt")), 37_561);
    }

    #[test]
    fn solution_b() {
        assert_eq!(solve_b(include_str!("input.txt")), 31_108);
    }
}
