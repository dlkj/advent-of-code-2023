#![warn(clippy::pedantic)]
#![warn(clippy::style)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]
#![warn(clippy::use_self)]

#[must_use]
pub fn solve_a(input: &str) -> u32 {
    let lines: Vec<_> = input
        .lines()
        .map(|l| {
            l.chars()
                .filter(|c| c.is_numeric())
                .map(|c| c.to_digit(10).unwrap())
                .collect()
        })
        .collect();

    sum(&lines)
}

#[must_use]
pub fn solve_b(input: &str) -> u32 {
    let lines: Vec<_> = input
        .lines()
        .map(|l| {
            let mut number = vec![];
            for i in 0..l.len() {
                if l[i..].starts_with("one") {
                    number.push(1);
                } else if l[i..].starts_with("two") {
                    number.push(2);
                } else if l[i..].starts_with("three") {
                    number.push(3);
                } else if l[i..].starts_with("four") {
                    number.push(4);
                } else if l[i..].starts_with("five") {
                    number.push(5);
                } else if l[i..].starts_with("six") {
                    number.push(6);
                } else if l[i..].starts_with("seven") {
                    number.push(7);
                } else if l[i..].starts_with("eight") {
                    number.push(8);
                } else if l[i..].starts_with("nine") {
                    number.push(9);
                } else if l[i..].chars().next().unwrap().is_numeric() {
                    number.push(l[i..].chars().next().unwrap().to_digit(10).unwrap());
                }
            }
            number
        })
        .collect();

    sum(&lines)
}

fn sum(lines: &[Vec<u32>]) -> u32 {
    lines
        .iter()
        .map(|l| {
            let first = *l.first().unwrap();
            let last = *l.last().unwrap();

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
    fn example_a() {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
";

        assert_eq!(solve_a(input), 142);
    }

    #[test]
    fn example_b() {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

        assert_eq!(solve_b(input), 281);
    }

    #[test]
    fn all_numeric() {
        let input = "371
";

        assert_eq!(solve_a(input), 31);
    }

    #[test]
    fn overlapping() {
        let input = "1tbbsmdhtwonedtt
";

        assert_eq!(solve_b(input), 11);
    }
}
