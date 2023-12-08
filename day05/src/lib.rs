#![warn(clippy::pedantic)]
#![warn(clippy::style)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]
#![warn(clippy::use_self)]

use std::ops::Range;

use winnow::{
    ascii::{dec_uint, line_ending},
    combinator::{preceded, repeat, separated},
    token::take_until1,
    PResult, Parser,
};

fn parse_seeds(input: &mut &str) -> PResult<Vec<u64>> {
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
    let seeds = parse_seeds.parse_next(parse_input).unwrap();
    let location_maps: Vec<Vec<_>> = repeat(1.., parse_map).parse(parse_input).unwrap();

    seeds
        .into_iter()
        .map(|s| map_seed(s, &location_maps))
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

#[must_use]
pub fn solve_b(input: &str) -> u64 {
    input.len().try_into().unwrap()
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
