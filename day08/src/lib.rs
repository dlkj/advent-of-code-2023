#![warn(clippy::pedantic)]
#![warn(clippy::style)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]
#![warn(clippy::use_self)]

use std::collections::HashMap;

use num::Integer;
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

type Element<'a> = (&'a str, (&'a str, &'a str));

fn parse_input<'a>(input: &mut &'a str) -> PResult<(Vec<Step>, Vec<Element<'a>>)> {
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

    find_distance("AAA", &elements, &steps)
}

#[must_use]
pub fn solve_b(input: &str) -> u64 {
    let (steps, elements) = parse_input.parse(input).unwrap();
    let elements: HashMap<_, _> = elements.into_iter().collect();

    elements
        .keys()
        .filter(|e| e.ends_with('A'))
        .map(|p| u64::from(find_distance(p, &elements, &steps)))
        .reduce(|a, b| a.lcm(&b))
        .unwrap()
}

fn find_distance<'a>(
    mut current: &'a str,
    elements: &HashMap<&'a str, (&'a str, &'a str)>,
    steps: &[Step],
) -> u32 {
    let mut steps = steps.iter().cycle();
    let mut count = 0;

    while !current.ends_with('Z') {
        let (l, r) = elements.get(current).unwrap();
        match steps.next().unwrap() {
            Step::Left => current = l,
            Step::Right => current = r,
        }
        count += 1;
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT_A: &str = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

    const INPUT_B: &str = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

    #[test]
    fn example_a() {
        assert_eq!(solve_a(INPUT_A), 6);
    }

    #[test]
    fn example_b() {
        assert_eq!(solve_b(INPUT_B), 6);
    }

    #[test]
    fn solution_a() {
        assert_eq!(solve_a(include_str!("input.txt")), 17873);
    }

    #[test]
    fn solution_b() {
        assert_eq!(solve_b(include_str!("input.txt")), 15_746_133_679_061);
    }
}
