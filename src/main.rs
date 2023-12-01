/* src/main.rs
 *
 * Solutions to 2023 Advent of Code
 *
 * Christopher Phan
 */

use cphan_advent_2023::common;
use cphan_advent_2023::day1;

fn main() {
    let day1_input = common::get_day(1).unwrap();
    println!(
        "Day 1, part 1 solution: {}",
        day1::part_1(day1_input.clone()).unwrap()
    );
    println!(
        "Day 1, part 2 solution: {}",
        day1::part_2(day1_input).unwrap()
    );
}
