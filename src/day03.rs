/* Solution to 2023 Advent of Code, Day 3
 *
 * Christopher Phan
 */

use std::error::Error;
use std::str::FromStr;

use crate::common;

pub fn run() {
    let day3_input = common::get_day(3).unwrap();
    println!(
        "{}",
        common::soln_output(3, 1, part_1(day3_input.clone()).unwrap())
    );
    println!(
        "{}",
        common::soln_output(3, 2, part_2(day3_input.clone()).unwrap())
    );
}

pub fn part_1(input: Vec<String>) -> Result<u64, Box<dyn Error>> {
    let engine_map = EngineMap::read_map(&input);
    Ok(engine_map.sum_pn())
}
pub fn part_2(input: Vec<String>) -> Result<u64, Box<dyn Error>> {
    let engine_map = EngineMap::read_map(&input);
    Ok(engine_map.sum_gr())
}

#[derive(Copy, Clone, Debug)]
struct MapNumber {
    pub start_pos: (i64, i64),
    pub val: u64,
    pub num_digits: usize,
}

impl MapNumber {
    fn record(start_pos: (i64, i64), text: &str) -> Result<Self, Box<dyn Error>> {
        let val: u64 = u64::from_str(text)?;
        let num_digits = text.len();
        Ok(Self {
            start_pos,
            val,
            num_digits,
        })
    }

    fn adjacent_positions(&self) -> Vec<(i64, i64)> {
        let mut ret_vec: Vec<(i64, i64)> = (-1..2)
            .map(|k| (self.start_pos.0 - k, self.start_pos.1 - 1))
            .collect();
        for k in (self.start_pos.1)..(self.start_pos.1 + self.num_digits as i64) {
            ret_vec.push((self.start_pos.0 - 1, k));
            ret_vec.push((self.start_pos.0 + 1, k));
        }
        for k in -1..2 {
            ret_vec.push((
                self.start_pos.0 + k,
                self.start_pos.1 + self.num_digits as i64,
            ));
        }
        ret_vec
    }
}

#[derive(Copy, Clone, Debug)]
struct MapSymbol {
    pub pos: (i64, i64),
    pub symb: char,
}

impl MapSymbol {
    fn adjacent_numbers(&self, numbers: &Vec<MapNumber>) -> Vec<MapNumber> {
        numbers
            .iter()
            .filter(|n| n.adjacent_positions().contains(&self.pos))
            .cloned()
            .collect()
    }

    fn gear_ratio(&self, numbers: &Vec<MapNumber>) -> u64 {
        if self.symb != '*' {
            0
        } else {
            let adj_num = self.adjacent_numbers(numbers);
            if adj_num.len() == 2 {
                adj_num[0].val * adj_num[1].val
            } else {
                0
            }
        }
    }
}

#[derive(Clone, Debug)]
struct EngineMap {
    pub numbers: Vec<MapNumber>,
    pub symbols: Vec<MapSymbol>,
}

impl EngineMap {
    fn symbol_pos(&self) -> Vec<(i64, i64)> {
        self.symbols.iter().to_owned().map(|k| k.pos).collect()
    }

    fn part_numbers(&self) -> Vec<MapNumber> {
        let symb_pos = self.symbol_pos();
        self.numbers
            .iter()
            .cloned()
            .filter(|n| n.adjacent_positions().iter().any(|k| symb_pos.contains(k)))
            .collect()
    }

    fn sum_pn(&self) -> u64 {
        self.part_numbers().iter().map(|n| n.val).sum()
    }

    fn sum_gr(&self) -> u64 {
        self.symbols
            .iter()
            .map(|k| k.gear_ratio(&self.numbers))
            .sum()
    }

    fn read_map(input: &Vec<String>) -> Self {
        let mut current_num_start: Option<(i64, i64)> = None;
        let mut num_buffer: String = "".into();
        let mut numbers: Vec<MapNumber> = vec![];
        let mut symbols: Vec<MapSymbol> = vec![];
        for (row, row_str) in input.iter().enumerate() {
            for (col, c) in row_str.chars().enumerate() {
                if c.is_ascii_digit() {
                    if current_num_start.is_none() {
                        current_num_start = Some((row as i64, col as i64));
                    }
                    num_buffer.push(c);
                } else {
                    if !num_buffer.is_empty() {
                        numbers.push(
                            MapNumber::record(current_num_start.unwrap(), &(num_buffer.clone()))
                                .unwrap(),
                        );
                        current_num_start = None;
                        num_buffer = "".into();
                    }
                    if c != '.' {
                        symbols.push(MapSymbol {
                            pos: (row as i64, col as i64),
                            symb: c,
                        });
                    }
                }
            }
        }
        Self { numbers, symbols }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common;

    #[test]
    fn part2_test() {
        let example_input = common::split_string(
            "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."
                .into(),
        );
        assert_eq!(part_2(example_input).unwrap(), 467835);
    }

    #[test]
    fn part1_test() {
        let example_input = common::split_string(
            "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."
                .into(),
        );
        assert_eq!(part_1(example_input).unwrap(), 4361);
    }
}
