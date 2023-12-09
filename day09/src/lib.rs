#![warn(clippy::pedantic)]
#![warn(clippy::style)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]
#![warn(clippy::use_self)]

use itertools::Itertools;
use winnow::{
    ascii::{dec_int, line_ending},
    combinator::separated,
    PResult, Parser,
};

fn parse_line(input: &mut &str) -> PResult<Vec<i32>> {
    separated(1.., dec_int::<_, i32, _>, ' ').parse_next(input)
}

fn parse_lines(input: &mut &str) -> PResult<Vec<Vec<i32>>> {
    separated(1.., parse_line, line_ending).parse_next(input)
}

fn predict(items: Vec<i32>) -> i32 {
    if items.iter().all(|&i| i == 0) {
        0
    } else {
        items.last().copied().unwrap()
            + predict(
                items
                    .into_iter()
                    .tuple_windows()
                    .map(|(a, b)| b - a)
                    .collect_vec(),
            )
    }
}

#[must_use]
pub fn solve_a(input: &str) -> i32 {
    let lines = parse_lines.parse(input).unwrap();
    lines.into_iter().map(predict).sum()
}

#[must_use]
pub fn solve_b(input: &str) -> i32 {
    let lines = parse_lines.parse(input).unwrap();
    lines
        .into_iter()
        .map(|l| l.into_iter().rev().collect_vec())
        .map(predict)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
    #[test]
    fn example_a() {
        assert_eq!(solve_a(INPUT), 114);
    }

    #[test]
    fn example_b() {
        assert_eq!(solve_b(INPUT), 2);
    }

    #[test]
    fn solution_a() {
        assert_eq!(solve_a(include_str!("input.txt")), 1_974_232_246);
    }

    #[test]
    fn solution_b() {
        assert_eq!(solve_b(include_str!("input.txt")), 928);
    }
}
