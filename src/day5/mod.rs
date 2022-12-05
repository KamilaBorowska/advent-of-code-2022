use crate::Solution;
use std::{collections::VecDeque, error::Error};

fn parse_stacks<'a>(
    lines: &mut impl Iterator<Item = &'a str>,
) -> Result<Vec<VecDeque<u8>>, &'static str> {
    let mut stacks = Vec::new();
    for line in &mut *lines {
        if line.contains('[') {
            let line = line.trim_end();
            stacks.resize_with((line.len() + 1) / 4, VecDeque::new);
            for (chunk, stack) in line.as_bytes().chunks(4).zip(&mut stacks) {
                let c = *chunk.get(1).ok_or("Invalid chunk")?;
                if c != b' ' {
                    stack.push_front(c);
                }
            }
        } else {
            break;
        }
    }
    if lines.next() != Some("") {
        return Err("Expected a blank line");
    }
    Ok(stacks)
}

fn parse_line(line: &str) -> Result<(usize, usize, usize), Box<dyn Error>> {
    let line = line
        .strip_prefix("move ")
        .ok_or("Expected line to start with move")?;
    let (count, rest) = line
        .split_once(" from ")
        .ok_or("Expected line to have count")?;
    let (from, to) = rest
        .split_once(" to ")
        .ok_or("Expected line to have a rearrangment")?;
    Ok((count.parse()?, from.parse()?, to.parse()?))
}

fn get_stack_top(stacks: &[VecDeque<u8>]) -> Result<String, Box<dyn Error>> {
    stacks
        .iter()
        .map(|stack| Ok(char::from(*stack.back().ok_or("Missing stack element")?)))
        .collect()
}

pub(super) const DAY5: Solution = Solution {
    part1: |input| {
        let mut lines = input.lines();
        let mut stacks = parse_stacks(&mut lines)?;
        for line in lines {
            let (count, from, to) = parse_line(line)?;
            for _ in 0..count {
                let elem = stacks[from - 1]
                    .pop_back()
                    .ok_or_else(|| format!("Expected an element in {from}"))?;
                stacks[to - 1].push_back(elem);
            }
        }
        get_stack_top(&stacks)
    },
    part2: |input| {
        let mut lines = input.lines();
        let mut stacks = parse_stacks(&mut lines)?;
        for line in lines {
            let (count, from, to) = parse_line(line)?;
            let from = &mut stacks[from - 1];
            let elems: Vec<u8> = from.drain(from.len() - count..).collect();
            stacks[to - 1].extend(elems);
        }
        get_stack_top(&stacks)
    },
};

#[cfg(test)]
mod test {
    use crate::{lines, test};
    const EXAMPLE: &str = lines!(
        "    [D]   "
        "[N] [C]    "
        "[Z] [M] [P]"
        " 1   2   3 "
        ""
        "move 1 from 2 to 1"
        "move 3 from 1 to 3"
        "move 2 from 2 to 1"
        "move 1 from 1 to 2"
    );
    test!(
        DAY5.part1,
        example: EXAMPLE => "CMZ",
        input: "BZLVHBWQF",
    );
    test!(
        DAY5.part2,
        example: EXAMPLE => "MCD",
        input: "TDGJQTZSL",
    );
}
