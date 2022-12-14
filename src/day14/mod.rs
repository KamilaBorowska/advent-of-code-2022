use std::{collections::HashSet, error::Error};

use crate::Solution;

fn parse_input(input: &str) -> Result<HashSet<(i32, i32)>, Box<dyn Error>> {
    let mut grid = HashSet::new();
    for line in input.lines() {
        let mut path = line
            .split(" -> ")
            .map(|point| -> Result<_, Box<dyn Error>> {
                let (x, y) = point.split_once(',').ok_or("Unable to parse a point")?;
                Ok((x.parse()?, y.parse()?))
            });
        let mut previous = path.next().expect("First point")?;
        for point in path {
            let point = point?;
            let mut x_range = [previous.0, point.0];
            x_range.sort_unstable();
            let mut y_range = [previous.1, point.1];
            y_range.sort_unstable();
            for x in x_range[0]..=x_range[1] {
                for y in y_range[0]..=y_range[1] {
                    grid.insert((x, y));
                }
            }
            previous = point;
        }
    }
    Ok(grid)
}

fn get_max_y(grid: &HashSet<(i32, i32)>) -> Result<i32, &'static str> {
    grid.iter()
        .map(|point| point.1)
        .max()
        .ok_or("No points parsed")
}

fn run_part2_simulation(
    grid: &mut HashSet<(i32, i32)>,
    position @ (x, y): (i32, i32),
    max_y: i32,
) -> u32 {
    let mut sum = 0;
    for offset in [0, -1, 1] {
        let new_position = (x + offset, y + 1);
        if !grid.contains(&new_position) && y != max_y + 1 {
            sum += run_part2_simulation(grid, new_position, max_y);
        }
    }
    grid.insert(position);
    sum + 1
}

pub(super) const DAY14: Solution = Solution {
    part1: |input| {
        let mut grid = parse_input(input)?;
        let max_y = get_max_y(&grid)?;
        'counting: for count in 0.. {
            let mut sand_x = 500;
            'falling: for y in 0..=max_y {
                for offset in [0, -1, 1] {
                    if !grid.contains(&(sand_x + offset, y + 1)) {
                        sand_x += offset;
                        continue 'falling;
                    }
                }
                grid.insert((sand_x, y));
                continue 'counting;
            }
            return Ok(count.to_string());
        }
        unreachable!()
    },
    part2: |input| {
        let mut grid = parse_input(input)?;
        let max_y = get_max_y(&grid)?;
        Ok(run_part2_simulation(&mut grid, (500, 0), max_y).to_string())
    },
};

#[cfg(test)]
mod test {
    use crate::{lines, test};
    const EXAMPLE: &str = lines!(
        "498,4 -> 498,6 -> 496,6"
        "503,4 -> 502,4 -> 502,9 -> 494,9"
    );
    test!(
        DAY14.part1,
        example: EXAMPLE => 24,
        input: 672,
    );
    test!(
        DAY14.part2,
        example: EXAMPLE => 93,
        input: 26_831,
    );
}
