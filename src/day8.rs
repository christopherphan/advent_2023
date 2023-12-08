/* Solution to 2023 Advent of Code, Day 8
 *
 * Christopher Phan
 */

use std::collections::HashMap;
use std::error::Error;

use crate::common;
use crate::common::AdventError;

const DAY: usize = 8;
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
    let all_data = AllData::parse(input);
    let start = all_data.get_start1()?;
    all_data.num_moves(start, false)
}

pub fn part_2(input: Vec<String>) -> Result<u64, Box<dyn Error>> {
    Ok(AllData::parse(input).num_moves_2()?)
}

#[derive(Copy, Clone, Debug)]
enum Direction {
    Left,
    Right,
}

impl TryFrom<char> for Direction {
    type Error = AdventError;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c.to_ascii_uppercase() {
            'R' => Ok(Direction::Right),
            'L' => Ok(Direction::Left),
            k => Err(AdventError(format!("invalid direction: {}", k))),
        }
    }
}

impl Direction {
    fn parse_seq(s: &str) -> Vec<Self> {
        s.chars()
            .map(|c| Direction::try_from(c))
            .filter(|k| k.is_ok())
            .map(|k| k.unwrap())
            .collect()
    }
}

#[derive(Clone, Debug)]
struct Location {
    name: String,
    left: String,
    right: String,
}

impl Location {
    fn get_key(&self, direction: Direction) -> String {
        match direction {
            Direction::Left => self.left.clone(),
            Direction::Right => self.right.clone(),
        }
    }

    fn is_start(&self, part2: bool) -> bool {
        if part2 {
            self.name.chars().last().unwrap() == 'A'
        } else {
            self.name == "AAA".to_string()
        }
    }

    fn is_end(&self, part2: bool) -> bool {
        if part2 {
            self.name.chars().last().unwrap() == 'Z'
        } else {
            self.name == "ZZZ".to_string()
        }
    }
}

#[derive(Clone, Debug)]
struct LocationData(HashMap<String, Location>);

impl LocationData {
    fn make_move(
        &self,
        current_loc: Location,
        direction: Direction,
    ) -> Result<Location, Box<dyn Error>> {
        let next_place_key = current_loc.get_key(direction);
        Ok(self
            .0
            .get(&next_place_key)
            .ok_or(AdventError(format!(
                "invalid location key: {}",
                next_place_key
            )))?
            .clone())
    }
}

#[derive(Clone, Debug)]
struct AllData {
    loc_data: LocationData,
    move_seq: Vec<Direction>,
}

impl AllData {
    fn parse(input: Vec<String>) -> Self {
        let mut loc_data_hm: HashMap<String, Location> = HashMap::new();
        let mut move_seq: Vec<Direction> = vec![];
        for (idx, line) in input.iter().enumerate() {
            if idx == 0 {
                move_seq = Direction::parse_seq(&line);
            } else if idx >= 2 {
                let parts: Vec<String> = line.split('=').map(|k| k.to_string()).collect();
                if parts.len() > 1 {
                    let loc_name = parts[0].trim();
                    let directions_parts: Vec<String> = parts[1]
                        .replace(&['(', ')'], "")
                        .split(',')
                        .map(|k| k.to_string())
                        .collect();
                    if directions_parts.len() > 1 {
                        let loc = Location {
                            name: loc_name.into(),
                            left: directions_parts[0].trim().into(),
                            right: directions_parts[1].trim().into(),
                        };
                        loc_data_hm.insert(loc_name.into(), loc);
                    }
                }
            }
        }
        Self {
            loc_data: LocationData(loc_data_hm),
            move_seq,
        }
    }

    fn get_start1(&self) -> Result<Location, Box<dyn Error>> {
        Ok(self
            .loc_data
            .0
            .get("AAA")
            .ok_or(AdventError::new("no start!"))?
            .clone())
    }

    fn get_start2(&self) -> Vec<Location> {
        self.loc_data
            .0
            .clone()
            .into_values()
            .filter(|loc| loc.is_start(true))
            .collect()
    }

    fn num_moves(&self, start_loc: Location, part2: bool) -> Result<u64, Box<dyn Error>> {
        let mut out_val: u64 = 0;
        let move_seq_len = self.move_seq.len();
        let mut move_pos: usize = 0;
        let mut current_loc = start_loc.clone();
        while !current_loc.is_end(part2) && out_val < u64::MAX {
            let next_move = self.move_seq[move_pos];
            move_pos = if move_pos == move_seq_len - 1 {
                0
            } else {
                move_pos + 1
            };
            current_loc = self.loc_data.make_move(current_loc.clone(), next_move)?;
            out_val += 1;
        }
        if current_loc.is_end(part2) {
            Ok(out_val)
        } else {
            Err(Box::new(AdventError(format!(
                "took more than {} moves!!!",
                u64::MAX
            ))))
        }
    }

    /* LOOK, you can try to simulate all the ghosts at once, but that takes a long time. But note
     * that the ghosts act independently, going in a cycle, until they are all at the end. Figure
     * out the cycle length of each ghost, then you are looking for the LCM of all the cycle
     * lengths. */

    fn num_moves_2(&self) -> Result<u64, Box<dyn Error>> {
        let mut cycle_lengths: Vec<u64> = vec![];
        for k in self.get_start2() {
            cycle_lengths.push(self.num_moves(k.clone(), true)?);
        }
        let mut out_val = cycle_lengths.pop().unwrap();
        while !cycle_lengths.is_empty() {
            out_val = lcm(out_val, cycle_lengths.pop().unwrap());
        }
        Ok(out_val)
    }
}

fn fast_euclid(a: u64, b: u64) -> u64 {
    let mut u = a;
    let mut v = b;
    while u != 0 {
        let t = u;
        u = v % u;
        v = t;
    }
    v
}

fn lcm(a: u64, b: u64) -> u64 {
    a * b / fast_euclid(a, b)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common;

    const EXAMPLE_INPUT_1: &str = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

    const EXAMPLE_INPUT_2: &str = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

    const EXAMPLE_INPUT_3: &str = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

    #[test]
    fn part1_test() {
        let example_input_1 = common::split_string(EXAMPLE_INPUT_1.into());
        assert_eq!(part_1(example_input_1).unwrap(), 2);
        let example_input_2 = common::split_string(EXAMPLE_INPUT_2.into());
        assert_eq!(part_1(example_input_2).unwrap(), 6);
    }

    #[test]
    fn part2_test() {
        if PART_2_IMPL {
            let example_input = common::split_string(EXAMPLE_INPUT_3.into());
            assert_eq!(part_2(example_input).unwrap(), 6);
        }
    }
}
