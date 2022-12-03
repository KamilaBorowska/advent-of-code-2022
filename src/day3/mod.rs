use crate::Solution;
use std::{collections::HashSet, iter};

fn priority(item: u8) -> Result<u32, &'static str> {
    match item {
        b'a'..=b'z' => Ok(u32::from(item - b'a') + 1),
        b'A'..=b'Z' => Ok(u32::from(item - b'A') + 27),
        _ => Err("Expected an ASCII alphebetic character"),
    }
}

pub(super) const DAY3: Solution = Solution {
    part1: |input| {
        input
            .lines()
            .map(|line| {
                let (a, b) = line.as_bytes().split_at(line.len() / 2);
                let first_compartment: HashSet<u8> = a.iter().copied().collect();
                b.iter()
                    .find(|item| first_compartment.contains(item))
                    .ok_or("Expected compartments to have duplicate elements")
                    .and_then(|&item| priority(item))
            })
            .try_fold(0, |acc, priority| Ok(acc + priority?))
            .map(|sum| sum.to_string())
    },
    part2: |input| {
        let mut lines = input.lines();
        iter::from_fn(|| match (lines.next(), lines.next(), lines.next()) {
            (Some(a), Some(b), Some(c)) => Some([a, b, c]),
            _ => None,
        })
        .map(|rucksacks| {
            let possible_choices: HashSet<u8> = rucksacks
                .iter()
                .map(|rucksack| rucksack.bytes().collect())
                .reduce(|mut a: HashSet<u8>, b| {
                    a.retain(|elem| b.contains(elem));
                    a
                })
                .expect("Non-empty iterator");
            if possible_choices.len() != 1 {
                Err("Incorrect number of choices")
            } else {
                priority(*possible_choices.iter().next().unwrap())
            }
        })
        .try_fold(0, |acc, priority| Ok(acc + priority?))
        .map(|sum| sum.to_string())
    },
};

#[cfg(test)]
mod test {
    use crate::{lines, test};
    const EXAMPLE: &str = lines!(
        "vJrwpWtwJgWrhcsFMMfFFhFp"
        "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL"
        "PmmdzqPrVvPwwTWBwg"
        "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn"
        "ttgJtRGJQctTZtZT"
        "CrZsJsPPZsGzwwsLwLmpwMDw"
    );
    test!(
        DAY3.part1,
        example: EXAMPLE => 157,
        input: 7766,
    );
    test!(
        DAY3.part2,
        example: EXAMPLE => 70,
        input: 2415,
    );
}
