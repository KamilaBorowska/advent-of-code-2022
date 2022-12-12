use crate::{grid::Grid, Solution};
use std::{collections::HashSet, error::Error, mem};

fn run_search(
    mut open_set: HashSet<(usize, usize)>,
    grid: &Grid<'_>,
) -> Result<String, Box<dyn Error>> {
    let mut checked = HashSet::new();
    for cost in 0.. {
        for position @ (x, y) in mem::take(&mut open_set) {
            let mut height = grid.at(x, y).unwrap();
            if height == b'E' {
                return Ok(cost.to_string());
            }
            if height == b'S' {
                height = b'a';
            }
            if checked.insert(position) {
                for (offset_x, offset_y) in [(usize::MAX, 0), (1, 0), (0, usize::MAX), (0, 1)] {
                    let new_position @ (x, y) =
                        (x.wrapping_add(offset_x), y.wrapping_add(offset_y));
                    if let Some(mut c) = grid.at(x, y) {
                        if c == b'E' {
                            c = b'z';
                        }
                        if height + 1 >= c {
                            open_set.insert(new_position);
                        }
                    }
                }
            }
        }
        if open_set.is_empty() {
            break;
        }
    }
    return Err("Unable to reach best signal".into());
}

pub(super) const DAY12: Solution = Solution {
    part1: |input| {
        let grid = Grid::parse(input)?;
        let start = grid.find(b'S').ok_or("Unable to find start point")?;
        let open_set = HashSet::from_iter([start]);
        run_search(open_set, &grid)
    },
    part2: |input| {
        let grid = Grid::parse(input)?;
        let open_set = (0..grid.width())
            .flat_map(|x| (0..grid.height()).map(move |y| (x, y)))
            .filter(|&(x, y)| b"aS".contains(&grid.at(x, y).unwrap()))
            .collect();
        run_search(open_set, &grid)
    },
};

#[cfg(test)]
mod test {
    use crate::{lines, test};
    const EXAMPLE: &str = lines!(
        "Sabqponm"
        "abcryxxl"
        "accszExk"
        "acctuvwj"
        "abdefghi"
    );
    test!(
        DAY12.part1,
        example: EXAMPLE => 31,
        input: 361,
    );
    test!(
        DAY12.part2,
        example: EXAMPLE => 29,
        input: 354,
    );
}
