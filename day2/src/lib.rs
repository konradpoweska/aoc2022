mod common;
mod p1;
mod p2;

use self::common::{parse_lines, solve};
use p1::RoundPart1;
use p2::RoundPart2;
use ::common::lines_from_stdin;

pub fn run() -> Result<(), &'static str> {
    let letters = parse_lines(lines_from_stdin())?;

    let solution1 = solve(&letters, RoundPart1::parse)?;
    println!("Part 1 solution: {}", solution1);

    let solution2 = solve(&letters, RoundPart2::parse)?;
    println!("Part 2 solution: {}", solution2);

    Ok(())
}
