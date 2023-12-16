#![warn(clippy::pedantic)]
#![warn(clippy::style)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]
#![warn(clippy::use_self)]

use std::collections::{HashMap, HashSet};

use itertools::Itertools;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, PartialOrd, Ord)]
enum Dir {
    N,
    E,
    S,
    W,
}

#[must_use]
pub fn solve_a(input: &str) -> usize {
    let input = input
        .lines()
        .map(|s| s.trim().chars().collect_vec())
        .collect_vec();

    solve(&input, ((0, 0), Dir::E))
}

fn solve(input: &[Vec<char>], start: ((usize, usize), Dir)) -> usize {
    let mut rays = vec![start];

    let y_len = input.len();
    let x_len = input.first().unwrap().len();

    let mut visited: HashMap<(usize, usize), HashSet<Dir>> = HashMap::new();

    while let Some((loc, dir)) = rays.pop() {
        // stop tracing ray if we have already seen a ray with this loc and dir
        let v = visited.entry(loc).or_default();
        if v.contains(&dir) {
            continue;
        }
        v.insert(dir);

        // where does the ray go next?
        let (x, y) = loc;

        // check we haven't gone off the grid
        if let Some(c) = input.get(y).and_then(|c| c.get(x)) {
            let next_rays = match c {
                '.' => empty(loc, dir),
                '|' => v_split(loc, dir),
                '-' => h_split(loc, dir),
                '/' => f_mirror(loc, dir),
                '\\' => b_mirror(loc, dir),
                _ => panic!("Unexpected char {c}"),
            };
            rays.extend(
                next_rays
                    .into_iter()
                    .filter(|&((x, y), _)| x < x_len && y < y_len),
            );
        }
    }

    visited.len()
}

fn north((x, y): (usize, usize)) -> Vec<((usize, usize), Dir)> {
    if y > 0 {
        vec![((x, y - 1), Dir::N)]
    } else {
        vec![]
    }
}
fn south((x, y): (usize, usize)) -> Vec<((usize, usize), Dir)> {
    vec![((x, y + 1), Dir::S)]
}

fn east((x, y): (usize, usize)) -> Vec<((usize, usize), Dir)> {
    vec![((x + 1, y), Dir::E)]
}

fn west((x, y): (usize, usize)) -> Vec<((usize, usize), Dir)> {
    if x > 0 {
        vec![((x - 1, y), Dir::W)]
    } else {
        vec![]
    }
}

fn empty(loc: (usize, usize), dir: Dir) -> Vec<((usize, usize), Dir)> {
    match dir {
        Dir::N => north(loc),
        Dir::E => east(loc),
        Dir::S => south(loc),
        Dir::W => west(loc),
    }
}

fn v_split(loc: (usize, usize), dir: Dir) -> Vec<((usize, usize), Dir)> {
    match dir {
        Dir::N => north(loc),
        Dir::E | Dir::W => north(loc).into_iter().chain(south(loc)).collect(),
        Dir::S => south(loc),
    }
}

fn h_split(loc: (usize, usize), dir: Dir) -> Vec<((usize, usize), Dir)> {
    match dir {
        Dir::N | Dir::S => east(loc).into_iter().chain(west(loc)).collect(),
        Dir::E => east(loc),
        Dir::W => west(loc),
    }
}

fn f_mirror(loc: (usize, usize), dir: Dir) -> Vec<((usize, usize), Dir)> {
    match dir {
        Dir::N => east(loc),
        Dir::E => north(loc),
        Dir::S => west(loc),
        Dir::W => south(loc),
    }
}
fn b_mirror(loc: (usize, usize), dir: Dir) -> Vec<((usize, usize), Dir)> {
    match dir {
        Dir::N => west(loc),
        Dir::E => south(loc),
        Dir::S => east(loc),
        Dir::W => north(loc),
    }
}

#[must_use]
pub fn solve_b(input: &str) -> usize {
    let input = input
        .lines()
        .map(|s| s.trim().chars().collect_vec())
        .collect_vec();

    let y_len = input.len();
    let x_len = input.first().unwrap().len();

    let e = (0..y_len)
        .map(|y| solve(&input, ((0, y), Dir::E)))
        .max()
        .unwrap();
    let w = (0..y_len)
        .map(|y| solve(&input, ((x_len - 1, y), Dir::W)))
        .max()
        .unwrap();
    let s = (0..x_len)
        .map(|x| solve(&input, ((x, 0), Dir::S)))
        .max()
        .unwrap();
    let n = (0..x_len)
        .map(|x| solve(&input, ((x, y_len - 1), Dir::N)))
        .max()
        .unwrap();

    e.max(w).max(s).max(n)
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = ".|...\\....
|.-.\\.....
.....|-...
........|.
..........
.........\\
..../.\\\\..
.-.-/..|..
.|....-|.\\
..//.|....";
    #[test]
    fn example_a() {
        assert_eq!(solve_a(INPUT), 46);
    }

    #[test]
    fn example_b() {
        assert_eq!(solve_b(INPUT), 51);
    }

    #[test]
    fn solution_a() {
        assert_eq!(solve_a(include_str!("input.txt")), 7496);
    }

    #[test]
    fn solution_b() {
        assert_eq!(solve_b(include_str!("input.txt")), 7932);
    }
}
