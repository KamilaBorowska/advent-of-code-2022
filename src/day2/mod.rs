use crate::Solution;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

fn parse_line(line: &str) -> Result<(Shape, u8), &'static str> {
    match line.as_bytes() {
        &[opponent, b' ', response] => Ok((Shape::parse_opponent(opponent)?, response)),
        _ => Err("Line doesn't match expected pattern"),
    }
}

impl Shape {
    fn parse_opponent(opponent: u8) -> Result<Self, &'static str> {
        Ok(match opponent {
            b'A' => Self::Rock,
            b'B' => Self::Paper,
            b'C' => Self::Scissors,
            _ => return Err("Unexpected opponent shape"),
        })
    }

    fn shape_score(self) -> u32 {
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }

    fn outcome(self, opponent: Self) -> u32 {
        let match_result = match (self, opponent) {
            (a, b) if a == b.winning_shape() => 6,
            (a, b) if a == b => 3,
            _ => 0,
        };
        self.shape_score() + match_result
    }

    fn winning_shape(self) -> Self {
        match self {
            Shape::Rock => Shape::Paper,
            Shape::Paper => Shape::Scissors,
            Shape::Scissors => Shape::Rock,
        }
    }

    fn losing_shape(self) -> Self {
        self.winning_shape().winning_shape()
    }
}

pub(super) const DAY2: Solution = Solution {
    part1: |input| {
        input
            .lines()
            .map(|line| {
                let (opponent, response) = parse_line(line)?;
                let response = match response {
                    b'X' => Shape::Rock,
                    b'Y' => Shape::Paper,
                    b'Z' => Shape::Scissors,
                    _ => return Err("Unexpected response"),
                };
                Ok(response.outcome(opponent))
            })
            .try_fold(0, |acc, score| Ok(acc + score?))
            .map(|total_score| total_score.to_string())
    },
    part2: |input| {
        input
            .lines()
            .map(|line| {
                let (opponent, response) = parse_line(line)?;
                let response = match response {
                    b'X' => opponent.losing_shape(),
                    b'Y' => opponent,
                    b'Z' => opponent.winning_shape(),
                    _ => return Err("Unexpected outcome"),
                };
                Ok(response.outcome(opponent))
            })
            .try_fold(0, |acc, score| Ok(acc + score?))
            .map(|total_score| total_score.to_string())
    },
};

#[cfg(test)]
mod test {
    use crate::{lines, test};
    const EXAMPLE: &str = lines!("A Y" "B X" "C Z");
    test!(
        DAY2.part1,
        example: EXAMPLE => 15,
        input: 10994,
    );
    test!(
        DAY2.part2,
        example: EXAMPLE => 12,
        input: 12526,
    );
}
