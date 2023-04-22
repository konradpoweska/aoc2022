use std::fs::read_to_string;

type Error = &'static str;

pub fn run(filename: &str) -> Result<(), Error> {
    let input = read_to_string(filename).or(Err("Couldn't open file"))?;
    println!("Solution P1: {}", solve_p1(&input)?);
    println!("Solution P2: {}", solve_p2(&input)?);
    Ok(())
}

fn solve_p1(input: &str) -> Result<usize, Error> {
    solve::<4>(input)
}

fn solve_p2(input: &str) -> Result<usize, Error> {
    solve::<14>(input)
}

fn solve<const LENGTH: usize>(input: &str) -> Result<usize, Error> {
    let bytes = input.as_bytes();
    let mut idx = 0;
    'str_iter: while idx < input.len() - LENGTH {
        for i in 0..LENGTH {
            for j in i + 1..LENGTH {
                if bytes[idx + i] == bytes[idx + j] {
                    idx += i + 1;
                    continue 'str_iter;
                }
            }
        }
        return Ok(idx + LENGTH);
    }
    Err("No solution has been found")
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("bvwbjplbgvbhsrlpgdmjqwftvncz", 5)]
    #[test_case("nppdvjthqldpwncqszvftbrmjlhg", 6)]
    #[test_case("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10)]
    #[test_case("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11)]
    fn test_p1(input: &str, expected: usize) {
        let result = solve_p1(input).unwrap();
        assert_eq!(result, expected);
    }

    #[test_case("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 19)]
    #[test_case("bvwbjplbgvbhsrlpgdmjqwftvncz", 23)]
    #[test_case("nppdvjthqldpwncqszvftbrmjlhg", 23)]
    #[test_case("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 29)]
    #[test_case("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 26)]
    fn test_p2(input: &str, expected: usize) {
        let result = solve_p2(input).unwrap();
        assert_eq!(result, expected);
    }
}
