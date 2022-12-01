mod day1;
#[cfg(test)]
mod testmacros;

use std::env;
use std::error::Error;
use std::io::{self, Read, Write};

struct Solution {
    part1: fn(&str) -> Result<String, Box<dyn Error + '_>>,
    part2: fn(&str) -> Result<String, Box<dyn Error + '_>>,
}

const SOLUTIONS: &[Solution] = &[day1::DAY1];

const USAGE: &str = "advent-of-code-2022
USAGE:
    advent-of-code-2022 <day> [input]
FLAGS:
    -h, --help      Prints help information
ARGS:
    <day>      Day for which a solution should be ran
    <input>    Input, if not provided taken from stdin";

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = env::args();
    // Skip the program name
    args.next();
    let mut day = args.next();
    if [Some("--help"), Some("-h")].contains(&day.as_deref()) {
        day = None;
    }
    let day = if let Some(day) = day {
        day
    } else {
        eprintln!("{}", USAGE);
        return Ok(());
    };
    let day: usize = day.parse()?;
    let solution = SOLUTIONS.get(day - 1).ok_or("Day number out of range")?;
    let input = if let Some(input) = args.next() {
        input
    } else {
        let mut input = String::new();
        io::stdin().read_to_string(&mut input)?;
        input
    };
    writeln!(
        io::stdout(),
        "Part 1: {}",
        (solution.part1)(&input).map_err(|e| e.to_string())?
    )?;
    writeln!(
        io::stdout(),
        "Part 2: {}",
        (solution.part2)(&input).map_err(|e| e.to_string())?
    )?;
    Ok(())
}
