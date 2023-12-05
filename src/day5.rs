/* Solution to 2023 Advent of Code, Day 5
 *
 * Christopher Phan
 */

use std::error::Error;
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
    let locations: Vec<Obj> = seeds
        .iter()
        .map(|s| ObjMapPart::apply_stack(map_stack.clone(), *s))
        .collect();
    if locations.iter().all(|k| k.obj_type == ObjType::Location) {
        locations
            .iter()
            .map(|k| k.id as u64)
            .min()
            .ok_or(Box::new(AdventError("No locations".into())))
    } else {
        Err(Box::new(AdventError(
            "Not all seeds turned into locations".into(),
        )))
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum ObjType {
    Seed,
    Soil,
    Fertilizer,
    Water,
    Light,
    Temperature,
    Humidity,
    Location,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
struct Obj {
    obj_type: ObjType,
    id: usize,
}

impl Obj {
    fn read_seeds(input: String, part2: bool) -> Vec<Obj> {
        let parts: Vec<&str> = input.split_whitespace().collect();
        let mut input_numbers: Vec<usize> = parts
            .iter()
            .map(|k| usize::from_str(k))
            .filter(|k| k.is_ok())
            .map(|k| k.unwrap())
            .collect();
        let seed_nos: Vec<usize> = if part2 {
            let mut out_vec: Vec<usize> = vec![];
            while input_numbers.len() > 1 {
                let range_len = input_numbers.pop().unwrap();
                let start = input_numbers.pop().unwrap();
                let mut to_append: Vec<usize> = (start..(start + range_len)).collect();
                out_vec.append(&mut to_append);
            }
            out_vec
        } else {
            input_numbers.clone()
        };
        seed_nos
            .iter()
            .map(|k| Obj {
                obj_type: ObjType::Seed,
                id: *k,
            })
            .collect()
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum ObjMapPart {
    Shift {
        source_start: Obj,
        destination_start: Obj,
        range_length: usize,
    },
    NoShift {
        source_type: ObjType,
        destination_type: ObjType,
    },
}

impl ObjMapPart {
    fn apply(&self, x: Obj) -> Obj {
        match self {
            Self::Shift {
                source_start,
                destination_start,
                range_length,
            } => {
                if x.obj_type == source_start.obj_type
                    && x.id >= source_start.id
                    && x.id < source_start.id + range_length
                {
                    Obj {
                        obj_type: destination_start.obj_type,
                        id: x.id - source_start.id + destination_start.id,
                    }
                } else {
                    x
                }
            }
            Self::NoShift {
                source_type,
                destination_type,
            } => {
                if x.obj_type == *source_type {
                    Obj {
                        obj_type: *destination_type,
                        id: x.id,
                    }
                } else {
                    x
                }
            }
        }
    }

    fn read_line(
        s: String,
        source_type: ObjType,
        destination_type: ObjType,
    ) -> Result<Self, Box<dyn Error>> {
        let parts: Vec<&str> = s.split_whitespace().collect();
        let destination_start_id: usize = parts
            .get(0)
            .ok_or(AdventError("malformed input".into()))?
            .parse()?;
        let source_start_id: usize = parts
            .get(1)
            .ok_or(AdventError("malformed input".into()))?
            .parse()?;
        let range_length: usize = parts
            .get(2)
            .ok_or(AdventError("malformed input".into()))?
            .parse()?;

        Ok(Self::Shift {
            source_start: Obj {
                obj_type: source_type,
                id: source_start_id,
            },
            destination_start: Obj {
                obj_type: destination_type,
                id: destination_start_id,
            },
            range_length,
        })
    }

    fn read_lines(
        input: Vec<String>,
        source_type: ObjType,
        destination_type: ObjType,
    ) -> Vec<Self> {
        let mut out_vec: Vec<Self> = input
            .iter()
            .map(|line| Self::read_line(line.into(), source_type, destination_type))
            .filter(|k| k.is_ok())
            .map(|k| k.unwrap())
            .collect();
        out_vec.push(Self::NoShift {
            source_type,
            destination_type,
        });
        out_vec
    }

    fn apply_stack(stack: Vec<Self>, x: Obj) -> Obj {
        let mut y = x;
        for m in stack {
            y = m.apply(y);
        }
        y
    }
}

const OBJ_ORDER: [ObjType; 8] = [
    ObjType::Seed,
    ObjType::Soil,
    ObjType::Fertilizer,
    ObjType::Water,
    ObjType::Light,
    ObjType::Temperature,
    ObjType::Humidity,
    ObjType::Location,
];

fn parse_input(
    input: Vec<String>,
    part2: bool,
) -> Result<(Vec<Obj>, Vec<ObjMapPart>), Box<dyn Error>> {
    let mut source_type_num: usize = 0;
    let mut target_type_num: usize = 1;
    let mut seeds: Vec<Obj> = vec![];
    let mut map_stack: Vec<ObjMapPart> = vec![];
    let mut part_stack: Vec<String> = vec![];
    let mut ignore_next = true;
    for (idx, line) in input.iter().enumerate() {
        if idx == 0 {
            let parts: Vec<String> = line.split(':').map(|k| k.trim().into()).collect();
            seeds = Obj::read_seeds(
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
            } else if line.trim().is_empty() && target_type_num < 8 {
                let mut new_map_stack_part = ObjMapPart::read_lines(
                    part_stack,
                    OBJ_ORDER[source_type_num],
                    OBJ_ORDER[target_type_num],
                );
                map_stack.append(&mut new_map_stack_part);
                source_type_num += 1;
                target_type_num += 1;
                part_stack = vec![];
            } else {
                part_stack.push(line.to_string());
            }
        }
    }
    if !part_stack.is_empty() {
        let mut new_map_stack_part = ObjMapPart::read_lines(
            part_stack,
            OBJ_ORDER[source_type_num],
            OBJ_ORDER[target_type_num],
        );
        map_stack.append(&mut new_map_stack_part);
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
    fn read_line_test() {
        let a1 = Obj {
            obj_type: ObjType::Seed,
            id: 98,
        };
        let a2 = Obj {
            obj_type: ObjType::Seed,
            id: 99,
        };
        let a3 = Obj {
            obj_type: ObjType::Seed,
            id: 20,
        };
        let b1 = Obj {
            obj_type: ObjType::Soil,
            id: 50,
        };
        let b2 = Obj {
            obj_type: ObjType::Soil,
            id: 51,
        };
        let omp = ObjMapPart::Shift {
            source_start: a1,
            destination_start: b1,
            range_length: 2,
        };
        assert_eq!(
            ObjMapPart::read_line("50 98 2".into(), ObjType::Seed, ObjType::Soil).unwrap(),
            omp
        );
        assert_eq!(omp.apply(a1), b1);
        assert_eq!(omp.apply(a2), b2);
        assert_eq!(omp.apply(a3), a3);
    }
}
