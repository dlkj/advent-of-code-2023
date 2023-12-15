#![warn(clippy::pedantic)]
#![warn(clippy::style)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]
#![warn(clippy::use_self)]

use itertools::Itertools;
use winnow::{
    ascii::{dec_uint, line_ending},
    combinator::{alt, eof, repeat, separated, separated_pair, terminated},
    error::{ContextError, ErrMode},
    token::{one_of, take_until1},
    PResult, Parser,
};

fn parse_lines<'a>(input: &mut &'a str) -> PResult<Vec<(&'a str, Vec<u32>)>> {
    separated(1.., parse_line, line_ending).parse_next(input)
}
fn parse_line<'a>(input: &mut &'a str) -> PResult<(&'a str, Vec<u32>)> {
    separated_pair(
        take_until1(" "),
        " ",
        separated(1.., dec_uint::<_, u32, _>, ","),
    )
    .parse_next(input)
}

fn group_parser<'a>(len: usize) -> impl Parser<&'a str, (), ErrMode<ContextError>> {
    terminated(
        repeat(len..=len, one_of(['#', '?'])),
        alt((one_of(['.', '?']), eof.value('~'))),
    )
}

fn solve(lines: Vec<(String, Vec<u32>)>) -> u64 {
    let mut result = 0;

    for (record, groups) in lines {
        let mut c = vec![(1, &record[..], &groups[..])];

        loop {
            c = c
                .into_iter()
                .flat_map(|(n, r, g)| possible_next(n, r, g))
                .collect();

            result += c
                .iter()
                .filter(|(_, record, g)| record.is_empty() && g.is_empty())
                .map(|&(x, _, _)| x)
                .sum::<usize>();

            c = c
                .into_iter()
                .filter(|(_, record, g)| !record.is_empty() || !g.is_empty())
                .sorted_by_key(|&(_, r, g)| (r, g))
                .group_by(|&(_, r, g)| (r, g))
                .into_iter()
                .map(|((r, g), group)| {
                    let n = group.into_iter().map(|(x, _, _)| x).sum();
                    (n, r, g)
                })
                .collect_vec();

            if c.is_empty() {
                break;
            }
        }
    }
    result.try_into().unwrap()
}

fn possible_next<'a>(
    count: usize,
    mut record: &'a str,
    groups: &'a [u32],
) -> Vec<(usize, &'a str, &'a [u32])> {
    // consume any '.'s
    while let Some('.') = record.chars().next() {
        record = &record[1..];
    }

    if !groups.is_empty() && record.is_empty() {
        return vec![];
    }

    if groups.is_empty() {
        if !record.contains('#') {
            return vec![(count, "", groups)];
        }
        return vec![];
    }

    match record.chars().next() {
        Some('?') => {
            if !groups.is_empty() {
                let g = groups.first().copied().unwrap().try_into().unwrap();
                if group_parser(g).parse_peek(record).is_ok() {
                    let offset = record.len().min(g + 1);
                    return vec![
                        (count, &record[offset..], &groups[1..]),
                        (count, &record[1..], groups),
                    ];
                }
            }
            return vec![(count, &record[1..], groups)];
        } // branch with '.' and check for possible '#'
        Some('#') => {
            if !groups.is_empty() {
                let g = groups.first().copied().unwrap().try_into().unwrap();
                if group_parser(g).parse_peek(record).is_ok() {
                    let offset = record.len().min(g + 1);
                    return vec![(count, &record[offset..], &groups[1..])];
                }
            }
        } // verify the next group can be written then advance
        None => {
            if groups.is_empty() {
                return vec![(count, record, groups)];
            }
        }
        Some(c) => panic!("Unexpected char {c}"),
    }
    vec![]
}

#[must_use]
pub fn solve_a(input: &str) -> u64 {
    let lines = parse_lines.parse(input).unwrap();
    let lines = lines.into_iter().map(|(r, g)| (r.to_owned(), g)).collect();
    solve(lines)
}

#[must_use]
pub fn solve_b(input: &str) -> u64 {
    let lines = parse_lines.parse(input).unwrap();
    let lines = lines
        .into_iter()
        .map(|(r, g)| {
            let mut long_r = String::new();
            long_r.push_str(r);
            long_r.push('?');
            long_r.push_str(r);
            long_r.push('?');
            long_r.push_str(r);
            long_r.push('?');
            long_r.push_str(r);
            long_r.push('?');
            long_r.push_str(r);
            let mut long_g = vec![];
            long_g.extend(&g);
            long_g.extend(&g);
            long_g.extend(&g);
            long_g.extend(&g);
            long_g.extend(&g);
            (long_r, long_g)
        })
        .collect_vec();

    solve(lines)
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

    #[test]
    fn eof() {
        assert_eq!(possible_next(1, "###", &[3]), vec![(1, "", &[][..])]);
    }

    #[test]
    fn example_a() {
        assert_eq!(solve_a(INPUT), 21);
    }

    #[test]
    fn example_b() {
        assert_eq!(solve_b(INPUT), 525_152);
    }

    #[test]
    fn solution_a() {
        assert_eq!(solve_a(include_str!("input.txt")), 6935);
    }

    #[test]
    fn solution_b() {
        assert_eq!(solve_b(include_str!("input.txt")), 3_920_437_278_260);
    }
}
