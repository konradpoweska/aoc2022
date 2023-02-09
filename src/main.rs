use aoc2022::day1;
use std::env;

fn main() -> Result<(), &'static str> {
    match env::args().nth(1).as_deref() {
        Some("1") => day1::run(),
        _ => {
            return Err("Please specify the day");
        }
    }
}
