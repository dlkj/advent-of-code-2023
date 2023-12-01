#![warn(clippy::pedantic)]
#![warn(clippy::style)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]
#![warn(clippy::use_self)]

#[must_use]
pub fn solve_a(input: &str) -> u32 {
    todo!()
}

#[must_use]
pub fn solve_b(input: &str) -> u32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_a() {
        let input = "";

        assert_eq!(solve_a(input), 0);
    }

    #[test]
    fn example_b() {
        let input = "";

        assert_eq!(solve_b(input), 0);
    }

    #[test]
    fn a() {
        assert_eq!(solve_a(include_str!("input.txt")), 0);
    }

    #[test]
    fn b() {
        assert_eq!(solve_b(include_str!("input.txt")), 0);
    }
}
