use crate::{grid::Grid, Solution};
use std::collections::HashSet;

fn scan_valid_trees(
    valid_trees: &mut HashSet<(usize, usize)>,
    grid: &Grid<'_>,
    mut positions: impl Iterator<Item = (usize, usize)>,
) {
    let position @ (x, y) = positions.next().expect("non-empty list");
    valid_trees.insert(position);
    let mut current_tree = grid.at(x, y).unwrap();
    for position @ (x, y) in positions {
        let new_tree = grid.at(x, y).unwrap();
        if new_tree > current_tree {
            current_tree = new_tree;
            valid_trees.insert(position);
        }
    }
}

fn scan_score(mut trees: impl Iterator<Item = u8>) -> u32 {
    let first_tree = trees.next().expect("non-empty list");
    let mut score = 0;
    for new_tree in trees {
        score += 1;
        if new_tree >= first_tree {
            break;
        }
    }
    score
}

pub(super) const DAY8: Solution = Solution {
    part1: |input| {
        let grid = Grid::parse(input)?;
        let mut valid_trees = HashSet::new();
        for x in 0..grid.width() {
            scan_valid_trees(&mut valid_trees, &grid, (0..grid.height()).map(|y| (x, y)));
            scan_valid_trees(
                &mut valid_trees,
                &grid,
                (0..grid.height()).rev().map(|y| (x, y)),
            );
        }
        for y in 0..grid.height() {
            scan_valid_trees(&mut valid_trees, &grid, (0..grid.width()).map(|x| (x, y)));
            scan_valid_trees(
                &mut valid_trees,
                &grid,
                (0..grid.width()).rev().map(|x| (x, y)),
            );
        }
        Ok(valid_trees.len().to_string())
    },
    part2: |input| {
        let grid = Grid::parse(input)?;
        let mut max_score = 0;
        for x in 1..grid.width() - 1 {
            for y in 1..grid.height() - 1 {
                let up_score = scan_score((0..=y).rev().map(|y| grid.at(x, y).unwrap()));
                let down_score = scan_score((y..grid.height()).map(|y| grid.at(x, y).unwrap()));
                let left_score = scan_score((0..=x).rev().map(|x| grid.at(x, y).unwrap()));
                let right_score = scan_score((x..grid.width()).map(|x| grid.at(x, y).unwrap()));
                max_score = max_score.max(up_score * down_score * left_score * right_score);
            }
        }
        Ok(max_score.to_string())
    },
};

#[cfg(test)]
mod test {
    use crate::{lines, test};
    const EXAMPLE: &str = lines!(
        "30373"
        "25512"
        "65332"
        "33549"
        "35390"
    );
    test!(
        DAY8.part1,
        example: EXAMPLE => 21,
        input: 1695,
    );
    test!(
        DAY8.part2,
        example: EXAMPLE => 8,
        input: 287040,
    );
}
