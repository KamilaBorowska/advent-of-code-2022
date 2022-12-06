use crate::Solution;
use std::{collections::HashSet, error::Error};

fn solution<const SIZE: usize>(input: &str) -> Result<String, Box<dyn Error>> {
    let pos = input
        .as_bytes()
        .windows(SIZE)
        .position(|window| window.iter().copied().collect::<HashSet<_>>().len() == SIZE)
        .ok_or("No valid start-of-packet marker found")?;
    Ok((pos + SIZE).to_string())
}

pub(super) const DAY6: Solution = Solution {
    part1: solution::<4>,
    part2: solution::<14>,
};

#[cfg(test)]
mod test {
    use crate::test;
    test!(
        DAY6.part1,
        example1: "mjqjpqmgbljsphdztnvjfqwrcgsmlb" => 7,
        example2: "bvwbjplbgvbhsrlpgdmjqwftvncz" => 5,
        example3: "nppdvjthqldpwncqszvftbrmjlhg" => 6,
        example4: "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg" => 10,
        example5: "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw" => 11,
        input: 1965,
    );
    test!(
        DAY6.part2,
        example1: "mjqjpqmgbljsphdztnvjfqwrcgsmlb" => 19,
        example2: "bvwbjplbgvbhsrlpgdmjqwftvncz" => 23,
        example3: "nppdvjthqldpwncqszvftbrmjlhg" => 23,
        example4: "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg" => 29,
        example5: "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw" => 26,
        input: 2773,
    );
}
