use common::lines_from_stdin;
use day1::*;

fn main() -> Result<(), &'static str> {
    let elves = parse_elves(lines_from_stdin());
    let result_part1 = solve_part1(&elves)?;
    let result_part2 = solve_part2(&elves);
    println!("Part 1: {}", result_part1);
    println!("Part 2: {}", result_part2);
    Ok(())
}
