#![warn(clippy::pedantic)]
#![warn(clippy::style)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]
#![warn(clippy::use_self)]

use std::ops::Range;

use itertools::Itertools;
use winnow::{
    ascii::{dec_uint, line_ending},
    combinator::{preceded, repeat, separated, VerifyMap},
    token::take_until1,
    PResult, Parser,
};

fn parse_seeds_a(input: &mut &str) -> PResult<Vec<u64>> {
    preceded("seeds: ", separated(1.., dec_uint::<_, u64, _>, ' ')).parse_next(input)
}

fn parse_map(input: &mut &str) -> PResult<Vec<(u64, Range<u64>)>> {
    preceded(
        (line_ending, line_ending, take_until1("map:"), "map:"),
        repeat(
            1..,
            preceded(
                line_ending,
                (
                    dec_uint::<_, u64, _>,
                    ' ',
                    dec_uint::<_, u64, _>,
                    ' ',
                    dec_uint::<_, u64, _>,
                )
                    .map(|(a, _, b, _, c)| (a, b..(b + c))),
            ),
        ),
    )
    .parse_next(input)
}

#[must_use]
pub fn solve_a(mut input: &str) -> u64 {
    let parse_input = &mut input;
    let seeds = parse_seeds_a.parse_next(parse_input).unwrap();
    let location_maps: Vec<Vec<_>> = repeat(1.., parse_map).parse(parse_input).unwrap();

    seeds
        .into_iter()
        .map(|s| map_seed(s, &location_maps))
        .min()
        .unwrap()
}

#[must_use]
pub fn solve_b(mut input: &str) -> u64 {
    let parse_input = &mut input;
    let seeds = parse_seeds_a
        .parse_next(parse_input)
        .unwrap()
        .into_iter()
        .tuples()
        .map(|(a, b)| a..(a + b))
        .collect_vec();
    let location_maps: Vec<Vec<_>> = repeat(1.., parse_map).parse(parse_input).unwrap();

    map_seed_range(&seeds, &location_maps)
        .into_iter()
        .map(|r| r.start)
        .min()
        .unwrap()
}

fn map_seed(mut seed: u64, location_maps: &[Vec<(u64, Range<u64>)>]) -> u64 {
    for map_list in location_maps {
        seed = if let Some((dest, source)) =
            map_list.iter().find(|(_, source)| source.contains(&seed))
        {
            seed - source.start + dest
        } else {
            seed
        };
    }
    seed
}

fn map_seed_range(
    seeds: &[Range<u64>],
    location_maps: &[Vec<(u64, Range<u64>)>],
) -> Vec<Range<u64>> {
    let mut seeds: Vec<Range<u64>> = seeds.into_iter().cloned().collect();

    for map_list in location_maps {
        let mut processed = vec![];
        for (dest, source) in map_list {
            let mut missed = vec![];
            for s in &seeds {
                let (m, hit) = intersect(s, source);
                if let Some(hit) = hit {
                    processed.push(hit);
                    missed.extend(m);
                }
            }
            seeds.extend(missed);
        }
        seeds = processed;
    }
    seeds
}

fn intersect(s: &Range<u64>, source: &Range<u64>) -> (Vec<Range<u64>>, Option<Range<u64>>) {
    // no intersection
    if s.end <= source.start || s.start >= source.end {
        return (vec![s.clone()], None);
    }

    // bisect
    if s.start < source.start && s.end > source.end {
        return (
            vec![s.start..source.start, source.end..s.end],
            Some(source.clone()),
        );
    }

    // single overlap greater
    if s.start < source.start && s.end <= source.end {
        return (vec![s.start..source.start], Some(source.start..s.end));
    }
    // single overlap less
    if s.start >= source.start && s.end > source.end {
        return (vec![source.end..s.end], Some(s.start..source.end));
    }
    // contained
    return (vec![], Some(s.clone()));
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
    #[test]
    fn example_a() {
        assert_eq!(solve_a(INPUT), 35);
    }

    #[test]
    fn example_b() {
        assert_eq!(solve_b(INPUT), 46);
    }

    #[test]
    fn solution_a() {
        assert_eq!(solve_a(include_str!("input.txt")), 226_172_555);
    }

    #[test]
    fn solution_b() {
        assert_eq!(solve_b(include_str!("input.txt")), 0);
    }
}
