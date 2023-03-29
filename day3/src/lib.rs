use std::borrow::Borrow;

use common::lines_from_stdin;

pub fn run() -> Result<(), &'static str> {
    let solution = solve_p1(lines_from_stdin())?;
    println!("Part 1 solution: {solution}");
    Ok(())
}

#[derive(Debug)]
struct Rucksack(Compartment, Compartment);

impl Rucksack {
    fn parse(line: &str) -> Result<Self, &'static str> {
        let (left, right) = line.split_at(line.len() / 2);
        Ok(Rucksack(
            Compartment::parse(left)?,
            Compartment::parse(right)?,
        ))
    }
    fn shared_item_priority(&self) -> u32 {
        Compartment::shared_item_priority(&self.0, &self.1)
    }
}

#[derive(Clone, Copy, Debug)]
struct Compartment(u64);

impl Compartment {
    fn parse(line: &str) -> Result<Self, &'static str> {
        let mut compartment = Compartment(0);
        for char in line.chars() {
            compartment.add(char)?
        }
        Ok(compartment)
    }
    fn add(&mut self, item: char) -> Result<(), &'static str> {
        let priority = priority_of_item(item).ok_or("Couldn't parse item")?;
        self.0 |= 1u64 << priority;
        Ok(())
    }
    fn shared_item_priority(left: &Self, right: &Self) -> u32 {
        // returns only the smallest priority
        (left.0 & right.0).trailing_zeros()
    }
}

fn priority_of_item(char: char) -> Option<u64> {
    let item = char as u64;
    if item >= 65 && item <= 90 {
        return Some(item - 38);
    }
    if item >= 97 && item <= 122 {
        return Some(item - 96);
    }
    None
}

fn solve_p1<I>(lines: I) -> Result<u32, &'static str>
where
    I: IntoIterator,
    I::Item: Borrow<str>,
{
    let rucksacks = lines
        .into_iter()
        .map(|line| Rucksack::parse(line.borrow()))
        .collect::<Result<Vec<_>, _>>()?;

    let sum = rucksacks.iter().map(Rucksack::shared_item_priority).sum();
    Ok(sum)
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
        assert!(priority_of_item('a') == Some(1));
        assert!(priority_of_item('b') == Some(2));
        assert!(priority_of_item('z') == Some(26));
        assert!(priority_of_item('A') == Some(27));
        assert!(priority_of_item('Z') == Some(52));
    }

    #[test]
    fn solution_p1() {
        let solution = solve_p1(INPUT.lines()).unwrap();
        assert!(solution == 157);
    }
}
