#![feature(iter_array_chunks)]
use std::{
    borrow::Borrow,
    fs::File,
    io::{BufRead, BufReader},
};

pub fn run(filename: &str) -> Result<(), &'static str> {
    let solution_p1 = solve_p1(lines_from_file(&filename)?)?;
    let solution_p2 = solve_p2(lines_from_file(&filename)?)?;

    println!("Part 1 solution: {}", solution_p1);
    println!("Part 2 solution: {}", solution_p2);

    Ok(())
}

fn lines_from_file(filename: &str) -> Result<impl Iterator<Item = String>, &'static str> {
    let file = File::open(&filename).or(Err("Couldn't open file."))?;
    Ok(BufReader::new(file).lines().map(Result::unwrap))
}

fn build_base(definition: &&str) -> Result<u64, &'static str> {
    let mut result: u64 = 0;
    for char in definition.chars() {
        let priority = priority_of_item(char)?;
        result |= 1u64 << priority;
    }
    return Ok(result);
}

fn priority_of_item(char: char) -> Result<u32, &'static str> {
    let item = char as u32;
    if item >= 65 && item <= 90 {
        return Ok(item - 38);
    }
    if item >= 97 && item <= 122 {
        return Ok(item - 96);
    }
    Err("Couldn't parse item")
}

fn solve_p1(lines: impl Iterator<Item = impl Borrow<str>>) -> Result<u32, &'static str> {
    Ok(lines
        .map(|line| {
            let line = line.borrow();
            let (left, right) = line.split_at(line.len() / 2);
            get_duplicated_item_priority([left, right])
        })
        .collect::<Result<Vec<_>, _>>()?
        .iter()
        .sum())
}

fn solve_p2(lines: impl Iterator<Item = impl Borrow<str>>) -> Result<u32, &'static str> {
    Ok(lines
        .array_chunks::<3>()
        .map(|[a, b, c]| get_duplicated_item_priority([a.borrow(), b.borrow(), c.borrow()]))
        .collect::<Result<Vec<_>, _>>()?
        .iter()
        .sum())
}

fn get_duplicated_item_priority<const T: usize>(items: [&str; T]) -> Result<u32, &'static str> {
    let mut iter = items.iter();
    let first = iter.next().ok_or("Not enough items")?;

    let a = iter
        .map(build_base)
        .reduce(|a, b| Ok(a? & b?))
        .ok_or("Not enough items")??;

    for char in first.chars() {
        let priority = priority_of_item(char)?;
        if (a & 1u64 << priority) != 0 {
            return Ok(priority);
        }
    }

    Err("Could not find any common item")
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn priorities_parsing() {
        assert!(priority_of_item('a') == Ok(1));
        assert!(priority_of_item('b') == Ok(2));
        assert!(priority_of_item('z') == Ok(26));
        assert!(priority_of_item('A') == Ok(27));
        assert!(priority_of_item('Z') == Ok(52));
    }

    #[test]
    fn solution_p1() {
        let solution = solve_p1(INPUT.lines().into_iter()).unwrap();
        assert!(solution == 157);
    }

    #[test]
    fn solution_p2() {
        let solution = solve_p2(INPUT.lines().into_iter()).unwrap();
        assert!(solution == 70);
    }
}
