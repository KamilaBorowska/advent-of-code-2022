use crate::Solution;
use std::{collections::VecDeque, error::Error};

struct Monkeys {
    monkeys: Vec<Monkey>,
}

impl Monkeys {
    fn parse(input: &str) -> Result<Self, Box<dyn Error>> {
        let monkeys = input
            .split("\n\n")
            .map(Monkey::parse)
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self { monkeys })
    }

    fn run_round(&mut self, modifier: impl Fn(u64) -> u64) {
        for i in 0..self.monkeys.len() {
            while let Some(item) = self.monkeys[i].items.pop_front() {
                let monkey = &mut self.monkeys[i];
                monkey.inspected_items += 1;
                let item = monkey.operation.execute(item);
                let item = modifier(item);
                let sent_to_monkey = if item % monkey.test == 0 {
                    monkey.if_true
                } else {
                    monkey.if_false
                };
                self.monkeys[sent_to_monkey].items.push_back(item);
            }
        }
    }

    fn monkey_business(&self) -> u64 {
        let mut inspected_items: Vec<_> = self.monkeys.iter().map(|m| m.inspected_items).collect();
        inspected_items.sort();
        inspected_items.iter().rev().take(2).product::<u64>()
    }
}

struct Monkey {
    items: VecDeque<u64>,
    operation: Operation,
    test: u64,
    if_true: usize,
    if_false: usize,
    inspected_items: u64,
}

impl Monkey {
    fn parse(input: &str) -> Result<Self, Box<dyn Error>> {
        let rest = input
            .trim()
            .strip_prefix("Monkey ")
            .ok_or("Missing monkey prefix")?;
        let (_id, rest) = rest
            .split_once(":\n  Starting items: ")
            .ok_or("Missing starting items")?;
        let (items, rest) = rest
            .split_once("\n  Operation: new = old ")
            .ok_or("Missing operation")?;
        let (operation, rest) = if let Some(rest) = rest.strip_prefix("* ") {
            let (value, rest) = rest
                .split_once('\n')
                .ok_or("Missing multiplication value")?;
            let operation = if value == "old" {
                Operation::Pow2
            } else {
                Operation::Mul(value.parse()?)
            };
            (operation, rest)
        } else if let Some(rest) = rest.strip_prefix("+ ") {
            let (value, rest) = rest.split_once('\n').ok_or("Missing addition value")?;
            (Operation::Add(value.parse()?), rest)
        } else {
            return Err("Expected a valid operation".into());
        };
        let rest = rest
            .strip_prefix("  Test: divisible by ")
            .ok_or("Missing test")?;
        let (test, rest) = rest
            .split_once("\n    If true: throw to monkey ")
            .ok_or("Missing if true condition")?;
        let (if_true, if_false) = rest
            .split_once("\n    If false: throw to monkey ")
            .ok_or("Missing if false condition")?;
        Ok(Self {
            items: items
                .split(", ")
                .map(str::parse)
                .collect::<Result<_, _>>()?,
            operation,
            test: test.parse()?,
            if_true: if_true.parse()?,
            if_false: if_false.parse()?,
            inspected_items: 0,
        })
    }
}

enum Operation {
    Mul(u64),
    Add(u64),
    Pow2,
}

impl Operation {
    fn execute(&self, a: u64) -> u64 {
        match self {
            Self::Mul(b) => a * b,
            Self::Add(b) => a + b,
            Self::Pow2 => a * a,
        }
    }
}

pub(super) const DAY11: Solution = Solution {
    part1: |input| {
        let mut monkeys = Monkeys::parse(input)?;
        for _ in 0..20 {
            monkeys.run_round(|value| value / 3);
        }
        Ok(monkeys.monkey_business().to_string())
    },
    part2: |input| {
        let mut monkeys = Monkeys::parse(input)?;
        let modulo: u64 = monkeys.monkeys.iter().map(|monkey| monkey.test).product();
        for _ in 0..10_000 {
            monkeys.run_round(|value| value % modulo);
        }
        Ok(monkeys.monkey_business().to_string())
    },
};

#[cfg(test)]
mod test {
    use crate::{lines, test};
    const EXAMPLE: &str = lines!(
        "Monkey 0:"
        "  Starting items: 79, 98"
        "  Operation: new = old * 19"
        "  Test: divisible by 23"
        "    If true: throw to monkey 2"
        "    If false: throw to monkey 3"
        ""
        "Monkey 1:"
        "  Starting items: 54, 65, 75, 74"
        "  Operation: new = old + 6"
        "  Test: divisible by 19"
        "    If true: throw to monkey 2"
        "    If false: throw to monkey 0"
        ""
        "Monkey 2:"
        "  Starting items: 79, 60, 97"
        "  Operation: new = old * old"
        "  Test: divisible by 13"
        "    If true: throw to monkey 1"
        "    If false: throw to monkey 3"
        ""
        "Monkey 3:"
        "  Starting items: 74"
        "  Operation: new = old + 3"
        "  Test: divisible by 17"
        "    If true: throw to monkey 0"
        "    If false: throw to monkey 1"
    );
    test!(
        DAY11.part1,
        example: EXAMPLE => 10605,
        input: 58786,
    );
    test!(
        DAY11.part2,
        example: EXAMPLE => 2_713_310_158,
        input: 14_952_185_856,
    );
}
