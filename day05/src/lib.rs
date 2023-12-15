#![warn(clippy::pedantic)]
#![warn(clippy::style)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]
#![warn(clippy::use_self)]

use std::ops::Range;

use itertools::Itertools;
use winnow::{
    ascii::{dec_uint, line_ending},
    combinator::{preceded, repeat, separated},
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

    println!("{seeds:?}");

    map_seed_range(seeds, &location_maps)
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
    mut seeds: Vec<Range<u64>>,
    location_maps: &[Vec<(u64, Range<u64>)>],
) -> Vec<Range<u64>> {
    for map_list in location_maps {
        let mut processed = vec![];

        while let Some(seed) = seeds.pop() {
            let mut any_hit = false;
            for (dest, source) in map_list {
                let (misses, hit) = intersect(&seed, source, *dest);

                if let Some(hit) = hit {
                    processed.push(hit);
                    seeds.extend(misses);
                    any_hit = true;
                    break;
                }
            }
            if !any_hit {
                processed.push(seed);
            }
        }

        seeds = processed;
    }

    seeds
}

fn intersect(
    seed: &Range<u64>,
    source: &Range<u64>,
    dest: u64,
) -> (Vec<Range<u64>>, Option<Range<u64>>) {
    // no intersection
    if seed.end <= source.start || seed.start >= source.end {
        return (vec![seed.clone()], None);
    }

    // bisect
    if seed.start < source.start && seed.end > source.end {
        return (
            vec![seed.start..source.start, source.end..seed.end],
            Some((dest)..(dest + source.end - source.start)),
        );
    }

    // single overlap greater
    if seed.start < source.start && seed.end <= source.end {
        return (
            vec![seed.start..source.start],
            Some((dest)..(seed.end - source.start + dest)),
        );
    }
    // single overlap less
    if seed.start >= source.start && seed.end > source.end {
        return (
            vec![source.end..seed.end],
            Some((seed.start - source.start + dest)..(source.end - source.start + dest)),
        );
    }

    // contained
    (
        vec![],
        Some((seed.start - source.start + dest)..(seed.end - source.start + dest)),
    )
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
        assert_eq!(solve_b(include_str!("input.txt")), 47_909_639);
    }

    #[test]
    fn intersect_miss() {
        assert_eq!(intersect(&(0..10), &(10..20), 100), (vec![0..10], None));
    }

    #[test]
    fn intersect_match() {
        assert_eq!(
            intersect(&(10..20), &(10..20), 100),
            (vec![], Some(100..110))
        );
    }

    #[test]
    fn intersect_bisect() {
        assert_eq!(
            intersect(&(0..30), &(10..20), 100),
            (vec![0..10, 20..30], Some(100..110))
        );
    }

    #[test]
    fn intersect_inside() {
        assert_eq!(
            intersect(&(10..20), &(0..30), 100),
            (vec![], Some(110..120))
        );
    }

    #[test]
    fn intersect_overlap_greater() {
        assert_eq!(
            intersect(&(0..20), &(10..30), 100),
            (vec![0..10], Some(100..110))
        );
    }

    #[test]
    fn intersect_overlap_less() {
        assert_eq!(
            intersect(&(10..30), &(0..20), 100),
            (vec![20..30], Some(110..120))
        );
    }
}
