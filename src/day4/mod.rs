use crate::Solution;
use std::error::Error;

fn parse_line(line: &str) -> Result<(u8, u8, u8, u8), Box<dyn Error>> {
    let (a, line) = line.split_once('-').ok_or("Missing first dash")?;
    let (b, line) = line.split_once(',').ok_or("Missing first comma")?;
    let (c, d) = line.split_once('-').ok_or("Missing second dash")?;
    Ok((a.parse()?, b.parse()?, c.parse()?, d.parse()?))
}

pub(super) const DAY4: Solution = Solution {
    part1: |input| {
        let mut count = 0;
        for line in input.lines() {
            let (a, b, c, d) = parse_line(line)?;
            if (a <= c && b >= d) || (c <= a && d >= b) {
                count += 1;
            }
        }
        Ok(count.to_string())
    },
    part2: |input| {
        let mut count = 0;
        for line in input.lines() {
            let (a, b, c, d) = parse_line(line)?;
            if (a <= c && b >= c) || (c <= a && d >= a) {
                count += 1;
            }
        }
        Ok(count.to_string())
    },
};

#[cfg(test)]
mod test {
    use crate::{lines, test};
    const EXAMPLE: &str = lines!(
        "2-4,6-8"
        "2-3,4-5"
        "5-7,7-9"
        "2-8,3-7"
        "6-6,4-6"
        "2-6,4-8"
    );
    test!(
        DAY4.part1,
        example: EXAMPLE => 2,
        input: 528,
    );
    test!(
        DAY4.part2,
        example: EXAMPLE => 4,
        input: 881,
    );
}
