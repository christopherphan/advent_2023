/* Solution to 2023 Advent of Code, Day 1
 *
 * Christopher Phan
 */

use std::error::Error;

use crate::common;
use crate::common::AdventError;

pub fn run() {
    let day1_input = common::get_day(1).unwrap();
    println!(
        "{}",
        common::soln_output(1, 1, part_1(day1_input.clone()).unwrap())
    );
    println!("{}", common::soln_output(1, 2, part_2(day1_input).unwrap()));
}

pub fn part_1(input: Vec<String>) -> Result<u64, Box<dyn Error>> {
    Ok(input
        .iter()
        .map(get_number_part_1)
        .filter(|k| k.is_ok())
        .map(|k| k.unwrap())
        .sum())
}

pub fn part_2(input: Vec<String>) -> Result<u64, Box<dyn Error>> {
    Ok(input
        .iter()
        .map(get_number_part_2)
        .filter(|k| k.is_ok())
        .map(|k| k.unwrap())
        .sum())
}

fn check_for_digit_at_start(x: &str, words: bool) -> Result<u64, AdventError> {
    if x.is_empty() {
        Err(AdventError("Empty string".into()))
    } else if let Some(d) = x.chars().next().unwrap().to_digit(10) {
        // check for digit
        Ok(d.into())
    } else if !words {
        Err(AdventError("No digits".into()))
    } else if x.starts_with("one") {
        Ok(1)
    } else if x.starts_with("two") {
        Ok(2)
    } else if x.starts_with("three") {
        Ok(3)
    } else if x.starts_with("four") {
        Ok(4)
    } else if x.starts_with("five") {
        Ok(5)
    } else if x.starts_with("six") {
        Ok(6)
    } else if x.starts_with("seven") {
        Ok(7)
    } else if x.starts_with("eight") {
        Ok(8)
    } else if x.starts_with("nine") {
        Ok(9)
    } else {
        Err(AdventError("No digits".into()))
    }
}

fn get_first_digit(s: &str, words: bool) -> Result<u64, AdventError> {
    for k in 0..s.len() {
        if let Ok(d) = check_for_digit_at_start(&s[k..], words) {
            return Ok(d);
        }
    }
    Err(AdventError("No digits found".into()))
}

fn get_last_digit(s: &str, words: bool) -> Result<u64, AdventError> {
    let ell = s.len() - 1;
    for k in 0..s.len() {
        if let Ok(d) = check_for_digit_at_start(&s[(ell - k)..], words) {
            return Ok(d);
        }
    }
    Err(AdventError("No digits found".into()))
}

fn get_number(s: &str, words: bool) -> Result<u64, AdventError> {
    Ok(get_first_digit(s, words)? * 10 + get_last_digit(s, words)?)
}

fn get_number_part_1(s: &String) -> Result<u64, AdventError> {
    get_number(s, false)
}

fn get_number_part_2(s: &String) -> Result<u64, AdventError> {
    get_number(s, true)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common;

    #[test]
    fn part1_test() {
        let example_input = common::split_string(
            "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"
                .into(),
        );
        assert_eq!(part_1(example_input).unwrap(), 142);
    }
    #[test]
    fn part2_test() {
        let example_input = common::split_string(
            "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"
                .into(),
        );
        assert_eq!(part_2(example_input).unwrap(), 281);
    }
}
