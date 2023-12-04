#![warn(clippy::pedantic)]
#![warn(clippy::style)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]
#![warn(clippy::use_self)]

#[must_use]
pub fn solve_a(input: &str) -> u32 {
    input.len().try_into().unwrap()
}

#[must_use]
pub fn solve_b(input: &str) -> u32 {
    input.len().try_into().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "";
    #[test]
    fn example_a() {
        assert_eq!(solve_a(INPUT), 0);
    }

    #[test]
    fn example_b() {
        assert_eq!(solve_b(INPUT), 0);
    }

    #[test]
    fn solution_a() {
        assert_eq!(solve_a(include_str!("input.txt")), 0);
    }

    #[test]
    fn solution_b() {
        assert_eq!(solve_b(include_str!("input.txt")), 0);
    }
}
