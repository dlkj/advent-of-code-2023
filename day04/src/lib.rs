#![warn(clippy::pedantic)]
#![warn(clippy::style)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]
#![warn(clippy::use_self)]

use std::collections::HashSet;
use winnow::{
    ascii::{dec_uint, multispace0},
    combinator::{delimited, preceded, repeat},
    PResult, Parser,
};

fn parse_line(mut input: &str) -> PResult<(u32, Vec<u32>, Vec<u32>)> {
    (
        delimited(
            "Card",
            delimited(multispace0, dec_uint::<_, u32, _>, multispace0),
            ':',
        ),
        repeat(
            1..,
            delimited(multispace0, dec_uint::<_, u32, _>, multispace0),
        ),
        preceded(
            '|',
            repeat(
                1..,
                delimited(multispace0, dec_uint::<_, u32, _>, multispace0),
            ),
        ),
    )
        .parse_next(&mut input)
}

#[must_use]
pub fn solve_a(input: &str) -> u32 {
    input
        .lines()
        .map(|l| parse_line(l).unwrap())
        .map(|(_, winning, picked)| {
            let winning: HashSet<u32> = winning.iter().copied().collect();
            match picked.iter().filter(|p| winning.contains(p)).count() {
                0 => 0,
                n => 2_u32.pow((n - 1).try_into().unwrap()),
            }
        })
        .sum()
}

#[must_use]
pub fn solve_b(input: &str) -> u32 {
    let winning_games: Vec<usize> = input
        .lines()
        .map(|l| parse_line(l).unwrap())
        .map(|(_, winning, picked)| {
            let winning_numbers: HashSet<u32> = winning.iter().copied().collect();
            let winning_count = picked
                .iter()
                .filter(|p| winning_numbers.contains(p))
                .count();

            winning_count
        })
        .collect();

    let line_count = input.lines().count();
    let mut game_counts = vec![1; line_count];
    let mut result = 0;
    for i in 0..line_count {
        let count = game_counts.get(i).copied().unwrap();
        for game in game_counts
            .iter_mut()
            .skip(i + 1)
            .take(*winning_games.get(i).unwrap())
        {
            *game += count;
        }
        result += count;
    }

    result
}

#[cfg(test)]
mod tests {

    use super::*;
    const INPUT: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
    #[test]
    fn example_a() {
        assert_eq!(solve_a(INPUT), 13);
    }

    #[test]
    fn example_b() {
        assert_eq!(solve_b(INPUT), 30);
    }

    #[test]
    fn solution_a() {
        assert_eq!(solve_a(include_str!("input.txt")), 18653);
    }

    #[test]
    fn solution_b() {
        assert_eq!(solve_b(include_str!("input.txt")), 5_921_508);
    }
}
