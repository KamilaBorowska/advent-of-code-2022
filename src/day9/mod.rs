use crate::Solution;
use std::{collections::HashSet, error::Error};

struct Simulation<const N: usize> {
    visited_positions: HashSet<(i32, i32)>,
    positions: [(i32, i32); N],
}

impl<const N: usize> Simulation<N> {
    fn new() -> Self {
        Self {
            visited_positions: HashSet::from_iter([(0, 0)]),
            positions: [(0, 0); N],
        }
    }

    fn move_by(&mut self, offset: (i32, i32)) {
        let (mut previous_position, positions) = self.positions.split_first_mut().unwrap();
        previous_position.0 += offset.0;
        previous_position.1 += offset.1;
        for (i, position) in positions.iter_mut().enumerate() {
            if previous_position.0.abs_diff(position.0) > 1
                || previous_position.1.abs_diff(position.1) > 1
            {
                position.0 += previous_position.0.cmp(&position.0) as i32;
                position.1 += previous_position.1.cmp(&position.1) as i32;
                previous_position = position;
                if i == N - 2 {
                    self.visited_positions.insert(*previous_position);
                }
            } else {
                break;
            }
        }
    }
}

fn solve<const N: usize>(input: &str) -> Result<String, Box<dyn Error>> {
    let mut simulation = Simulation::<N>::new();
    for line in input.lines() {
        let (direction, count) = line.split_once(' ').ok_or("Expected a valid line")?;
        let offset = match direction {
            "L" => (-1, 0),
            "R" => (1, 0),
            "U" => (0, -1),
            "D" => (0, 1),
            _ => return Err(format!("Unrecognized direction {direction}").into()),
        };
        for _ in 0u32..count.parse()? {
            simulation.move_by(offset);
        }
    }
    Ok(simulation.visited_positions.len().to_string())
}

pub(super) const DAY9: Solution = Solution {
    part1: solve::<2>,
    part2: solve::<10>,
};

#[cfg(test)]
mod test {
    use crate::{lines, test};
    const EXAMPLE: &str = lines!(
        "R 4"
        "U 4"
        "L 3"
        "D 1"
        "R 4"
        "D 1"
        "L 5"
        "R 2"
    );
    test!(
        DAY9.part1,
        example: EXAMPLE => 13,
        input: 6256,
    );
    test!(
        DAY9.part2,
        example1: EXAMPLE => 1,
        example2: lines!(
            "R 5"
            "U 8"
            "L 8"
            "D 3"
            "R 17"
            "D 10"
            "L 25"
            "U 20"
        ) => 36,
        input: 2665,
    );
}
