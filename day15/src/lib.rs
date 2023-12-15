#![warn(clippy::pedantic)]
#![warn(clippy::style)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]
#![warn(clippy::use_self)]

use winnow::{
    ascii::{alpha1, dec_uint},
    combinator::{alt, preceded, separated},
    PResult, Parser,
};

#[must_use]
pub fn solve_a(input: &str) -> u32 {
    let steps = input.split(',');
    steps.map(hash).sum()
}

fn hash(input: &str) -> u32 {
    input
        .chars()
        .fold(0u32, |a, c| ((a + u32::try_from(c).unwrap()) * 17) % 256)
}

#[derive(Clone, Copy, Debug)]
enum Action {
    Set(usize),
    Remove,
}

fn parse_step<'a>(input: &mut &'a str) -> PResult<(&'a str, Action)> {
    (
        alpha1,
        alt((
            ("-").value(Action::Remove),
            preceded('=', dec_uint::<_, u32, _>).map(|n| Action::Set(n.try_into().unwrap())),
        )),
    )
        .parse_next(input)
}

fn parse_steps<'a>(input: &mut &'a str) -> PResult<Vec<(&'a str, Action)>> {
    separated(1.., parse_step, ',').parse_next(input)
}

#[must_use]
pub fn solve_b(input: &str) -> usize {
    let steps = parse_steps.parse(input).unwrap();

    let mut boxes: Vec<Vec<(&str, usize)>> = vec![vec![]; 256];

    for (label, act) in steps {
        let hash = usize::try_from(hash(label)).unwrap();

        match act {
            Action::Set(i) => {
                if let Some(item) = boxes[hash].iter_mut().find(|(l, _)| l == &label) {
                    *item = (label, i);
                } else {
                    boxes[hash].push((label, i));
                }
            }
            Action::Remove => boxes[hash].retain(|&(l, _)| l != label),
        }
    }

    boxes
        .into_iter()
        .enumerate()
        .flat_map(|(i, b)| {
            b.into_iter()
                .enumerate()
                .map(move |(x, (_, n))| (i + 1) * (x + 1) * n)
        })
        .sum::<usize>()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
    #[test]
    fn example_a() {
        assert_eq!(solve_a(INPUT), 1320);
    }

    #[test]
    fn example_b() {
        assert_eq!(solve_b(INPUT), 145);
    }

    #[test]
    fn solution_a() {
        assert_eq!(solve_a(include_str!("input.txt")), 513_214);
    }

    #[test]
    fn solution_b() {
        assert_eq!(solve_b(include_str!("input.txt")), 258_826);
    }
}
