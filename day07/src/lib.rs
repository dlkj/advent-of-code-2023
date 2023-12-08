#![warn(clippy::pedantic)]
#![warn(clippy::style)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]
#![warn(clippy::use_self)]

use itertools::Itertools;
use winnow::ascii::dec_uint;
use winnow::ascii::line_ending;
use winnow::combinator::separated;
use winnow::combinator::separated_pair;
use winnow::stream::AsChar;
use winnow::token::take_while;
use winnow::PResult;
use winnow::Parser;

fn parse_hands(input: &mut &str) -> PResult<Vec<([Card; 5], u32)>> {
    separated(
        1..,
        separated_pair(
            take_while(5..=5, char::is_alphanum).map(|s: &str| {
                s.chars()
                    .map(|c| c.try_into().unwrap())
                    .collect_vec()
                    .try_into()
                    .unwrap()
            }),
            ' ',
            dec_uint,
        ),
        line_ending,
    )
    .parse_next(input)
}

#[derive(Debug)]
struct TryFromCardError;

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Debug)]
enum Card {
    C2 = 2,
    C3 = 3,
    C4 = 4,
    C5 = 5,
    C6 = 6,
    C7 = 7,
    C8 = 8,
    C9 = 9,
    CT = 10,
    CJ = 11,
    CQ = 12,
    CK = 13,
    CA = 14,
}

impl TryFrom<char> for Card {
    type Error = TryFromCardError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '2' => Ok(Self::C2),
            '3' => Ok(Self::C3),
            '4' => Ok(Self::C4),
            '5' => Ok(Self::C5),
            '6' => Ok(Self::C6),
            '7' => Ok(Self::C7),
            '8' => Ok(Self::C8),
            '9' => Ok(Self::C9),
            'T' => Ok(Self::CT),
            'J' => Ok(Self::CJ),
            'Q' => Ok(Self::CQ),
            'K' => Ok(Self::CK),
            'A' => Ok(Self::CA),
            _ => Err(TryFromCardError {}),
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Debug)]
#[repr(u32)]
enum HandType {
    High,
    OnePair,
    TwoPair,
    Three,
    FullHouse,
    Four,
    Five,
}

#[must_use]
pub fn solve_a(input: &str) -> u32 {
    let hands = parse_hands.parse(input).unwrap();

    let types = hands
        .iter()
        .map(|(cards, points)| (hand_type(cards), cards, points))
        .collect_vec();

    types
        .iter()
        .sorted_by(|(h0, cards0, _), (h1, cards1, _)| match h0.cmp(h1) {
            std::cmp::Ordering::Less => std::cmp::Ordering::Less,
            std::cmp::Ordering::Greater => std::cmp::Ordering::Greater,
            std::cmp::Ordering::Equal => cards0.cmp(cards1),
        })
        .enumerate()
        .fold(0, |score, (i, (_, _, &points))| {
            score + ((i as u32 + 1) * points)
        })
}

fn hand_type(cards: &[Card; 5]) -> HandType {
    let unique = cards.iter().unique().count();
    match unique {
        1 => HandType::Five,
        2 => {
            if let Some(1) = frequencies(cards).next() {
                HandType::Four
            } else {
                HandType::FullHouse
            }
        } // aaaab Four, aaabb three
        3 => {
            if let Some(3) = frequencies(cards).nth(2) {
                HandType::Three
            } else {
                HandType::TwoPair
            }
        } // aaabc three, aabbc two pair
        4 => HandType::OnePair, // aabcd one pair
        5 => HandType::High,    // high card
        _ => unreachable!(),
    }
}

fn frequencies(cards: &[Card]) -> impl Iterator<Item = usize> {
    cards
        .iter()
        .sorted()
        .copied()
        .group_by(|&x| x)
        .into_iter()
        .map(|(_, g)| g.count())
        .sorted()
}

#[must_use]
pub fn solve_b(input: &str) -> u32 {
    input.len().try_into().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
    #[test]
    fn example_a() {
        assert_eq!(solve_a(INPUT), 6440);
    }

    #[test]
    fn example_b() {
        assert_eq!(solve_b(INPUT), 0);
    }

    #[test]
    fn solution_a() {
        let r = solve_a(include_str!("input.txt"));
        assert_eq!(r, 251_806_792);
    }

    #[test]
    fn solution_b() {
        assert_eq!(solve_b(include_str!("input.txt")), 0);
    }

    fn to_cards(input: &str) -> [Card; 5] {
        input
            .chars()
            .map(|c| Card::try_from(c).unwrap())
            .collect_vec()
            .try_into()
            .unwrap()
    }

    #[test]
    fn five() {
        assert_eq!(hand_type(&to_cards("55555")), HandType::Five);
    }

    #[test]
    fn four() {
        assert_eq!(hand_type(&to_cards("55A55")), HandType::Four);
        assert_eq!(hand_type(&to_cards("A5555")), HandType::Four);
        assert_eq!(hand_type(&to_cards("5555A")), HandType::Four);
    }

    #[test]
    fn three() {
        assert_eq!(hand_type(&to_cards("T5A55")), HandType::Three);
        assert_eq!(hand_type(&to_cards("A555T")), HandType::Three);
        assert_eq!(hand_type(&to_cards("555TA")), HandType::Three);
    }

    #[test]
    fn one_pair() {
        assert_eq!(hand_type(&to_cards("TQA55")), HandType::OnePair);
        assert_eq!(hand_type(&to_cards("A5Q5T")), HandType::OnePair);
        assert_eq!(hand_type(&to_cards("Q55TA")), HandType::OnePair);
    }

    #[test]
    fn two_pair() {
        assert_eq!(hand_type(&to_cards("TTA55")), HandType::TwoPair);
        assert_eq!(hand_type(&to_cards("T5Q5T")), HandType::TwoPair);
        assert_eq!(hand_type(&to_cards("T55TA")), HandType::TwoPair);
    }

    #[test]
    fn full_house() {
        assert_eq!(hand_type(&to_cards("23232")), HandType::FullHouse);
        assert_eq!(hand_type(&to_cards("23332")), HandType::FullHouse);
        assert_eq!(hand_type(&to_cards("Q2Q2Q")), HandType::FullHouse);
    }
}
