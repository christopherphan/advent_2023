/* Solution to 2023 Advent of Code, Day 6
 *
 * Christopher Phan
 */

use std::cmp::Ordering;
use std::error::Error;
use std::iter;
use std::str::FromStr;

use crate::common;
use crate::common::AdventError;

const DAY: usize = 6;
const PART_2_IMPL: bool = true;

pub fn run() {
    let input = common::get_day(DAY).unwrap();
    println!(
        "{}",
        common::soln_output(DAY, 1, part_1(input.clone()).unwrap())
    );
    if PART_2_IMPL {
        println!("{}", common::soln_output(DAY, 2, part_2(input).unwrap()));
    }
}

pub fn part_1(input: Vec<String>) -> Result<u64, Box<dyn Error>> {
    Ok(RaceCollection::parse_input(input)?.ways_to_win())
}

pub fn part_2(input: Vec<String>) -> Result<u64, Box<dyn Error>> {
    let rekerned_input: Vec<String> = input.iter().map(|k| rekern_line(k.into())).collect();
    Ok(RaceCollection::parse_input(rekerned_input)?.ways_to_win())
}

fn rekern_line(s: String) -> String {
    let parts: Vec<String> = s.split(':').map(|k| k.to_string()).collect();
    if parts.len() < 2 {
        s
    } else {
        let numbers: String = parts[1].replace(' ', "");
        format!("{}: {}", parts[0], numbers)
    }
}

struct Race {
    duration: u64,
    record: u64,
}

impl Race {
    fn distance(&self, button_held: u64) -> u64 {
        if button_held <= self.duration {
            button_held * (self.duration - button_held)
        } else {
            0
        }
    }

    /* We will beat the record for values of t such that self.distance(t) > self.record.
     * To determine how many values there are, we first solve the quadratic equation
     * self.distance(t) == self.record. If there are two solutions, any integers strictly between
     * these are values that beat the record.
     */

    // Calculate the discriminant of the quadratic equation
    fn desc_i(&self) -> i128 {
        (self.duration.pow(2) as i128) - 4 * (self.record as i128)
    }

    fn raw_zeros(&self) -> Option<(f64, f64)> {
        let desc_i = self.desc_i();
        match desc_i.cmp(&0) {
            Ordering::Less => None,
            Ordering::Equal => {
                let d = self.duration as f64;
                Some((d / 2.0, d / 2.0))
            }
            Ordering::Greater => {
                let d = self.duration as f64;
                let sr = (desc_i as f64).sqrt();
                Some(((d - sr) / 2.0, (d + sr) / 2.0))
            }
        }
    }

    fn winning_range(&self) -> Option<(u64, u64)> {
        if self.desc_i() > 0 {
            let (lower_root, upper_root) = self.raw_zeros().unwrap();
            // find lowest winning integer (being carefule because casting to int can cause small
            // errors)
            let lower_root_floor = lower_root.floor() as u64;
            let mut lower = if lower_root_floor > 1 {
                lower_root_floor - 1
            } else {
                0
            };
            while self.distance(lower) <= self.record {
                lower += 1;
            }
            // find highest winning integer
            let upper_root_ceiling = upper_root.ceil() as u64;
            let mut upper = upper_root_ceiling + 1;
            while self.distance(upper) <= self.record {
                upper -= 1;
            }
            Some((lower, upper))
        } else {
            None
        }
    }

    fn ways_to_win(&self) -> u64 {
        match self.winning_range() {
            Some((lower, upper)) => upper - lower + 1,
            None => 0,
        }
    }
}

struct RaceCollection(Vec<Race>);

impl RaceCollection {
    fn ways_to_win(&self) -> u64 {
        self.0.iter().map(|k| k.ways_to_win()).product()
    }

    fn parse_input(input: Vec<String>) -> Result<Self, Box<dyn Error>> {
        if input.len() < 2 {
            Err(Box::new(AdventError("input has too few lines".into())))
        } else {
            let durations: Vec<u64> = input[0]
                .strip_prefix("Time:")
                .ok_or(AdventError("Time line of input malformed".into()))?
                .trim()
                .split_whitespace()
                .map(|k| u64::from_str(k))
                .filter(|k| k.is_ok())
                .map(|k| k.unwrap())
                .collect();
            let records: Vec<u64> = input[1]
                .strip_prefix("Distance:")
                .ok_or(AdventError("Distance line of input malformed".into()))?
                .trim()
                .split_whitespace()
                .map(|k| u64::from_str(k))
                .filter(|k| k.is_ok())
                .map(|k| k.unwrap())
                .collect();
            if records.len() != durations.len() {
                Err(Box::new(AdventError(
                    "malformed input: # of times != # of distances".into(),
                )))
            } else {
                let races: Vec<Race> = iter::zip(durations, records)
                    .map(|(duration, record)| Race { duration, record })
                    .collect();
                Ok(Self(races))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common;

    const EXAMPLE_INPUT: &str = "Time:      7  15   30
Distance:  9  40  200";

    #[test]
    fn winning_range_7_9_test() {
        let r = Race {
            duration: 7,
            record: 9,
        };
        assert_eq!(r.winning_range().unwrap(), (2, 5));
    }

    #[test]
    fn part1_test() {
        let example_input = common::split_string(EXAMPLE_INPUT.into());
        assert_eq!(part_1(example_input).unwrap(), 288);
    }

    #[test]
    fn part2_test() {
        if PART_2_IMPL {
            let example_input = common::split_string(EXAMPLE_INPUT.into());
            assert_eq!(part_2(example_input).unwrap(), 71503);
        }
    }
}
