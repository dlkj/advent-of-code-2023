#![warn(clippy::pedantic)]
#![warn(clippy::style)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]
#![warn(clippy::use_self)]

use std::ops::Range;

use itertools::Itertools;
use winnow::{
    ascii::dec_uint,
    combinator::{alt, delimited, repeat},
    token::any,
    Located, PResult, Parser,
};

#[derive(Debug)]
enum Token {
    Number(u32),
    Symbol(char),
}

fn parse_dots(input: &mut Located<&str>) -> PResult<()> {
    repeat(0.., '.').parse_next(input)
}

fn parse_line(input: &mut Located<&str>) -> PResult<Vec<(Token, Range<usize>)>> {
    repeat(
        0..,
        delimited(
            parse_dots,
            alt((
                dec_uint.map(Token::Number).with_span(),
                any.map(Token::Symbol).with_span(),
            )),
            parse_dots,
        ),
    )
    .parse_next(input)
}

#[must_use]
pub fn solve_a(input: &str) -> u32 {
    let rows = input
        .lines()
        .map(|line| parse_line.parse(Located::new(line)).unwrap())
        .enumerate()
        .collect_vec();

    let symbol_coordinates = rows
        .iter()
        .flat_map(|(y, row)| {
            row.iter().filter_map(|(t, r)| {
                if let Token::Symbol(_) = t {
                    Some((r.start, *y))
                } else {
                    None
                }
            })
        })
        .collect_vec();

    rows.iter()
        .flat_map(|(y, row)| {
            row.iter().filter_map(|(t, r)| {
                if let Token::Number(n) = t {
                    let x_range = Range {
                        start: r.start.saturating_sub(1),
                        end: r.end.saturating_add(1),
                    };

                    let y_range = Range {
                        start: y.saturating_sub(1),
                        end: y.saturating_add(2), //range is exclusive above
                    };

                    symbol_coordinates
                        .iter()
                        .any(|(symbol_x, symbol_y)| {
                            x_range.contains(symbol_x) && y_range.contains(symbol_y)
                        })
                        .then_some(n)
                } else {
                    None
                }
            })
        })
        .sum()
}

#[must_use]
pub fn solve_b(input: &str) -> u32 {
    let rows = input
        .lines()
        .map(|line| parse_line.parse(Located::new(line)).unwrap())
        .enumerate()
        .collect_vec();

    let number_areas = rows
        .iter()
        .flat_map(|(y, row)| {
            row.iter().filter_map(|(t, r)| {
                if let Token::Number(n) = t {
                    Some((
                        *n,
                        Range {
                            start: r.start.saturating_sub(1),
                            end: r.end.saturating_add(1),
                        },
                        Range {
                            start: y.saturating_sub(1),
                            end: y.saturating_add(2), //range is exclusive above
                        },
                    ))
                } else {
                    None
                }
            })
        })
        .collect_vec();

    rows.iter()
        .flat_map(|(y, row)| {
            row.iter().filter_map(|(t, r)| match t {
                Token::Symbol('*') => number_areas
                    .iter()
                    .filter_map(|(n, num_x, num_y)| {
                        (num_x.contains(&r.start) && num_y.contains(y)).then_some(n)
                    })
                    .collect_tuple()
                    .map(|(a, b)| a * b),
                _ => None,
            })
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    #[test]
    fn example_a() {
        assert_eq!(solve_a(INPUT), 4361);
    }

    #[test]
    fn example_b() {
        assert_eq!(solve_b(INPUT), 467_835);
    }

    #[test]
    fn solution_a() {
        assert_eq!(solve_a(include_str!("input.txt")), 554_003);
    }

    #[test]
    fn solution_b() {
        assert_eq!(solve_b(include_str!("input.txt")), 87_263_515);
    }
}
