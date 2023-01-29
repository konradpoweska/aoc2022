use std::borrow::Borrow;

pub fn run() {
    let elves = parse_elves(lines_from_stdin());
    let result_part1 = solve_part1(&elves);
    let result_part2 = solve_part2(&elves);
    println!("Part 1: {}", result_part1);
    println!("Part 2: {}", result_part2);
}

fn parse_elves<I>(lines: I) -> Vec<u32>
where
    I: IntoIterator,
    I::Item: Borrow<str>,
{
    let mut elves: Vec<u32> = vec![];
    let mut accumulator: u32 = 0;

    for line in lines {
        match line.borrow().parse::<u32>() {
            Ok(number) => accumulator += number,
            Err(_) => {
                elves.push(accumulator);
                accumulator = 0
            }
        }
    }

    if accumulator != 0 {
        elves.push(accumulator)
    }

    elves
}

fn solve_part1(elves: &Vec<u32>) -> u32 {
    elves.iter().max().expect("Max couldn't be found").clone()
}

fn solve_part2(elves: &Vec<u32>) -> u32 {
    let mut elves = elves.clone();
    elves.sort_unstable_by(|a, b| b.cmp(a));
    elves[..3].iter().fold(0, |a, b| a + b)
}

fn lines_from_stdin() -> impl Iterator<Item = String> {
    std::io::stdin().lines().map(|e| e.expect("Invalid line"))
}

#[cfg(test)]
pub mod test {
    use super::*;

    const INPUT: &str = "\
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    #[test]
    fn parser() {
        let elves = parse_elves(INPUT.lines());
        assert_eq!(elves, vec![6000, 4000, 11000, 24000, 10000]);
    }

    #[test]
    fn solution_part1() {
        let elves = vec![6000, 4000, 11000, 24000, 10000];
        let result = super::solve_part1(&elves);
        assert_eq!(result, 24000);
    }

    #[test]
    fn solution_part2() {
        let elves = vec![6000, 4000, 11000, 24000, 10000];
        let result = super::solve_part2(&elves);
        assert_eq!(result, 45000);
    }
}
