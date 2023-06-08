use crate::{pair::Pair, Forest};

pub fn solve(forest: &Forest<u8>) -> usize {
    let trees_visibility = get_trees_visibility(forest);
    count_visible(&trees_visibility)
}

fn get_trees_visibility(trees_height: &Forest<u8>) -> Forest<bool> {
    let size = trees_height.size;
    let mut trees_visibility = Forest::<bool> {
        buffer: vec![false; (size.x * size.y) as usize],
        size,
    };

    // Looking from left
    for y in 0..size.y {
        let mut highest = None;
        for x in 0..size.x {
            update_visibility(trees_height, x, y, &mut highest, &mut trees_visibility);
        }
    }
    // Looking from right
    for y in 0..size.y {
        let mut highest = None;
        for x in (0..size.x).rev() {
            update_visibility(trees_height, x, y, &mut highest, &mut trees_visibility);
        }
    }
    // Looking from top
    for x in 0..size.x {
        let mut highest = None;
        for y in 0..size.y {
            update_visibility(trees_height, x, y, &mut highest, &mut trees_visibility);
        }
    }
    // Looking from bottom
    for x in 0..size.x {
        let mut highest = None;
        for y in (0..size.y).rev() {
            update_visibility(trees_height, x, y, &mut highest, &mut trees_visibility);
        }
    }
    trees_visibility
}

fn update_visibility(
    trees_height: &Forest<u8>,
    x: i32,
    y: i32,
    highest: &mut Option<u8>,
    trees_visibility: &mut Forest<bool>,
) {
    let current = trees_height.get(Pair { x, y });
    if match *highest {
        None => true,
        Some(highest) if highest < current => true,
        _ => false,
    } {
        trees_visibility.set(Pair { x, y }, true);
        *highest = Some(current);
    }
}

fn count_visible(trees_visibility: &Forest<bool>) -> usize {
    trees_visibility
        .buffer
        .iter()
        .copied()
        .filter(|&v| v)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::INPUT;

    #[test]
    fn solution() {
        let forest = Forest::parse(INPUT.lines()).unwrap();
        let solution = solve(&forest);
        assert_eq!(solution, 21);
    }
}
