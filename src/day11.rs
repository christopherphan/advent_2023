/* Solution to 2023 Advent of Code, Day 11
 *
 * Christopher Phan
 */

use std::error::Error;
use std::num::NonZeroUsize;

use crate::common;
use crate::common::AdventError;

const DAY: usize = 11;
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

pub fn part_1(input: Vec<String>) -> Result<usize, Box<dyn Error>> {
    Ok(distance_after_expansion(
        input,
        NonZeroUsize::new(2).unwrap(),
    ))
}

pub fn part_2(input: Vec<String>) -> Result<usize, Box<dyn Error>> {
    Ok(distance_after_expansion(
        input,
        NonZeroUsize::new(1_000_000).unwrap(),
    ))
}

fn distance_after_expansion(input: Vec<String>, factor: NonZeroUsize) -> usize {
    GalaxyData::from(input).expand(factor).total_distances()
}

/* The distance function being used in this problem is the Manhattan distance (a.k.a. taxi-cab
 * metric) */
fn manhattan_distance(loc1: (usize, usize), loc2: (usize, usize)) -> usize {
    loc1.0.abs_diff(loc2.0) + loc1.1.abs_diff(loc2.1)
}

#[derive(Clone, Debug)]
struct GalaxyData {
    width: usize,
    height: usize,
    galaxies: Vec<(usize, usize)>, // galaxy positions will be stored as (col, row)
}

impl From<Vec<String>> for GalaxyData {
    fn from(input: Vec<String>) -> Self {
        let height = input.len();
        let mut width: usize = 0;
        let mut galaxies: Vec<(usize, usize)> = vec![];
        for (row, line) in input.iter().enumerate() {
            if line.len() > width {
                width = line.len();
            }
            for (col, c) in line.chars().enumerate() {
                if c == '#' {
                    galaxies.push((col, row));
                }
            }
        }
        Self {
            width,
            height,
            galaxies,
        }
    }
}

impl GalaxyData {
    fn empty_rows(&self) -> Vec<usize> {
        (0..(self.height))
            .filter(|k| self.galaxies.iter().all(|(_, y)| y != k))
            .collect()
    }

    fn empty_cols(&self) -> Vec<usize> {
        (0..(self.width))
            .filter(|k| self.galaxies.iter().all(|(x, _)| x != k))
            .collect()
    }

    fn expand(&self, factor: NonZeroUsize) -> Self {
        let empty_rows = self.empty_rows();
        let empty_cols = self.empty_cols();
        let expansion_add: usize = factor.get() - 1;
        let width = self.width + expansion_add * empty_cols.len();
        let height = self.height + expansion_add * empty_rows.len();
        let mut galaxies: Vec<(usize, usize)> = vec![];
        for (x, y) in self.galaxies.iter() {
            let new_x: usize = x + expansion_add * empty_cols.iter().filter(|k| *k < x).count();
            let new_y: usize = y + expansion_add * empty_rows.iter().filter(|k| *k < y).count();
            galaxies.push((new_x, new_y));
        }
        Self {
            width,
            height,
            galaxies,
        }
    }

    fn galaxy_pairs(&self) -> Vec<((usize, usize), (usize, usize))> {
        let mut out_vec: Vec<((usize, usize), (usize, usize))> = vec![];
        if self.galaxies.len() > 1 {
            for j in 0..(self.galaxies.len() - 1) {
                for k in (j + 1)..(self.galaxies.len()) {
                    out_vec.push((self.galaxies[j], self.galaxies[k]));
                }
            }
        }
        out_vec
    }

    fn total_distances(&self) -> usize {
        self.galaxy_pairs()
            .iter()
            .map(|(u, v)| manhattan_distance(*u, *v))
            .sum()
    }
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

    const EXAMPLE_EXPANDED: &str = "....#........
.........#...
#............
.............
.............
........#....
.#...........
............#
.............
.............
.........#...
#....#.......";

    fn get_example_input() -> Vec<String> {
        common::split_string(EXAMPLE_INPUT.to_string())
    }

    #[test]
    fn galaxy_parsing_test() {
        let gal_data = GalaxyData::from(get_example_input());
        assert_eq!(gal_data.height, 10);
        assert_eq!(gal_data.width, 10);
        assert_eq!(gal_data.galaxies.len(), 9);
        assert!(gal_data.galaxies.contains(&(3, 0)));
        assert!(gal_data.galaxies.contains(&(7, 1)));
        assert!(gal_data.galaxies.contains(&(0, 2)));
        assert!(gal_data.galaxies.contains(&(6, 4)));
        assert!(gal_data.galaxies.contains(&(1, 5)));
        assert!(gal_data.galaxies.contains(&(9, 6)));
        assert!(gal_data.galaxies.contains(&(7, 8)));
        assert!(gal_data.galaxies.contains(&(0, 9)));
        assert!(gal_data.galaxies.contains(&(4, 9)));
    }

    #[test]
    fn empty_rows_test() {
        let gal_data = GalaxyData::from(get_example_input());
        let empty_rows = gal_data.empty_rows();
        assert_eq!(empty_rows.len(), 2);
        assert!(empty_rows.contains(&3));
        assert!(empty_rows.contains(&7));
    }

    #[test]
    fn empty_cols_test() {
        let gal_data = GalaxyData::from(get_example_input());
        let empty_cols = gal_data.empty_cols();
        assert_eq!(empty_cols.len(), 3);
        assert!(empty_cols.contains(&2));
        assert!(empty_cols.contains(&5));
        assert!(empty_cols.contains(&8));
    }

    #[test]
    fn galaxy_expansion_test() {
        let original = GalaxyData::from(get_example_input());
        let expanded_goal = GalaxyData::from(common::split_string(EXAMPLE_EXPANDED.to_string()));
        let expanded = original.expand(NonZeroUsize::new(2).unwrap());
        assert_eq!(expanded_goal.width, expanded.width);
        assert_eq!(expanded_goal.height, expanded.height);
        assert_eq!(expanded_goal.galaxies.len(), expanded.galaxies.len());
        for loc in expanded_goal.galaxies.iter() {
            assert!(expanded.galaxies.contains(loc));
        }
    }

    #[test]
    fn galaxy_pairs_test() {
        let gal_data = GalaxyData::from(get_example_input());
        assert_eq!(gal_data.galaxy_pairs().len(), 36);
    }

    #[test]
    fn part1_test() {
        assert_eq!(part_1(get_example_input()).unwrap(), 374);
    }

    #[test]
    fn part2_test() {
        if PART_2_IMPL {
            assert_eq!(
                distance_after_expansion(get_example_input(), NonZeroUsize::new(10).unwrap()),
                1030
            );
        }
    }
}
