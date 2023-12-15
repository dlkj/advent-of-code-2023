#![warn(clippy::pedantic)]
#![warn(clippy::style)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]
#![warn(clippy::use_self)]

use std::{
    collections::{HashMap, HashSet, VecDeque},
    ops::RangeBounds,
    usize,
};

use itertools::Itertools;

fn grid_coords<'a>(lines: impl Iterator<Item = &'a str>, target: char) -> Vec<(usize, usize)> {
    lines
        .enumerate()
        .flat_map(|(y, l)| {
            l.chars()
                .enumerate()
                .filter_map(move |(x, c)| (c == target).then_some((x, y)))
        })
        .collect_vec()
}

#[must_use]
pub fn solve_a(input: &str) -> usize {
    let y_len = input.lines().count();
    let x_len = input.lines().next().unwrap().len();

    let round = grid_coords(input.lines(), 'O');
    let cube = grid_coords(input.lines(), '#');

    let round = tilt_north(&round, &cube, x_len);

    round.into_iter().map(|(_, y)| y_len - y).sum()
}

fn tilt_north(
    round: &[(usize, usize)],
    cube: &[(usize, usize)],
    x_len: usize,
) -> Vec<(usize, usize)> {
    // group by column

    let (round, cube) = to_columns(round, cube);

    (0..x_len)
        .flat_map(|x| {
            let empty_r: &mut HashSet<(usize, usize)> = &mut HashSet::new();
            let empty_c: &Vec<(usize, usize)> = &vec![];
            let mut round = round.get(&x).unwrap_or(empty_r).clone();
            let cube = cube.get(&x).unwrap_or(empty_c);

            if cube.is_empty() {
                roll_up(&mut round, .., x);
            } else {
                //do the first #
                let &(_, cy) = cube.first().unwrap();
                roll_up(&mut round, ..cy, x);

                //do every pair of #
                for (&(_, cy1), &(_, cy2)) in cube.iter().tuple_windows() {
                    roll_up(&mut round, (cy1 + 1)..cy2, x);
                }

                //do the last #
                let &(_, cy) = cube.last().unwrap();
                roll_up(&mut round, (cy + 1).., x);
            }
            round
        })
        .collect()
}

fn tilt_south(
    round: &[(usize, usize)],
    cube: &[(usize, usize)],
    x_len: usize,
    y_len: usize,
) -> Vec<(usize, usize)> {
    // group by column

    let (round, cube) = to_columns(round, cube);

    (0..x_len)
        .flat_map(|x| {
            let empty_r: &mut HashSet<(usize, usize)> = &mut HashSet::new();
            let empty_c: &Vec<(usize, usize)> = &vec![];
            let mut round = round.get(&x).unwrap_or(empty_r).clone();
            let cube = cube.get(&x).unwrap_or(empty_c);

            if cube.is_empty() {
                roll_down(&mut round, .., x, y_len);
            } else {
                //do the first #
                let &(_, cy) = cube.first().unwrap();
                roll_down(&mut round, ..cy, x, y_len);

                //do every pair of #
                for (&(_, cy1), &(_, cy2)) in cube.iter().tuple_windows() {
                    roll_down(&mut round, (cy1 + 1)..cy2, x, y_len);
                }

                //do the last #
                let &(_, cy) = cube.last().unwrap();
                roll_down(&mut round, (cy + 1).., x, y_len);
            }
            round
        })
        .collect()
}

fn tilt_west(
    round: &[(usize, usize)],
    cube: &[(usize, usize)],
    x_len: usize,
) -> Vec<(usize, usize)> {
    // group by column
    let (round, cube) = to_rows(round, cube);

    (0..x_len)
        .flat_map(|x| {
            let empty_r: &mut HashSet<(usize, usize)> = &mut HashSet::new();
            let empty_c: &Vec<(usize, usize)> = &vec![];
            let mut round = round.get(&x).unwrap_or(empty_r).clone();
            let cube = cube.get(&x).unwrap_or(empty_c);

            if cube.is_empty() {
                roll_up(&mut round, .., x);
            } else {
                //do the first #
                let &(_, cy) = cube.first().unwrap();
                roll_up(&mut round, ..cy, x);

                //do every pair of #
                for (&(_, cy1), &(_, cy2)) in cube.iter().tuple_windows() {
                    roll_up(&mut round, (cy1 + 1)..cy2, x);
                }

                //do the last #
                let &(_, cy) = cube.last().unwrap();
                roll_up(&mut round, (cy + 1).., x);
            }
            round
        })
        .map(|(y, x)| (x, y))
        .collect()
}

fn tilt_east(
    round: &[(usize, usize)],
    cube: &[(usize, usize)],
    x_len: usize,
    y_len: usize,
) -> Vec<(usize, usize)> {
    // group by column

    let (round, cube) = to_rows(round, cube);

    (0..x_len)
        .flat_map(|x| {
            let empty_r: &mut HashSet<(usize, usize)> = &mut HashSet::new();
            let empty_c: &Vec<(usize, usize)> = &vec![];
            let mut round = round.get(&x).unwrap_or(empty_r).clone();
            let cube = cube.get(&x).unwrap_or(empty_c);

            if cube.is_empty() {
                roll_down(&mut round, .., x, y_len);
            } else {
                //do the first #
                let &(_, cy) = cube.first().unwrap();
                roll_down(&mut round, ..cy, x, y_len);

                //do every pair of #
                for (&(_, cy1), &(_, cy2)) in cube.iter().tuple_windows() {
                    roll_down(&mut round, (cy1 + 1)..cy2, x, y_len);
                }

                //do the last #
                let &(_, cy) = cube.last().unwrap();
                roll_down(&mut round, (cy + 1).., x, y_len);
            }
            round
        })
        .map(|(y, x)| (x, y))
        .collect()
}

