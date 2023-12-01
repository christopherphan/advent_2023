/* src/all_days.rs */

use crate::day1;

pub fn run_day(d: usize) {
    match d {
        1 => {
            day1::run();
        }
        k => {
            println!("Solution for day {} not found.", k);
        }
    }
}
