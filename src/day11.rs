/* Solution to 2023 Advent of Code, Day 11
 *
 * Christopher Phan
 */

use std::collections::HashMap;
use std::error::Error;

use crate::common;
use crate::common::AdventError;

const DAY: usize = 11;
const PART_1_IMPL: bool = false;
const PART_2_IMPL: bool = false;

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
    Err(Box::new(AdventError::not_impl()))
}

pub fn part_2(input: Vec<String>) -> Result<u64, Box<dyn Error>> {
    Err(Box::new(AdventError::not_impl()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common;

    const EXAMPLE_INPUT: &str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

    fn get_example_input() -> Vec<String> {
        common::split_string(EXAMPLE_INPUT.to_string())
    }

    #[test]
    fn part1_test() {
        assert_eq!(part_1(get_example_input()).unwrap(), 374);
    }

    #[test]
    fn part2_test() {
        if PART_2_IMPL {
            assert_eq!(part_2(get_example_input()).unwrap(), 374);
        }
    }
}
