use crate::Forest;
use common::pair::{CoordsIter, Direction, Pair, DOWN, LEFT, RIGHT, UP};

pub fn solve(forest: &Forest<u8>) -> usize {
    (1..(forest.size.x - 1))
        .flat_map(|x| (1..(forest.size.y - 1)).map(move |y| Pair { x, y }))
        .map(|pair| get_scenic_score(forest, pair))
        .max()
        .expect("Forest should not be empty")
}

fn get_scenic_score(forest: &Forest<u8>, coords: Pair) -> usize {
    [UP, RIGHT, DOWN, LEFT]
        .into_iter()
        .map(|direction| get_viewing_distance(forest, coords, direction))
        .reduce(|l, r| l * r)
        .expect("Scores are from 4 direction")
}

fn get_viewing_distance(forest: &Forest<u8>, coords: Pair, direction: Direction) -> usize {
    let height = forest.get(coords);

    let positions = CoordsIter {
        size: forest.size,
        current: Some(coords),
        direction,
    };

    let mut count = 0;

    for position in positions {
        count += 1;
        if forest.get(position) >= height {
            break;
        }
    }

    count
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::tests::INPUT;

    #[test]
    fn solution() {
        let forest = Forest::parse(INPUT.lines()).unwrap();
        let solution = solve(&forest);
        assert_eq!(solution, 8);
    }
}
