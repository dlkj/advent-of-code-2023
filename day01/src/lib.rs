use nom::{
    character::complete::{alpha0, line_ending, satisfy},
    combinator::opt,
    error::Error,
    multi::many1,
    sequence::{delimited, terminated},
};

pub fn solve(input: &str) -> u32 {
    let line = terminated(
        many1(delimited(alpha0, satisfy(|c| c.is_numeric()), alpha0)),
        opt(line_ending),
    );

    let (_, lines) = many1::<_, _, Error<_>, _>(line)(input).unwrap();

    lines
        .iter()
        .map(|l| {
            let first = l.first().unwrap().to_digit(10).unwrap();
            let last = l.last().unwrap().to_digit(10).unwrap();

            assert!(first < 10);
            assert!(last < 10);

            first * 10 + last
        })
        .sum::<u32>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
";

        assert_eq!(solve(input), 142);
    }

    #[test]
    fn all_numeric() {
        let input = "371
";

        assert_eq!(solve(input), 31);
    }
}
