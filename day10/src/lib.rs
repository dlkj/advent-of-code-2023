#![warn(clippy::pedantic)]
#![warn(clippy::style)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]
#![warn(clippy::use_self)]

use std::collections::HashSet;

use itertools::Itertools;

#[must_use]
pub fn solve_a(input: &str) -> u32 {
    let grid = &mut parse_input(input);

    u32::try_from(loop_coords(grid).len()).unwrap() / 2
}

#[must_use]
pub fn solve_b(input: &str) -> u32 {
    let grid = &mut parse_input(input);
    let coords: HashSet<_> = loop_coords(grid).into_iter().collect();

    let mut inside = false;
    let mut i_count = 0;
    let mut last_corner = None;

    for (y, x, c) in grid
        .iter()
        .enumerate()
        .flat_map(|(y, l)| l.iter().enumerate().map(move |(x, c)| (y, x, c)))
    {
        if coords.contains(&(y, x)) {
            match c {
                '|' => {
                    inside = !inside;
                }
                'L' | 'F' => {
                    assert_eq!(last_corner, None);
                    last_corner = Some(*c);
                }
                'J' | '7' => match (last_corner, c) {
                    (Some('L'), '7') | (Some('F'), 'J') => {
                        inside = !inside;
                        last_corner = None;
                    }
                    (Some('L'), 'J') | (Some('F'), '7') => {
                        last_corner = None;
                    }
                    _ => panic!(),
                },
                '-' => {}
                _ => panic!("Unexpected char {c}"),
            }
        } else if inside {
            i_count += 1;
        }
    }

    i_count
}

fn loop_coords(grid: &mut [Vec<char>]) -> Vec<(usize, usize)> {
    // find start
    let (mut y, mut x) = find(grid, 'S').expect("start 'S' not found");
    let start = (y, x);

    // replace 'S' with correct symbol
    let c = match (
        if y > 0 {
            grid.get(y - 1).and_then(|l| l.get(x))
        } else {
            None
        },
        grid.get(y + 1).and_then(|l| l.get(x)),
        if x > 0 {
            grid.get(y).and_then(|l| l.get(x - 1))
        } else {
            None
        },
        grid.get(y).and_then(|l| l.get(x + 1)),
    ) {
        (Some('7' | 'F' | '|'), Some('L' | 'J' | '|'), _, _) => '|',
        (_, _, Some('F' | 'L' | '-'), Some('7' | 'J' | '-')) => '-',
        (Some('7' | 'F' | '|'), _, _, Some('7' | 'J' | '-')) => 'L',
        (Some('7' | 'F' | '|'), _, Some('F' | 'L' | '-'), _) => 'J',
        (_, Some('L' | 'J' | '|'), Some('F' | 'L' | '-'), _) => '7',
        (_, Some('L' | 'J' | '|'), _, Some('7' | 'J' | '-')) => 'F',
        _ => panic!("Start is not connected to valid pipes"),
    };

    grid[y][x] = c;

    // follow the path round
    let mut coord = vec![(y, x)];
    let mut previous = (y, x);

    (y, x) = *find_next_steps(grid, y, x).first().unwrap();
    coord.push((y, x));

    while start != (y, x) {
        let next = find_next_steps(grid, y, x)
            .into_iter()
            .find(|&n| n != previous)
            .expect("No next step found");

        previous = (y, x);
        (y, x) = next;
        coord.push((y, x));
    }

    coord
}

fn find_next_steps(grid: &[Vec<char>], y: usize, x: usize) -> [(usize, usize); 2] {
    match grid[y].get(x) {
        Some('|') => [(y - 1, x), (y + 1, x)],
        Some('-') => [(y, x - 1), (y, x + 1)],
        Some('L') => [(y - 1, x), (y, x + 1)],
        Some('J') => [(y - 1, x), (y, x - 1)],
        Some('7') => [(y, x - 1), (y + 1, x)],
        Some('F') => [(y, x + 1), (y + 1, x)],
        _ => panic!("invalid char {}", grid[y][x]),
    }
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect_vec()).collect_vec()
}

fn find(grid: &[Vec<char>], target: char) -> Option<(usize, usize)> {
    grid.iter().enumerate().find_map(|(y, l)| {
        l.iter()
            .enumerate()
            .find_map(|(x, &c)| (c == target).then_some(x))
            .map(|x| (y, x))
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT_A: &str = "7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ";

    const INPUT_B: &str = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";
    #[test]
    fn example_a() {
        assert_eq!(solve_a(INPUT_A), 8);
    }

    #[test]
    fn example_b() {
        assert_eq!(solve_b(INPUT_B), 4);
    }

    #[test]
    fn solution_a() {
        assert_eq!(solve_a(include_str!("input.txt")), 6640);
    }

    #[test]
    fn solution_b() {
        assert_eq!(solve_b(include_str!("input.txt")), 411);
    }
}
