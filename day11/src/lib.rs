#![warn(clippy::pedantic)]
#![warn(clippy::style)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]
#![warn(clippy::use_self)]

use std::collections::HashSet;

use itertools::Itertools;

#[must_use]
pub fn solve_a(input: &str) -> u64 {
    solve(input, 1)
}

#[must_use]
pub fn solve_b(input: &str) -> u64 {
    solve(input, 999_999)
}

fn solve(input: &str, gap: usize) -> u64 {
    let mut coords = input
        .lines()
        .enumerate()
        .flat_map(|(y, l)| {
            l.chars()
                .enumerate()
                .filter_map(move |(x, c)| (c == '#').then_some((x, y)))
        })
        .collect_vec();

    let (filled_x, filled_y) = coords.iter().fold(
        (HashSet::new(), HashSet::new()),
        |(mut xs, mut ys), &(x, y)| {
            xs.insert(x);
            ys.insert(y);
            (xs, ys)
        },
    );

    // stretch in x
    for i in (0..*(filled_x.iter().max().unwrap())).rev() {
        if filled_x.contains(&i) {
            continue;
        }

        coords
            .iter_mut()
            .filter(|(x, _)| *x > i)
            .for_each(|(x, _)| *x += gap);
    }

    // stretch in y
    for i in (0..*(filled_y.iter().max().unwrap())).rev() {
        if filled_y.contains(&i) {
            continue;
        }

        coords
            .iter_mut()
            .filter(|(_, y)| *y > i)
            .for_each(|(_, y)| *y += gap);
    }

    // find distances
    coords
        .iter()
        .combinations(2)
        .map(|p| {
            let (&(x1, y1), &(x2, y2)) = p.into_iter().collect_tuple().unwrap();
            u64::try_from(x1.abs_diff(x2) + y1.abs_diff(y2)).unwrap()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
    #[test]
    fn example_a() {
        assert_eq!(solve_a(INPUT), 374);
    }

    #[test]
    fn example_b() {
        assert_eq!(solve(INPUT, 99), 8410);
    }

    #[test]
    fn solution_a() {
        assert_eq!(solve_a(include_str!("input.txt")), 10_422_930);
    }

    #[test]
    fn solution_b() {
        assert_eq!(solve_b(include_str!("input.txt")), 699_909_023_130);
    }
}
