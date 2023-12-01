/* src/main.rs
 *
 * Solutions to 2023 Advent of Code
 *
 * Christopher Phan
 */

use std::env;

use cphan_advent_2023::all_days;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: {} [day_numbers]", args[0]);
    } else {
        for val in args {
            if let Ok(d) = val.parse::<usize>() {
                all_days::run_day(d)
            }
        }
    }
}
