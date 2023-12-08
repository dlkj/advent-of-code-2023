#![warn(clippy::pedantic)]
#![warn(clippy::style)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]
#![warn(clippy::use_self)]

use winnow::ascii::dec_uint;
use winnow::ascii::line_ending;
use winnow::ascii::multispace1;
use winnow::combinator::delimited;
use winnow::combinator::preceded;
use winnow::combinator::repeat;
use winnow::token::take_till;
use winnow::PResult;
use winnow::Parser;

fn parse_input_lines_a(input: &mut &str) -> PResult<(Vec<u64>, Vec<u64>)> {
    (
        delimited(
            "Time:",
            repeat(1.., preceded(multispace1, dec_uint::<_, u64, _>)),
            line_ending,
        ),
        preceded(
            "Distance:",
            repeat(1.., preceded(multispace1, dec_uint::<_, u64, _>)),
        ),
    )
        .parse_next(input)
}

fn parse_input_lines_b(input: &mut &str) -> PResult<(u64, u64)> {
    (
        delimited(
            "Time:",
            preceded(
                multispace1,
                take_till(1.., |c| c == '\n' || c == '\r').map(|s: &str| {
                    s.chars()
                        .filter(|&c| c != ' ')
                        .collect::<String>()
                        .parse::<u64>()
                        .unwrap()
                }),
            ),
            line_ending,
        ),
        preceded(
            "Distance:",
            take_till(1.., |c| c == '\n' || c == '\r').map(|s: &str| {
                s.chars()
                    .filter(|&c| c != ' ')
                    .collect::<String>()
                    .parse::<u64>()
                    .unwrap()
            }),
        ),
    )
        .parse_next(input)
}

#[must_use]
pub fn solve_a(input: &str) -> u64 {
    let (time, distance) = parse_input_lines_a.parse(input).unwrap();

    solve_races(time, distance)
}

fn solve_races(time: Vec<u64>, distance: Vec<u64>) -> u64 {
    let mut result = 1;

    for (t, d) in time.into_iter().zip(distance.into_iter()) {
        #[allow(clippy::cast_precision_loss)]
        let t: f64 = t as f64;
        #[allow(clippy::cast_precision_loss)]
        let d: f64 = d as f64 + 0.1;

        let x1 = (t + (t * t - 4.0 * d).sqrt()) / 2.0;
        let x2 = (t - (t * t - 4.0 * d).sqrt()) / 2.0;

        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let tmp = (x1.floor() - x2.ceil() + 1.0) as u64;

        result *= tmp;
    }

    result
}

#[must_use]
pub fn solve_b(input: &str) -> u64 {
    let (time, distance) = parse_input_lines_b.parse(input).unwrap();

    solve_races(vec![time], vec![distance])
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "Time:      7  15   30
Distance:  9  40  200";
    #[test]
    fn example_a() {
        assert_eq!(solve_a(INPUT), 288);
    }

    #[test]
    fn example_b() {
        assert_eq!(solve_b(INPUT), 71503);
    }

    #[test]
    fn solution_a() {
        assert_eq!(solve_a(include_str!("input.txt")), 771_628);
    }

    #[test]
    fn solution_b() {
        assert_eq!(solve_b(include_str!("input.txt")), 27_363_861);
    }
}
