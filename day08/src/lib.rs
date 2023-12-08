#![warn(clippy::pedantic)]
#![warn(clippy::style)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]
#![warn(clippy::use_self)]

use std::collections::HashMap;

use winnow::ascii::line_ending;
use winnow::combinator::delimited;
use winnow::combinator::preceded;
use winnow::combinator::repeat;
use winnow::combinator::separated_pair;
use winnow::token::one_of;
use winnow::token::take_until1;
use winnow::PResult;
use winnow::Parser;

#[derive(Debug, Clone, Copy)]
enum Step {
    Left,
    Right,
}

fn parse_input<'a>(
    input: &mut &'a str,
) -> PResult<(Vec<Step>, Vec<(&'a str, (&'a str, &'a str))>)> {
    separated_pair(
        repeat(
            1..,
            one_of(['L', 'R']).map(|c| if c == 'L' { Step::Left } else { Step::Right }),
        ),
        line_ending,
        repeat(
            1..,
            separated_pair(
                preceded(line_ending, take_until1(" ")),
                " = ",
                delimited(
                    '(',
                    separated_pair(take_until1(","), ", ", take_until1(")")),
                    ')',
                ),
            ),
        ),
    )
    .parse_next(input)
}

#[must_use]
pub fn solve_a(input: &str) -> u32 {
    let (steps, elements) = parse_input.parse(input).unwrap();

    let elements: HashMap<_, _> = elements.into_iter().collect();

    let mut steps = steps.into_iter().cycle();

    let mut current = "AAA";
    let mut count = 0;

    while current != "ZZZ" {
        let (l, r) = elements.get(current).unwrap();
        match steps.next().unwrap() {
            Step::Left => current = l,
            Step::Right => current = r,
        }
        count += 1;
    }

    count
}

#[must_use]
pub fn solve_b(input: &str) -> u32 {
    input.len().try_into().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
    #[test]
    fn example_a() {
        assert_eq!(solve_a(INPUT), 6);
    }

    #[test]
    fn example_b() {
        assert_eq!(solve_b(INPUT), 0);
    }

    #[test]
    fn solution_a() {
        assert_eq!(solve_a(include_str!("input.txt")), 17873);
    }

    #[test]
    fn solution_b() {
        assert_eq!(solve_b(include_str!("input.txt")), 0);
    }
}
