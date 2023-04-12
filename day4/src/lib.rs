use common::lines_from_file;
use lazy_static::lazy_static;
use regex::Regex;
use std::borrow::Borrow;

pub fn run(filename: &str) -> Result<(), &'static str> {
    let solution1 = solve(lines_from_file(&filename)?, p1_solver)?;
    let solution2 = solve(lines_from_file(&filename)?, p2_solver)?;
    println!("Solution 1: {solution1}");
    println!("Solution 2: {solution2}");
    Ok(())
}

struct Line {
    left: Range,
    right: Range,
}

struct Range {
    start: i16,
    end: i16,
}

lazy_static! {
    static ref RE: Regex = Regex::new(r"^(\d+)-(\d+),(\d+)-(\d+)$").unwrap();
}

impl Line {
    fn parse(line: &str) -> Result<Self, &'static str> {
        let a = RE.captures(line).ok_or("Line doesn't match pattern")?;

        Ok(Line {
            left: Range {
                start: expect_number(&a[1])?,
                end: expect_number(&a[2])?,
            },
            right: Range {
                start: expect_number(&a[3])?,
                end: expect_number(&a[4])?,
            },
        })
    }
}

fn expect_number(captured: &str) -> Result<i16, &'static str> {
    captured.parse().or(Err("Can't parse item."))
}

fn p1_solver(Line { left, right }: &Line) -> bool {
    let left_contains_right = left.start <= right.start && left.end >= right.end;
    let right_contains_left = left.start >= right.start && left.end <= right.end;
    left_contains_right || right_contains_left
}

fn p2_solver(Line { left, right }: &Line) -> bool {
    // it does not collide if right end is before left start, or right start is after left end
    // !(right.end < left.start || right.start > left.end)
    right.end >= left.start && right.start <= left.end
}

fn solve(
    mut lines: impl Iterator<Item = impl Borrow<str>>,
    solver: fn(&Line) -> bool,
) -> Result<usize, &'static str> {
    lines.try_fold(0, |acc, line| {
        let parsed = Line::parse(line.borrow())?;
        let is_true = solver(&parsed);
        Ok(if is_true { acc + 1 } else { acc })
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn solution_p1() {
        let solution = solve(INPUT.lines(), p1_solver).unwrap();
        assert!(solution == 2);
    }

    #[test]
    fn contains_same() {
        let line = Line::parse("2-3,2-3").unwrap();
        let result = p1_solver(&line);
        assert!(result == true);
    }

    #[test]
    fn solution_p2() {
        let solution = solve(INPUT.lines(), p2_solver).unwrap();
        assert!(solution == 4);
    }
}
