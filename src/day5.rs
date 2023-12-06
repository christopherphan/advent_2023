/* Solution to 2023 Advent of Code, Day 5
 *
 * Christopher Phan
 */

use std::error::Error;
use std::num::NonZeroUsize;
use std::str::FromStr;

use crate::common;
use crate::common::AdventError;

pub fn run() {
    let input = common::get_day(5).unwrap();
    println!(
        "{}",
        common::soln_output(5, 1, part_1(input.clone()).unwrap())
    );
    println!("{}", common::soln_output(5, 2, part_2(input).unwrap()));
}

pub fn part_1(input: Vec<String>) -> Result<u64, Box<dyn Error>> {
    part(input, false)
}

pub fn part_2(input: Vec<String>) -> Result<u64, Box<dyn Error>> {
    part(input, true)
}

pub fn part(input: Vec<String>, part2: bool) -> Result<u64, Box<dyn Error>> {
    let (seeds, map_stack) = parse_input(input, part2)?;
    match map_stack.apply(&seeds).min() {
        Some(k) => Ok(k as u64),
        None => Err(Box::new(AdventError("No locations".into()))),
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
struct IntegerInterval {
    first: usize,
    length: NonZeroUsize,
}

impl IntegerInterval {
    fn single(val: usize) -> Self {
        Self {
            first: val,
            length: NonZeroUsize::new(1).unwrap(),
        }
    }

    fn last(&self) -> usize {
        self.first + self.length.get() - 1
    }

    fn overlapping(&self, other: &Self) -> bool {
        let max_first = self.first.max(other.first);
        let min_last = self.last().min(other.last());
        max_first <= min_last
    }

    fn overlap(&self, other: &Self) -> Option<Self> {
        let max_first = self.first.max(other.first);
        let min_last = self.last().min(other.last());
        if max_first <= min_last {
            let length = NonZeroUsize::new(min_last - max_first + 1).unwrap();
            Some(Self {
                first: max_first,
                length,
            })
        } else {
            None
        }
    }

    fn set_minus(&self, other: &Self) -> Vec<Self> {
        match self.overlap(other) {
            Some(interval) => {
                let mut out_vec: Vec<Self> = vec![];
                if self.first < interval.first {
                    out_vec.push(Self {
                        first: self.first,
                        length: NonZeroUsize::new(interval.first - self.first).unwrap(),
                    });
                }
                if self.last() > interval.last() {
                    out_vec.push(Self {
                        first: interval.last() + 1,
                        length: NonZeroUsize::new(self.last() - interval.last()).unwrap(),
                    });
                }
                out_vec
            }
            None => vec![self.clone()],
        }
    }

    fn combine(&self, other: &Self) -> Vec<Self> {
        let max_first = self.first.max(other.first);
        let min_last = self.last().min(other.last());
        if max_first <= min_last + 1 {
            // overlap or adjacent
            let first = self.first.min(other.first);
            let max_last = self.last().max(other.last());
            let length = NonZeroUsize::new(max_last - first + 1).unwrap();
            vec![Self { first, length }]
        } else {
            // not overlapping or adjacent
            vec![*self, *other]
        }
    }

    fn combine_vector(vals: Vec<Self>) -> Vec<Self> {
        let mut val_copy = vals.clone();
        val_copy.sort_by_key(|k| k.first);
        let mut out_vec: Vec<Self> = vec![];
        for val in val_copy {
            if out_vec.is_empty() {
                out_vec.push(val);
            } else {
                let rightmost = out_vec.pop().unwrap();
                let mut to_push = rightmost.combine(&val);
                out_vec.append(&mut to_push);
            }
        }
        out_vec
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct IntegerIntervalUnion(Vec<IntegerInterval>);

impl IntegerIntervalUnion {
    fn simplify(&mut self) {
        self.0 = IntegerInterval::combine_vector(self.0.clone());
    }

    fn min(&self) -> Option<usize> {
        self.0.iter().map(|k| k.first).min()
    }

    fn read_seeds(input: String, part2: bool) -> Self {
        let parts: Vec<&str> = input.split_whitespace().collect();
        let mut input_numbers: Vec<usize> = parts
            .iter()
            .map(|k| usize::from_str(k))
            .filter(|k| k.is_ok())
            .map(|k| k.unwrap())
            .collect();
        if part2 {
            let mut ranges: Vec<IntegerInterval> = vec![];
            while input_numbers.len() > 1 {
                let length_raw = input_numbers.pop().unwrap();
                let first = input_numbers.pop().unwrap();
                if length_raw > 0 {
                    ranges.push(IntegerInterval {
                        first,
                        length: NonZeroUsize::new(length_raw).unwrap(),
                    });
                }
            }
            let mut out_val = Self(ranges);
            out_val.simplify();
            out_val
        } else {
            Self(
                input_numbers
                    .iter()
                    .map(|k| IntegerInterval::single(*k))
                    .collect(),
            )
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
struct RangeShift {
    source_interval: IntegerInterval,
    destination_start: usize,
}

#[derive(PartialEq, Eq, Clone, Debug)]
struct RangeShiftResult {
    image: Option<IntegerInterval>,
    not_mapped: Vec<IntegerInterval>,
}

impl RangeShift {
    fn applies_to_interval(&self, val: &IntegerInterval) -> bool {
        self.source_interval.overlapping(val)
    }

    fn apply_to_interval(&self, val: &IntegerInterval) -> RangeShiftResult {
        let not_mapped: Vec<IntegerInterval> = val.set_minus(&self.source_interval);
        let image: Option<IntegerInterval> =
            if let Some(overlap) = val.overlap(&self.source_interval) {
                Some(IntegerInterval {
                    first: self.destination_start + overlap.first - self.source_interval.first,
                    length: overlap.length,
                })
            } else {
                None
            };
        RangeShiftResult { image, not_mapped }
    }

    fn read_line(s: String) -> Result<Self, Box<dyn Error>> {
        let parts: Vec<&str> = s.split_whitespace().collect();
        let destination_start: usize = parts
            .get(0)
            .ok_or(AdventError("malformed input".into()))?
            .parse()?;
        let first: usize = parts
            .get(1)
            .ok_or(AdventError("malformed input".into()))?
            .parse()?;
        let length = NonZeroUsize::new(
            parts
                .get(2)
                .ok_or(AdventError("malformed input".into()))?
                .parse()?,
        )
        .ok_or(AdventError("zero-length range".into()))?;
        let source_interval = IntegerInterval { first, length };

        Ok(RangeShift {
            source_interval,
            destination_start,
        })
    }
}

#[derive(Clone, Debug)]
struct RangeShiftStack(Vec<RangeShift>);

impl RangeShiftStack {
    fn apply(&self, int_union: &IntegerIntervalUnion) -> IntegerIntervalUnion {
        let stack_len: usize = self.0.len();
        let mut out_vec: Vec<IntegerInterval> = vec![];
        let mut source_vec: Vec<IntegerInterval> = int_union.0.clone();
        while !source_vec.is_empty() {
            let mut pos: usize = 0;
            let mut matched = false;
            let next_int = source_vec.pop().unwrap();
            while !matched && pos < stack_len {
                if self.0[pos].applies_to_interval(&next_int) {
                    let mut res = self.0[pos].apply_to_interval(&next_int);
                    source_vec.append(&mut res.not_mapped);
                    out_vec.push(res.image.unwrap());
                    matched = true;
                } else {
                    pos += 1;
                }
            }
            if !matched {
                out_vec.push(next_int);
            }
        }
        let mut ret_val = IntegerIntervalUnion(out_vec);
        ret_val.simplify();
        ret_val
    }
}

#[derive(Clone, Debug)]
struct RangeShiftStackSequence(Vec<RangeShiftStack>);

impl RangeShiftStackSequence {
    fn apply(&self, val: &IntegerIntervalUnion) -> IntegerIntervalUnion {
        let mut out_val = val.clone();
        for stack in self.0.clone() {
            out_val = stack.apply(&out_val);
        }
        out_val
    }
}

fn parse_input(
    input: Vec<String>,
    part2: bool,
) -> Result<(IntegerIntervalUnion, RangeShiftStackSequence), Box<dyn Error>> {
    let mut seeds = IntegerIntervalUnion(vec![]);
    let mut map_stack = RangeShiftStackSequence(vec![]);
    let mut current_rss = RangeShiftStack(vec![]);
    let mut ignore_next = true;
    for (idx, line) in input.iter().enumerate() {
        if idx == 0 {
            let parts: Vec<String> = line.split(':').map(|k| k.trim().into()).collect();
            seeds = IntegerIntervalUnion::read_seeds(
                parts
                    .get(1)
                    .ok_or(AdventError("malformed input (seeds)".into()))?
                    .into(),
                part2,
            );
        } else if idx == 1 {
            /* pass */
        } else {
            if ignore_next {
                ignore_next = false;
            } else if line.trim().is_empty() {
                map_stack.0.push(current_rss);
                current_rss = RangeShiftStack(vec![]);
            } else {
                if let Ok(shift) = RangeShift::read_line(line.to_string()) {
                    current_rss.0.push(shift);
                }
            }
        }
    }
    if !current_rss.0.is_empty() {
        map_stack.0.push(current_rss);
    }
    Ok((seeds, map_stack))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common;

    const EXAMPLE_INPUT: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    #[test]
    fn part1_test() {
        let example_input = common::split_string(EXAMPLE_INPUT.into());
        assert_eq!(part_1(example_input).unwrap(), 35);
    }

    #[test]
    fn part2_test() {
        let example_input = common::split_string(EXAMPLE_INPUT.into());
        assert_eq!(part_2(example_input).unwrap(), 46);
    }

    #[test]
    fn part1_seeds_test() {
        let example_input = common::split_string(EXAMPLE_INPUT.into());
        let (seeds, _) = parse_input(example_input, false).unwrap();
        assert_eq!(seeds.min().unwrap(), 13);
    }

    #[test]
    fn part2_seeds_test() {
        let example_input = common::split_string(EXAMPLE_INPUT.into());
        let (seeds, _) = parse_input(example_input, true).unwrap();
        assert_eq!(seeds.min().unwrap(), 55);
    }

    #[test]
    fn overlap_test1() {
        let interval = IntegerInterval {
            first: 50,
            length: NonZeroUsize::new(48).unwrap(),
        };
        let val = IntegerInterval::single(79);
        assert_eq!(interval.overlap(&val), Some(val));
    }

    #[test]
    fn range_shift_test1() {
        let rs = RangeShift::read_line("52 50 48".into()).unwrap();
        let val = IntegerInterval::single(79);
        let output = rs.apply_to_interval(&val);
        assert_eq!(output.image, Some(IntegerInterval::single(81)));
        assert!(output.not_mapped.is_empty());
    }

    #[test]
    fn range_shift_test2() {
        let rss = RangeShiftStack(vec![
            RangeShift::read_line("50 98 2".into()).unwrap(),
            RangeShift::read_line("52 50 48".into()).unwrap(),
        ]);
        let output = rss.apply(&IntegerIntervalUnion(vec![IntegerInterval::single(79)]));
        assert_eq!(output.0.len(), 1);
        assert_eq!(output.0[0], IntegerInterval::single(81));
    }
}
