#![warn(clippy::pedantic)]
#![warn(clippy::style)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]
#![warn(clippy::use_self)]

use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{all_consuming, value},
    error::Error,
    multi::separated_list1,
    sequence::{delimited, pair},
};

#[derive(Clone, Copy, Debug)]
struct CountSummary {
    red: u32,
    green: u32,
    blue: u32,
}

#[derive(Clone, Copy, Debug)]
enum CubeColors {
    Red,
    Green,
    Blue,
}

fn parse_game(input: &str) -> (u32, Vec<CountSummary>) {
    let parse_round = separated_list1(
        tag(", "),
        pair(
            nom::character::complete::u32,
            alt((
                value(CubeColors::Red, tag(" red")),
                value(CubeColors::Green, tag(" green")),
                value(CubeColors::Blue, tag(" blue")),
            )),
        ),
    );

    let mut parse_game = all_consuming(pair(
        delimited(
            tag("Game "),
            nom::character::complete::u32::<_, Error<_>>,
            tag(": "),
        ),
        separated_list1(tag("; "), parse_round),
    ));

    let (_, (id, rounds)) = parse_game(input).unwrap();

    let rounds = rounds
        .iter()
        .map(|r| {
            r.iter().fold(
                CountSummary {
                    red: 0,
                    green: 0,
                    blue: 0,
                },
                |mut r, (count, color)| {
                    match color {
                        CubeColors::Red => r.red += count,
                        CubeColors::Green => r.green += count,
                        CubeColors::Blue => r.blue += count,
                    };
                    r
                },
            )
        })
        .collect_vec();

    (id, rounds)
}

#[must_use]
pub fn solve_a(input: &str) -> u32 {
    let games = input.lines().map(parse_game).collect_vec();

    let target = CountSummary {
        red: 12,
        green: 13,
        blue: 14,
    };

    games
        .iter()
        .filter(|(_, rounds)| {
            rounds.iter().all(|round| {
                round.red <= target.red && round.green <= target.green && round.blue <= target.blue
            })
        })
        .map(|(id, _)| id)
        .sum()
}

#[must_use]
pub fn solve_b(input: &str) -> u32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_a() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        assert_eq!(solve_a(input), 8);
    }

    // #[test]
    // fn example_b() {
    //     let input = "";

    //     assert_eq!(solve_b(input), 0);
    // }

    #[test]
    fn a() {
        assert_eq!(solve_a(include_str!("input.txt")), 0);
    }

    // #[test]
    // fn b() {
    //     assert_eq!(solve_b(include_str!("input.txt")), 0);
    // }
}
