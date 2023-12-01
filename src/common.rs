/* src/common.rs
 *
 * Christopher Phan
 * */

use std::error::Error;
use std::fmt;
use std::fs;

pub fn split_string(raw: String) -> Vec<String> {
    raw.split('\n').map(|k| k.to_string()).collect()
}

pub fn get_day(day_number: usize) -> Result<Vec<String>, Box<dyn Error>> {
    let filename = &format!("puzzle_inputs/day{:02}.txt", day_number);

    let raw_info = fs::read_to_string(filename)?;
    Ok(split_string(raw_info))
}

#[derive(Debug)]
pub struct AdventError(pub String);

impl fmt::Display for AdventError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for AdventError {}
