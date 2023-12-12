/* Solution to 2023 Advent of Code, Day 9
 *
 * Christopher Phan
 */

use std::error::Error;
use std::str::FromStr;

use crate::common;

const DAY: usize = 9;
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

pub fn part_1(input: Vec<String>) -> Result<i64, Box<dyn Error>> {
    let line_vecs: Vec<Vec<i64>> = input.iter().map(|k| parse_line(k.clone())).collect();
    let line_sums: Vec<i64> = line_vecs.iter().map(|v| part_1_line(v.to_vec())).collect();
    Ok(line_sums.iter().sum())
}

pub fn part_2(input: Vec<String>) -> Result<i64, Box<dyn Error>> {
    let line_vecs: Vec<Vec<i64>> = input.iter().map(|k| parse_line(k.clone())).collect();
    let line_sums: Vec<i64> = line_vecs.iter().map(|v| part_2_line(v.to_vec())).collect();
    Ok(line_sums.iter().sum())
}

fn vec_diff(v: Vec<i64>) -> Option<Vec<i64>> {
    if v.len() > 0 {
        Some((0..(v.len() - 1)).map(|k| v[k + 1] - v[k]).collect())
    } else {
        None
    }
}

fn derive(v: Vec<i64>) -> Vec<Vec<i64>> {
    let mut out_vec: Vec<Vec<i64>> = vec![v.clone()];
    let mut cur_vec = v.clone();
    while cur_vec.len() > 1 && cur_vec.iter().any(|k| *k != 0) {
        cur_vec = vec_diff(cur_vec.clone()).unwrap();
        out_vec.push(cur_vec.clone());
    }
    out_vec
}

fn part_2_extrapol(v: Vec<Vec<i64>>) -> Vec<i64> {
    let mut last_val: i64 = 0;
    let mut v = v;
    let mut out_vec: Vec<i64> = vec![];
    while !v.is_empty() {
        let w = v.pop().unwrap();
        last_val = w.get(0).unwrap() - last_val;
        out_vec.push(last_val);
    }
    out_vec
}

/* Part 1 shortcut: you can take the sum of the last element of each line of the resuling triangle
 * E.g. for the example input line 1, 3, 6, 10, 15, 21, we get the differences
 *  1   3   6  10  15  21
 *    2   3   4   5   6
 *      1   1   1   1
 *        0   0   0
 *  and the value of this history is 0 + 1 + 6 + 21 = 28.
 * */
fn part_1_line(v: Vec<i64>) -> i64 {
    match v.len() {
        0 => 0_i64,
        1 => *v.last().unwrap(),
        _ => derive(v).iter().map(|k| k.last().unwrap()).sum(),
    }
}

fn part_2_line(v: Vec<i64>) -> i64 {
    match v.len() {
        0 => 0_i64,
        1 => v[0],
        _ => part_2_extrapol(derive(v)).pop().unwrap(),
    }
}

fn parse_line(line: String) -> Vec<i64> {
    line.split_whitespace()
        .map(|k| i64::from_str(k))
        .filter(|k| k.is_ok())
        .map(|k| k.unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common;

    const EXAMPLE_INPUT: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

    #[test]
    fn part1_line_test() {
        let vec: Vec<i64> = parse_line(
            common::split_string(EXAMPLE_INPUT.into())
                .last()
                .unwrap()
                .into(),
        );
        assert_eq!(part_1_line(vec), 68);
    }

    #[test]
    fn part2_line_test1() {
        let example_input = common::split_string(EXAMPLE_INPUT.into());
        assert_eq!(
            part_2_line(parse_line(example_input.get(0).unwrap().to_string())),
            -3
        );
    }

    #[test]
    fn part2_line_test2() {
        let example_input = common::split_string(EXAMPLE_INPUT.into());
        assert_eq!(
            part_2_line(parse_line(example_input.get(1).unwrap().to_string())),
            0
        );
    }

    #[test]
    fn part2_line_test3() {
        let example_input = common::split_string(EXAMPLE_INPUT.into());
        assert_eq!(
            part_2_line(parse_line(example_input.get(2).unwrap().to_string())),
            5
        );
    }

    #[test]
    fn vec_diff_test_a() {
        let v1: Vec<i64> = vec![1, -4, -9, -11, -3, 26, 91];
        let v2 = vec_diff(v1).unwrap();
        assert_eq!(v2, vec![-5, -5, -2, 8, 29, 65]);
        let v3 = vec_diff(v2).unwrap();
        assert_eq!(v3, vec![0, 3, 10, 21, 36]);
        let v4 = vec_diff(v3).unwrap();
        assert_eq!(v4, vec![3, 7, 11, 15]);
        let v5 = vec_diff(v4).unwrap();
        assert_eq!(v5, vec![4, 4, 4]);
        let v6 = vec_diff(v5).unwrap();
        assert_eq!(v6, vec![0, 0]);
    }

    #[test]
    fn vec_diff_test_b() {
        let v1: Vec<i64> = vec![7, 1, -4, -9, -11, -3, 26, 91];
        let v2 = vec_diff(v1).unwrap();
        assert_eq!(v2, vec![-6, -5, -5, -2, 8, 29, 65]);
        let v3 = vec_diff(v2).unwrap();
        assert_eq!(v3, vec![1, 0, 3, 10, 21, 36]);
        let v4 = vec_diff(v3).unwrap();
        assert_eq!(v4, vec![-1, 3, 7, 11, 15]);
        let v5 = vec_diff(v4).unwrap();
        assert_eq!(v5, vec![4, 4, 4, 4]);
        let v6 = vec_diff(v5).unwrap();
        assert_eq!(v6, vec![0, 0, 0]);
    }

    #[test]
    fn part1_line_test_a() {
        assert_eq!(part_1_line(vec![1, -4, -9, -11, -3, 26, 91]), 211);
    }

    #[test]
    fn part2_line_test_a() {
        assert_eq!(part_2_line(vec![1, -4, -9, -11, -3, 26, 91]), 7);
    }

    #[test]
    fn part1_test() {
        let example_input = common::split_string(EXAMPLE_INPUT.into());
        assert_eq!(part_1(example_input).unwrap(), 114);
    }

    #[test]
    fn part2_test() {
        if PART_2_IMPL {
            let example_input = common::split_string(EXAMPLE_INPUT.into());
            assert_eq!(part_2(example_input).unwrap(), 2);
        }
    }
}
