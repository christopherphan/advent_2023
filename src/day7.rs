/* Solution to 2023 Advent of Code, Day 7
 *
 * Christopher Phan
 */

use std::cmp::Ordering;
use std::collections::HashMap;
use std::error::Error;
use std::str::FromStr;

use crate::common;
use crate::common::AdventError;

const DAY: usize = 7;
const PART_1_IMPL: bool = true;
const PART_2_IMPL: bool = true;

pub fn run() {
    let input = common::get_day(DAY).unwrap();
    if PART_1_IMPL {
        println!(
            "{}",
            common::soln_output(DAY, 1, part_1(input.clone()).unwrap())
        );
    }
    if PART_2_IMPL {
        println!("{}", common::soln_output(DAY, 2, part_2(input).unwrap()));
    }
    if !(PART_1_IMPL || PART_2_IMPL) {
        println!("Not implemented yet");
    }
}

pub fn part_1(input: Vec<String>) -> Result<u64, Box<dyn Error>> {
    Ok(value(parse_input(input)))
}

pub fn part_2(input: Vec<String>) -> Result<u64, Box<dyn Error>> {
    Ok(value2(parse_input_2(input)))
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
enum CamelCard {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl CamelCard {
    fn _comparison_value(&self) -> usize {
        match self {
            Self::Two => 0,
            Self::Three => 1,
            Self::Four => 2,
            Self::Five => 3,
            Self::Six => 4,
            Self::Seven => 5,
            Self::Eight => 6,
            Self::Nine => 7,
            Self::Ten => 8,
            Self::Jack => 9,
            Self::Queen => 10,
            Self::King => 11,
            Self::Ace => 12,
        }
    }
}

impl TryFrom<char> for CamelCard {
    type Error = AdventError;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c.to_ascii_uppercase() {
            '2' => Ok(Self::Two),
            '3' => Ok(Self::Three),
            '4' => Ok(Self::Four),
            '5' => Ok(Self::Five),
            '6' => Ok(Self::Six),
            '7' => Ok(Self::Seven),
            '8' => Ok(Self::Eight),
            '9' => Ok(Self::Nine),
            'T' => Ok(Self::Ten),
            'J' => Ok(Self::Jack),
            'Q' => Ok(Self::Queen),
            'K' => Ok(Self::King),
            'A' => Ok(Self::Ace),
            k => Err(AdventError(format!("invalid camel card character {}", k))),
        }
    }
}

impl PartialOrd for CamelCard {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for CamelCard {
    fn cmp(&self, other: &Self) -> Ordering {
        self._comparison_value().cmp(&other._comparison_value())
    }
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
enum CamelCard2 {
    Joker,
    NotJoker(CamelCard),
}

impl CamelCard2 {
    fn _comparison_value(&self) -> usize {
        match self {
            Self::Joker => 0,
            Self::NotJoker(k) => k._comparison_value() + 1,
        }
    }
}

impl From<CamelCard> for CamelCard2 {
    fn from(c: CamelCard) -> Self {
        match c {
            CamelCard::Jack => CamelCard2::Joker,
            k => CamelCard2::NotJoker(k),
        }
    }
}

impl TryFrom<char> for CamelCard2 {
    type Error = AdventError;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        Ok(CamelCard::try_from(c)?.into())
    }
}

impl PartialOrd for CamelCard2 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for CamelCard2 {
    fn cmp(&self, other: &Self) -> Ordering {
        self._comparison_value().cmp(&other._comparison_value())
    }
}

impl TryFrom<CamelCard2> for CamelCard {
    type Error = AdventError;

    fn try_from(c: CamelCard2) -> Result<Self, Self::Error> {
        match c {
            CamelCard2::Joker => Err(AdventError::new("can't convert jokers back")),
            CamelCard2::NotJoker(k) => Ok(k),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Hand([CamelCard; 5]);

impl FromStr for Hand {
    type Err = AdventError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        if s.len() != 5 {
            Err(AdventError::new("hands consist of exactly five cards"))
        } else {
            let mut out_val: [CamelCard; 5] = [CamelCard::Ace; 5];
            for (idx, val) in s.chars().map(|c| CamelCard::try_from(c)).enumerate() {
                match val {
                    Ok(card) => {
                        out_val[idx] = card;
                    }
                    Err(k) => {
                        return Err(k);
                    }
                }
            }
            Ok(Self(out_val))
        }
    }
}

impl Hand {
    fn full_freq(&self) -> [usize; 13] {
        let mut out_val: [usize; 13] = [0; 13];
        for c in self.0.iter() {
            out_val[c._comparison_value()] += 1;
        }
        out_val
    }

    fn hand_shape(&self) -> Vec<usize> {
        let mut out_vec: Vec<usize> = self
            .full_freq()
            .iter()
            .map(|k| *k)
            .filter(|k| *k != 0)
            .collect();
        out_vec.sort();
        out_vec
    }

    fn hand_type(&self) -> HandType {
        match self.hand_shape().as_slice() {
            &[5] => HandType::FiveOfAKind,
            &[1, 4] => HandType::FourOfAKind,
            &[2, 3] => HandType::FullHouse,
            &[1, 1, 3] => HandType::ThreeOfAKind,
            &[1, 2, 2] => HandType::TwoPair,
            &[1, 1, 1, 2] => HandType::OnePair,
            &[1, 1, 1, 1, 1] => HandType::HighCard,
            _ => {
                panic!("This shouldn't happen!");
            }
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.hand_type().cmp(&other.hand_type()) {
            Ordering::Equal => {
                for (self_card, other_card) in self.0.iter().zip(other.0.iter()) {
                    match self_card.cmp(&other_card) {
                        Ordering::Equal => { /* pass */ }
                        k => {
                            return k;
                        }
                    }
                }
                Ordering::Equal
            }
            k => k,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl HandType {
    fn _comparison_value(&self) -> usize {
        match self {
            Self::FiveOfAKind => 6,
            Self::FourOfAKind => 5,
            Self::FullHouse => 4,
            Self::ThreeOfAKind => 3,
            Self::TwoPair => 2,
            Self::OnePair => 1,
            Self::HighCard => 0,
        }
    }
}

impl Ord for HandType {
    fn cmp(&self, other: &Self) -> Ordering {
        self._comparison_value().cmp(&other._comparison_value())
    }
}

impl PartialOrd for HandType {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn parse_input(input: Vec<String>) -> Vec<(Hand, u64)> {
    let mut out_vec: Vec<(Hand, u64)> = vec![];
    for line in input {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 2 {
            let hand = Hand::from_str(parts[0]);
            let bid = u64::from_str(&parts[1]);
            if hand.is_ok() && bid.is_ok() {
                out_vec.push((hand.unwrap(), bid.unwrap()));
            }
        }
    }
    out_vec
}

fn parse_input_2(input: Vec<String>) -> Vec<(Hand2, u64)> {
    parse_input(input)
        .iter()
        .map(|(h, v)| (Hand2::from(*h), *v))
        .collect()
}

fn value(hands: Vec<(Hand, u64)>) -> u64 {
    let mut s_hands = hands.clone();
    s_hands.sort_by_key(|k| k.0);
    s_hands
        .iter()
        .enumerate()
        .map(|(idx, (_, bid))| bid * (idx as u64 + 1))
        .sum()
}

fn value2(hands: Vec<(Hand2, u64)>) -> u64 {
    let mut s_hands = hands.clone();
    s_hands.sort_by_key(|k| k.0);
    s_hands
        .iter()
        .enumerate()
        .map(|(idx, (_, bid))| bid * (idx as u64 + 1))
        .sum()
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Hand2([CamelCard2; 5]);

impl From<Hand> for Hand2 {
    fn from(h: Hand) -> Hand2 {
        let mut ret_val: [CamelCard2; 5] = [CamelCard2::NotJoker(CamelCard::Ace); 5];
        for (idx, card) in h.0.iter().enumerate() {
            ret_val[idx] = (*card).into();
        }
        Self(ret_val)
    }
}

impl FromStr for Hand2 {
    type Err = AdventError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Hand::from_str(s)?.into())
    }
}

impl Hand2 {
    fn non_jokers_present(&self) -> HashMap<CamelCard, usize> {
        let mut out_val: HashMap<CamelCard, usize> = HashMap::new();
        for card in self.0 {
            match card {
                CamelCard2::NotJoker(k) => {
                    if let Some(v) = out_val.get_mut(&k) {
                        *v += 1;
                    } else {
                        out_val.insert(k, 1);
                    }
                }
                CamelCard2::Joker => { /* pass */ }
            }
        }
        out_val
    }

    fn most_common_non_joker(&self) -> Option<CamelCard> {
        let mut pairs: Vec<(CamelCard, usize)> = self
            .non_jokers_present()
            .iter()
            .map(|(c, v)| (*c, *v))
            .collect();
        pairs.sort_by_key(|(_, v)| *v);
        pairs.iter().map(|(c, _)| *c).last()
    }

    fn replace_jokers(&self) -> Hand {
        match self.most_common_non_joker() {
            // if they are all jokers, can make it a five-of-a-kind
            None => Hand::from_str("AAAAA").unwrap(),
            Some(repl_card) => {
                /* The best outcome is to replace the Jokers with the most common card, because
                 * all of the hand-types are invariant under permutation of cards */
                let mut out_val: [CamelCard; 5] = [CamelCard::Ace; 5];
                for (idx, card) in self.0.iter().enumerate() {
                    let new_card: CamelCard = match card {
                        CamelCard2::Joker => repl_card,
                        CamelCard2::NotJoker(k) => *k,
                    };
                    out_val[idx] = new_card;
                }
                Hand(out_val)
            }
        }
    }

    fn hand_type(&self) -> HandType {
        self.replace_jokers().hand_type()
    }
}

impl Ord for Hand2 {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.hand_type().cmp(&other.hand_type()) {
            Ordering::Equal => {
                for (self_card, other_card) in self.0.iter().zip(other.0.iter()) {
                    match self_card.cmp(&other_card) {
                        Ordering::Equal => { /* pass */ }
                        k => {
                            return k;
                        }
                    }
                }
                Ordering::Equal
            }
            k => k,
        }
    }
}

impl PartialOrd for Hand2 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common;

    const EXAMPLE_INPUT: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    #[test]
    fn parse_test() {
        let h1: Hand = Hand::from_str("35KJA").unwrap();
        let h2: Hand = Hand([
            CamelCard::Three,
            CamelCard::Five,
            CamelCard::King,
            CamelCard::Jack,
            CamelCard::Ace,
        ]);
        assert_eq!(h1, h2);
    }

    #[test]
    fn hand_ordering_test() {
        assert!(Hand::from_str("22222").unwrap() > Hand::from_str("AAAAK").unwrap());
        assert!(Hand::from_str("22222").unwrap() < Hand::from_str("AAAAA").unwrap());
        assert!(Hand::from_str("22223").unwrap() > Hand::from_str("AAAKK").unwrap());
        assert!(Hand::from_str("QQQ55").unwrap() > Hand::from_str("KKKAQ").unwrap());
        assert!(Hand::from_str("33343").unwrap() > Hand::from_str("AAKK2").unwrap());
        assert!(Hand::from_str("22334").unwrap() > Hand::from_str("AAKQJ").unwrap());
        assert!(Hand::from_str("22345").unwrap() > Hand::from_str("AKQJT").unwrap());
        assert!(Hand::from_str("22345").unwrap() == Hand::from_str("22345").unwrap());
        assert!(Hand::from_str("33343").unwrap() < Hand::from_str("43333").unwrap());
        assert!(Hand::from_str("KKAAA").unwrap() < Hand::from_str("KAKKA").unwrap());
    }

    #[test]
    fn hand2_ordering_test() {
        assert!(Hand2::from_str("2222J").unwrap() > Hand2::from_str("KAKKJ").unwrap());
        assert!(Hand2::from_str("JAAAJ").unwrap() < Hand2::from_str("33333").unwrap());
    }

    #[test]
    fn hand_test() {
        assert_eq!(
            Hand::from_str("33333").unwrap().hand_type(),
            HandType::FiveOfAKind
        );
        assert_eq!(
            Hand::from_str("TTT6T").unwrap().hand_type(),
            HandType::FourOfAKind
        );
        assert_eq!(
            Hand::from_str("Q5Q5Q").unwrap().hand_type(),
            HandType::FullHouse
        );
        assert_eq!(
            Hand::from_str("99879").unwrap().hand_type(),
            HandType::ThreeOfAKind
        );
        assert_eq!(
            Hand::from_str("A2J2J").unwrap().hand_type(),
            HandType::TwoPair
        );
        assert_eq!(
            Hand::from_str("K47TK").unwrap().hand_type(),
            HandType::OnePair
        );
        assert_eq!(
            Hand::from_str("AKJQT").unwrap().hand_type(),
            HandType::HighCard
        );
    }

    #[test]
    fn part1_test() {
        let example_input = common::split_string(EXAMPLE_INPUT.into());
        assert_eq!(part_1(example_input).unwrap(), 6440);
    }

    #[test]
    fn part2_test() {
        if PART_2_IMPL {
            let example_input = common::split_string(EXAMPLE_INPUT.into());
            assert_eq!(part_2(example_input).unwrap(), 5905);
        }
    }
}
