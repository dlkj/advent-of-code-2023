#![warn(clippy::pedantic)]
#![warn(clippy::style)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]
#![warn(clippy::use_self)]

use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{all_consuming, opt, value},
    multi::{fold_many1, separated_list1},
    sequence::{delimited, pair, terminated},
    IResult,
};

#[derive(Debug, Default)]
struct CountSummary {
    red: u32,
    green: u32,
    blue: u32,
}

fn read_game(input: &str) -> (u32, Vec<CountSummary>) {
    #[derive(Clone, Copy, Debug)]
    enum CubeColors {
        Red,
        Green,
        Blue,
    }

    let rounds: IResult<_, _> = {
        let parse_round = fold_many1(
            terminated(
                pair(
                    nom::character::complete::u32,
                    alt((
                        value(CubeColors::Red, tag(" red")),
                        value(CubeColors::Green, tag(" green")),
                        value(CubeColors::Blue, tag(" blue")),
                    )),
                ),
                opt(tag(", ")),
            ),
            CountSummary::default,
            |mut r, (count, color)| {
                match color {
                    CubeColors::Red => r.red += count,
                    CubeColors::Green => r.green += count,
                    CubeColors::Blue => r.blue += count,
                };
                r
            },
        );

        all_consuming(pair(
            delimited(tag("Game "), nom::character::complete::u32, tag(": ")),
            separated_list1(tag("; "), parse_round),
        ))(input)
    };

    let (_, (id, rounds)) = rounds.unwrap();

    (id, rounds)
}

#[must_use]
pub fn solve_a(input: &str) -> u32 {
    let target = CountSummary {
        red: 12,
        green: 13,
        blue: 14,
    };

    input
        .lines()
        .map(read_game)
        .filter_map(|(id, rounds)| {
            rounds
                .into_iter()
                .all(|round| {
                    round.red <= target.red
                        && round.green <= target.green
                        && round.blue <= target.blue
                })
                .then_some(id)
        })
        .sum()
}

#[must_use]
pub fn solve_b(input: &str) -> u32 {
    input
        .lines()
        .map(read_game)
        .map(|(_, rounds)| {
            let max = rounds
                .into_iter()
                .fold(CountSummary::default(), |c, round| CountSummary {
                    red: c.red.max(round.red),
                    green: c.green.max(round.green),
                    blue: c.blue.max(round.blue),
                });
            max.red * max.green * max.blue
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn example_a() {
        assert_eq!(solve_a(EXAMPLE), 8);
    }

    #[test]
    fn example_b() {
        assert_eq!(solve_b(EXAMPLE), 2286);
    }

    #[test]
    fn solution_a() {
        assert_eq!(solve_a(include_str!("input.txt")), 2101);
    }

    #[test]
    fn solution_b() {
        assert_eq!(solve_b(include_str!("input.txt")), 58269);
    }
}
