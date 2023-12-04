/* src/all_days.rs */

use crate::day1;
use crate::day2;
use crate::day3;
use crate::day4;

pub fn run_day(d: usize) {
    match d {
        1 => {
            day1::run();
        }
        2 => {
            day2::run();
        }
        3 => {
            day3::run();
        }
        4 => {
            day4::run();
        }
        k => {
            println!("Solution for day {} not found.", k);
        }
    }
}
