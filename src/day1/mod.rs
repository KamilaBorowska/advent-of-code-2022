use crate::Solution;
use std::collections::BinaryHeap;
use std::num::ParseIntError;

fn get_elfs(input: &str) -> impl Iterator<Item = Result<u32, ParseIntError>> + '_ {
    input.split("\n\n").map(|input| {
        input
            .lines()
            .map(str::parse::<u32>)
            .try_fold(0, |acc, line| Ok(acc + line?))
    })
}

pub(super) const DAY1: Solution = Solution {
    part1: |input| {
        Ok(get_elfs(input)
            .try_fold(0, |acc, elf| elf.map(|elf| acc.max(elf)))?
            .to_string())
    },
    part2: |input| {
        let mut heap = get_elfs(input).collect::<Result<BinaryHeap<u32>, _>>()?;
        Ok((0..3).map_while(|_| heap.pop()).sum::<u32>().to_string())
    },
};

#[cfg(test)]
mod test {
    use crate::{lines, test};
    const EXAMPLE: &str = lines!(
        "1000"
        "2000"
        "3000"
        ""
        "4000"
        ""
        "5000"
        "6000"
        ""
        "7000"
        "8000"
        "9000"
        ""
        "10000"
    );
    test!(
        DAY1.part1,
        example: EXAMPLE => 24000,
        input: 68775,
    );
    test!(
        DAY1.part2,
        example: EXAMPLE => 45000,
        input: 202585,
    );
}
