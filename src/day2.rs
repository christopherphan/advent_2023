/* Solution to 2023 Advent of Code, Day 2
 *
 * Christopher Phan
 */

use std::error::Error;
use std::ops::{Add, AddAssign};
use std::str::FromStr;

use crate::common;
use crate::common::AdventError;

pub fn run() {
    let day2_input = common::get_day(2).unwrap();
    println!(
        "{}",
        common::soln_output(2, 1, part_1(day2_input.clone()).unwrap())
    );
    println!("{}", common::soln_output(2, 2, part_2(day2_input).unwrap()));
}

pub fn part_1(input: Vec<String>) -> Result<u64, Box<dyn Error>> {
    let bag = BlockCollection {
        red: 12,
        green: 13,
        blue: 14,
    };
    let mut output: u64 = 0;
    for k in input {
        let game = Game::from_str(&k);
        if let Ok(g) = game {
            if g.compatible_game(&bag) {
                output += g.id;
            }
        }
    }
    Ok(output)
}

pub fn part_2(input: Vec<String>) -> Result<u64, Box<dyn Error>> {
    let mut output: u64 = 0;
    for k in input {
        let game = Game::from_str(&k);
        if let Ok(g) = game {
            output += g.power();
        }
    }
    Ok(output)
}

#[derive(Copy, Clone, Debug)]
struct BlockCollection {
    pub red: u64,
    pub green: u64,
    pub blue: u64,
}

impl BlockCollection {
    fn compatible_draw(&self, bag: &Self) -> bool {
        self.red <= bag.red && self.green <= bag.green && self.blue <= bag.blue
    }

    fn parse_segment(s: &str) -> Result<Self, Box<dyn Error>> {
        let mut parts = s.trim().split_whitespace().take(2);
        let number: u64 = parts
            .next()
            .ok_or(AdventError("Bad game syntax".into()))?
            .parse()?;
        let color_str: &str = parts.next().ok_or(AdventError("Bad game syntax".into()))?;
        match color_str {
            "red" => Ok(BlockCollection {
                red: number,
                green: 0,
                blue: 0,
            }),
            "green" => Ok(BlockCollection {
                red: 0,
                green: number,
                blue: 0,
            }),
            "blue" => Ok(BlockCollection {
                red: 0,
                green: 0,
                blue: number,
            }),
            k => Err(Box::new(AdventError(format!("bad color: {}", k)))),
        }
    }

    fn power(&self) -> u64 {
        self.red * self.green * self.blue
    }

    fn max_by_cmpt(&self, other: &Self) -> Self {
        Self {
            red: self.red.max(other.red),
            green: self.green.max(other.green),
            blue: self.blue.max(other.blue),
        }
    }
}

impl FromStr for BlockCollection {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut output = Self {
            red: 0,
            green: 0,
            blue: 0,
        };
        for p in s.trim().split(',') {
            output += Self::parse_segment(p)?;
        }
        Ok(output)
    }
}

// Emmy
// Chris

impl Add for BlockCollection {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Self {
            red: self.red + other.red,
            green: self.green + other.green,
            blue: self.blue + other.blue,
        }
    }
}

impl AddAssign for BlockCollection {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            red: self.red + other.red,
            green: self.green + other.green,
            blue: self.blue + other.blue,
        };
    }
}

// dragon kitty

struct Game {
    pub id: u64,
    pub draws: Vec<BlockCollection>,
}

impl Game {
    fn compatible_game(&self, bag: &BlockCollection) -> bool {
        self.draws.iter().all(|d| d.compatible_draw(bag))
    }

    fn smallest_compat_bag(&self) -> BlockCollection {
        let mut output = BlockCollection {
            red: 0,
            green: 0,
            blue: 0,
        };
        for k in self.draws.iter().cloned() {
            output = output.max_by_cmpt(&k);
        }
        output
    }

    fn power(&self) -> u64 {
        self.smallest_compat_bag().power()
    }
}

impl FromStr for Game {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts1 = s.trim().split(':');
        let game_num_part = parts1.next().ok_or(AdventError("Bad game syntax".into()))?;
        let id: u64 = game_num_part
            .trim()
            .split_whitespace()
            .last()
            .ok_or(AdventError("Bad game syntax".into()))?
            .parse()?;
        let draws_raw: Vec<&str> = parts1
            .next()
            .ok_or(AdventError("Bad game syntax".into()))?
            .trim()
            .split(';')
            .collect();
        let mut draws: Vec<BlockCollection> = vec![];
        for d in draws_raw {
            let d_parsed = BlockCollection::from_str(d)?;
            draws.push(d_parsed);
        }
        Ok(Game { id, draws })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common;

    #[test]
    fn part1_test() {
        let example_input = common::split_string(
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
                .into(),
        );
        assert_eq!(part_1(example_input).unwrap(), 8);
    }

    #[test]
    fn part2_test() {
        let example_input = common::split_string(
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
                .into(),
        );
        assert_eq!(part_2(example_input).unwrap(), 2286);
    }
}
