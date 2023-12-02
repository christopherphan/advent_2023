/* src/all_days.rs */

use crate::day1;
use crate::day2;

pub fn run_day(d: usize) {
    match d {
        1 => {
            day1::run();
        }
        2 => {
            day2::run();
        }
        k => {
            println!("Solution for day {} not found.", k);
        }
    }
}
