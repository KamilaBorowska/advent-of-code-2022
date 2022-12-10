use crate::Solution;

pub(super) const DAY10: Solution = Solution {
    part1: |input| {
        let mut x = 1;
        let mut output = 0;
        let mut cycles = 0;
        let mut next_output_cycle = 20;
        let mut add_output = |cycles, x| {
            if cycles >= next_output_cycle {
                output += next_output_cycle * x;
                next_output_cycle += 40;
            }
        };
        for line in input.lines() {
            if line == "noop" {
                cycles += 1;
            } else if let Some(add_x) = line.strip_prefix("addx ") {
                cycles += 2;
                add_output(cycles, x);
                x += add_x.parse::<i32>()?;
            } else {
                return Err(format!("Unrecognized instruction {line}").into());
            }
            add_output(cycles, x);
        }
        Ok(output.to_string())
    },
    part2: |input| {
        let mut output = String::new();
        let mut cycles = 0;
        let mut run_cycle = |x: i32| {
            output.push(if x.abs_diff(cycles) <= 1 { '#' } else { '.' });
            cycles += 1;
            if cycles == 40 {
                output.push('\n');
                cycles = 0;
            }
        };
        let mut x = 1;
        for line in input.lines() {
            if line == "noop" {
                run_cycle(x);
            } else if let Some(add_x) = line.strip_prefix("addx ") {
                run_cycle(x);
                run_cycle(x);
                x += add_x.parse::<i32>()?;
            } else {
                return Err(format!("Unrecognized instruction {line}").into());
            }
        }
        Ok(output)
    },
};

#[cfg(test)]
mod test {
    use crate::{lines, test};
    const EXAMPLE: &str = lines!(
        "addx 15"
        "addx -11"
        "addx 6"
        "addx -3"
        "addx 5"
        "addx -1"
        "addx -8"
        "addx 13"
        "addx 4"
        "noop"
        "addx -1"
        "addx 5"
        "addx -1"
        "addx 5"
        "addx -1"
        "addx 5"
        "addx -1"
        "addx 5"
        "addx -1"
        "addx -35"
        "addx 1"
        "addx 24"
        "addx -19"
        "addx 1"
        "addx 16"
        "addx -11"
        "noop"
        "noop"
        "addx 21"
        "addx -15"
        "noop"
        "noop"
        "addx -3"
        "addx 9"
        "addx 1"
        "addx -3"
        "addx 8"
        "addx 1"
        "addx 5"
        "noop"
        "noop"
        "noop"
        "noop"
        "noop"
        "addx -36"
        "noop"
        "addx 1"
        "addx 7"
        "noop"
        "noop"
        "noop"
        "addx 2"
        "addx 6"
        "noop"
        "noop"
        "noop"
        "noop"
        "noop"
        "addx 1"
        "noop"
        "noop"
        "addx 7"
        "addx 1"
        "noop"
        "addx -13"
        "addx 13"
        "addx 7"
        "noop"
        "addx 1"
        "addx -33"
        "noop"
        "noop"
        "noop"
        "addx 2"
        "noop"
        "noop"
        "noop"
        "addx 8"
        "noop"
        "addx -1"
        "addx 2"
        "addx 1"
        "noop"
        "addx 17"
        "addx -9"
        "addx 1"
        "addx 1"
        "addx -3"
        "addx 11"
        "noop"
        "noop"
        "addx 1"
        "noop"
        "addx 1"
        "noop"
        "noop"
        "addx -13"
        "addx -19"
        "addx 1"
        "addx 3"
        "addx 26"
        "addx -30"
        "addx 12"
        "addx -1"
        "addx 3"
        "addx 1"
        "noop"
        "noop"
        "noop"
        "addx -9"
        "addx 18"
        "addx 1"
        "addx 2"
        "noop"
        "noop"
        "addx 9"
        "noop"
        "noop"
        "noop"
        "addx -1"
        "addx 2"
        "addx -37"
        "addx 1"
        "addx 3"
        "noop"
        "addx 15"
        "addx -21"
        "addx 22"
        "addx -6"
        "addx 1"
        "noop"
        "addx 2"
        "addx 1"
        "noop"
        "addx -10"
        "noop"
        "noop"
        "addx 20"
        "addx 1"
        "addx 2"
        "addx 2"
        "addx -6"
        "addx -11"
        "noop"
        "noop"
        "noop"
    );
    test!(
        DAY10.part1,
        example: EXAMPLE => 13140,
        input: 17180,
    );
    mod part2 {
        use super::super::DAY10;
        use super::EXAMPLE;
        use std::error::Error;

        #[test]
        fn example() -> Result<(), Box<dyn Error>> {
            let output = concat!(
                "##..##..##..##..##..##..##..##..##..##..\n",
                "###...###...###...###...###...###...###.\n",
                "####....####....####....####....####....\n",
                "#####.....#####.....#####.....#####.....\n",
                "######......######......######......####\n",
                "#######.......#######.......#######.....\n"
            );
            assert_eq!((DAY10.part2)(EXAMPLE)?, output);
            Ok(())
        }

        #[test]
        fn input() -> Result<(), Box<dyn Error>> {
            let output = concat!(
                "###..####.#..#.###..###..#....#..#.###..\n",
                "#..#.#....#..#.#..#.#..#.#....#..#.#..#.\n",
                "#..#.###..####.#..#.#..#.#....#..#.###..\n",
                "###..#....#..#.###..###..#....#..#.#..#.\n",
                "#.#..#....#..#.#....#.#..#....#..#.#..#.\n",
                "#..#.####.#..#.#....#..#.####..##..###..\n",
            );
            assert_eq!((DAY10.part2)(include_str!("input"))?, output);
            Ok(())
        }
    }
}
