/* Solution to 2023 Advent of Code, Day 4
 *
 * Christopher Phan
 */

use std::error::Error;
use std::str::FromStr;

use crate::common;
use crate::common::AdventError;

pub fn run() {
    let day4_input = common::get_day(4).unwrap();
    println!("{}", common::soln_output(4, 1, part_1(day4_input.clone())));
    println!("{}", common::soln_output(4, 2, part_2(day4_input)));
}

pub fn part_1(input: Vec<String>) -> u64 {
    let cards = Card::read_input(input);
    cards.iter().map(|k| k.value()).sum()
}

pub fn part_2(input: Vec<String>) -> u64 {
    let mut cards: Vec<SimplifiedCard> = Card::read_input(input)
        .iter()
        .cloned()
        .map(|k| k.into())
        .collect();
    let card_no = CardNumbering(cards.clone());
    let mut cards_won: u64 = 0;
    while !cards.is_empty() {
        let top_card = cards.pop().unwrap();
        cards_won += 1;
        for k in top_card.0 {
            cards.push(card_no.get(k).unwrap());
        }
    }
    cards_won
}

#[derive(Debug, Clone)]
struct Card {
    id: usize,
    left_side: Vec<u64>,
    right_side: Vec<u64>,
}

impl Card {
    fn read_line(s: String) -> Result<Self, Box<dyn Error>> {
        let parts: Vec<String> = s.split([':', '|']).map(|k| k.to_string()).collect();
        let parts0: Vec<String> = parts
            .get(0)
            .ok_or(AdventError("Malformed card".into()))?
            .split_whitespace()
            .map(|k| k.to_string())
            .collect();
        let id: usize = parts0
            .get(1)
            .ok_or(AdventError("Malformed card".into()))?
            .parse()?;
        let left_side: Vec<u64> = parts
            .get(1)
            .ok_or(AdventError("Malformed card".into()))?
            .split_whitespace()
            .map(|k| u64::from_str(k))
            .filter(|k| k.is_ok())
            .map(|k| k.unwrap())
            .collect();
        let right_side: Vec<u64> = parts
            .get(2)
            .ok_or(AdventError("Malformed card".into()))?
            .split_whitespace()
            .map(|k| u64::from_str(k))
            .filter(|k| k.is_ok())
            .map(|k| k.unwrap())
            .collect();
        Ok(Self {
            id,
            left_side,
            right_side,
        })
    }

    fn read_input(input: Vec<String>) -> Vec<Self> {
        input
            .iter()
            .map(|k| Card::read_line(k.to_string()))
            .filter(|k| k.is_ok())
            .map(|k| k.unwrap())
            .collect()
    }

    fn num_matches(&self) -> u32 {
        self.right_side
            .iter()
            .filter(|k| self.left_side.contains(k))
            .count() as u32
    }

    fn value(&self) -> u64 {
        let m = self.num_matches();
        match m {
            0 => 0,
            k => 2_u64.pow(k - 1),
        }
    }
}

#[derive(Debug, Clone)]
struct SimplifiedCard(Vec<usize>);

impl From<Card> for SimplifiedCard {
    fn from(card: Card) -> Self {
        let to_win: Vec<usize> = (0..(card.num_matches() as usize))
            .map(|k| card.id + k + 1)
            .collect();
        Self(to_win)
    }
}

struct CardNumbering(Vec<SimplifiedCard>);

impl CardNumbering {
    fn get(&self, card_id: usize) -> Result<SimplifiedCard, Box<dyn Error>> {
        Ok(self
            .0
            .get(card_id - 1)
            .ok_or(AdventError("Card not found".to_string()))?
            .clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common;

    #[test]
    fn part1_test() {
        let example_input = common::split_string(
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"
                .into(),
        );
        assert_eq!(part_1(example_input), 13);
    }

    #[test]
    fn part2_test() {
        let example_input = common::split_string(
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"
                .into(),
        );
        assert_eq!(part_2(example_input), 30);
    }
}