fn to_columns(
    round: &[(usize, usize)],
    cube: &[(usize, usize)],
) -> (
    HashMap<usize, HashSet<(usize, usize)>>,
    HashMap<usize, Vec<(usize, usize)>>,
) {
    let round: HashMap<usize, HashSet<_>> = round
        .iter()
        .copied()
        .sorted()
        .group_by(|&(x, _)| x)
        .into_iter()
        .map(|(x, g)| (x, g.collect()))
        .collect();

    let cube: HashMap<usize, Vec<_>> = cube
        .iter()
        .copied()
        .sorted()
        .group_by(|&(x, _)| x)
        .into_iter()
        .map(|(x, g)| (x, g.collect()))
        .collect();
    (round, cube)
}

fn to_rows(
    round: &[(usize, usize)],
    cube: &[(usize, usize)],
) -> (
    HashMap<usize, HashSet<(usize, usize)>>,
    HashMap<usize, Vec<(usize, usize)>>,
) {
    let round: HashMap<usize, HashSet<_>> = round
        .iter()
        .copied()
        .sorted_by_key(|&(x, y)| (y, x))
        .group_by(|&(_, y)| y)
        .into_iter()
        .map(|(y, g)| (y, g.map(|(x, y)| (y, x)).collect()))
        .collect();

    let cube: HashMap<usize, Vec<_>> = cube
        .iter()
        .copied()
        .sorted_by_key(|&(x, y)| (y, x))
        .group_by(|&(_, y)| y)
        .into_iter()
        .map(|(y, g)| (y, g.map(|(x, y)| (y, x)).collect()))
        .collect();
    (round, cube)
}

fn roll_up(round: &mut HashSet<(usize, usize)>, range: impl RangeBounds<usize>, x: usize) {
    // remove all the stones that are in the range and count them
    let removed_len = round.iter().filter(|(_, y)| range.contains(y)).count();
    round.retain(|(_, y)| !range.contains(y));

    // insert that many at the end or the range
    match range.start_bound() {
        std::ops::Bound::Included(&n) => round.extend((n..(n + removed_len)).map(|n| (x, n))),
        std::ops::Bound::Excluded(&n) => {
            round.extend(((n + 1)..=(n + removed_len)).map(|n| (x, n)));
        }
        std::ops::Bound::Unbounded => round.extend((0..removed_len).map(|n| (x, n))),
    }
}

fn roll_down(
    round: &mut HashSet<(usize, usize)>,
    range: impl RangeBounds<usize>,
    x: usize,
    len: usize,
) {
    // remove all the stones that are in the range and count them
    let removed_len = round.iter().filter(|(_, y)| range.contains(y)).count();
    round.retain(|(_, y)| !range.contains(y));

    // insert that many at the end or the range

    //stuff is going missing
    match range.end_bound() {
        std::ops::Bound::Included(&n) => round.extend(((n - removed_len - 1)..=n).map(|n| (x, n))),
        std::ops::Bound::Excluded(&n) => {
            round.extend(((n - removed_len)..n).map(|n| (x, n)));
        }
        std::ops::Bound::Unbounded => round.extend((len - removed_len..len).map(|n| (x, n))),
    }
}

#[must_use]
pub fn solve_b(input: &str) -> usize {
    let y_len = input.lines().count();
    let x_len = input.lines().next().unwrap().len();

    let mut round = grid_coords(input.lines(), 'O');
    let cube = grid_coords(input.lines(), '#');

    let mut score_history = VecDeque::new();

    for _ in 0..300 {
        let r_len = round.len();

        round = tilt_north(&round, &cube, x_len);
        assert_eq!(round.len(), r_len);

        let r_len = round.len();
        round = tilt_west(&round, &cube, y_len);
        assert_eq!(round.len(), r_len);

        let r_len = round.len();
        round = tilt_south(&round, &cube, x_len, y_len);
        assert_eq!(round.len(), r_len);

        let r_len = round.len();
        round = tilt_east(&round, &cube, y_len, x_len);
        assert_eq!(round.len(), r_len);

        let score = score(&round, y_len);

        score_history.push_back(score);
        while score_history.len() > 30 {
            score_history.pop_front();
        }
    }

    // detect cycles < 30

    let first: (usize, usize, usize, usize) = score_history.iter().copied().next_tuple().unwrap();

    let (mut cycle, _) = score_history
        .iter()
        .skip(4)
        .copied()
        .tuple_windows()
        .find_position(|x: &(_, _, _, _)| x == &first)
        .unwrap();

    cycle += 4;

    let end = 1_000_000_000 % cycle;

    let offset = (300 - 30) % cycle;

    if end > offset {
        score_history[end - offset - 1]
    } else {
        score_history[cycle + end - offset - 1]
    }
}

fn score(round: &[(usize, usize)], y_len: usize) -> usize {
    round.iter().map(|(_, y)| y_len - y).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";
    #[test]
    fn example_a() {
        assert_eq!(solve_a(INPUT), 136);
    }

    #[test]
    fn example_b() {
        assert_eq!(solve_b(INPUT), 64);
    }

    #[test]
    fn solution_a() {
        assert_eq!(solve_a(include_str!("input.txt")), 106_997);
    }

    #[test]
    fn solution_b() {
        assert_eq!(solve_b(include_str!("input.txt")), 99641);
    }
}
