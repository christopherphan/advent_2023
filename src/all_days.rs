/* src/all_days.rs */

use crate::day01;
use crate::day02;
use crate::day03;
use crate::day04;
use crate::day05;
use crate::day06;
use crate::day07;
use crate::day08;
use crate::day09;
use crate::day10;
use crate::day11;

pub fn run_day(d: usize) {
    match d {
        1 => {
            day01::run();
        }
        2 => {
            day02::run();
        }
        3 => {
            day03::run();
        }
        4 => {
            day04::run();
        }
        5 => {
            day05::run();
        }
        6 => {
            day06::run();
        }
        7 => {
            day07::run();
        }
        8 => {
            day08::run();
        }
        9 => {
            day09::run();
        }
        10 => {
            day10::run();
        }
        11 => {
            day11::run();
        }
        k => {
            println!("Solution for day {} not found.", k);
        }
    }
}
