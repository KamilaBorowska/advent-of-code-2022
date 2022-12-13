use crate::Solution;
use core::slice;
use std::{cmp::Ordering, error::Error};

#[derive(Debug)]
enum Signal {
    Array(Vec<Signal>),
    Int(u8),
}

impl Signal {
    fn parse_partial(input: &str) -> Result<(Signal, &str), Box<dyn Error>> {
        if let Some(mut rest) = input.strip_prefix('[') {
            if let Some(rest) = rest.strip_prefix(']') {
                return Ok((Self::Array(Vec::new()), rest));
            }
            let mut vec = Vec::new();
            loop {
                let (signal, after_partial) = Self::parse_partial(rest)?;
                vec.push(signal);
                if let Some(rest) = after_partial.strip_prefix(']') {
                    return Ok((Self::Array(vec), rest));
                } else if let Some(after_comma) = after_partial.strip_prefix(',') {
                    rest = after_comma;
                } else {
                    return Err("Expected ] or , after array element".into());
                }
            }
        } else if let Some(index) = input.find([',', ']']) {
            let (integer, rest) = input.split_at(index);
            Ok((Self::Int(integer.parse()?), rest))
        } else {
            Err("Unexpected input".into())
        }
    }

    fn parse(input: &str) -> Result<Signal, Box<dyn Error>> {
        match Self::parse_partial(input.trim())? {
            (signal, "") => Ok(signal),
            _ => Err("Expected string to match completely".into()),
        }
    }

    fn cmp(&self, other: &Self) -> Ordering {
        if let (Self::Int(a), Self::Int(b)) = (self, other) {
            return a.cmp(b);
        }
        let a = self.to_array();
        let b = other.to_array();
        for (a, b) in a.iter().zip(b) {
            match a.cmp(b) {
                Ordering::Equal => {}
                non_eq => return non_eq,
            }
        }
        a.len().cmp(&b.len())
    }

    fn to_array(&self) -> &[Signal] {
        match self {
            Self::Array(array) => array,
            int => slice::from_ref(int),
        }
    }
}

pub(super) const DAY13: Solution = Solution {
    part1: |input| {
        let mut sum = 0;
        for (i, pair) in (1..).zip(input.split("\n\n")) {
            let (a, b) = pair
                .split_once('\n')
                .ok_or("Expected two elements in a pair")?;
            let a = Signal::parse(a)?;
            let b = Signal::parse(b)?;
            if a.cmp(&b).is_le() {
                sum += i;
            }
        }
        Ok(sum.to_string())
    },
    part2: |input| {
        let mut vec = input
            .split("\n\n")
            .flat_map(|pair| pair.lines())
            .map(Signal::parse)
            .collect::<Result<Vec<_>, _>>()?;
        vec.push(Signal::Array(vec![Signal::Array(vec![Signal::Int(2)])]));
        vec.push(Signal::Array(vec![Signal::Array(vec![Signal::Int(6)])]));
        vec.sort_unstable_by(Signal::cmp);
        let mut first_packet = None;
        let mut second_packet = None;
        for (i, elem) in (1..).zip(vec) {
            if let Signal::Array(vec) = elem {
                if let [Signal::Array(vec)] = vec.as_slice() {
                    match vec.as_slice() {
                        [Signal::Int(2)] => first_packet = Some(i),
                        [Signal::Int(6)] => second_packet = Some(i),
                        _ => {}
                    }
                }
            }
        }
        let decoder_key = first_packet.ok_or("Missing first packet")?
            * second_packet.ok_or("Missing second packet")?;
        Ok(decoder_key.to_string())
    },
};

#[cfg(test)]
mod test {
    use crate::{lines, test};
    const EXAMPLE: &str = lines!(
        "[1,1,3,1,1]"
        "[1,1,5,1,1]"
        ""
        "[[1],[2,3,4]]"
        "[[1],4]"
        ""
        "[9]"
        "[[8,7,6]]"
        ""
        "[[4,4],4,4]"
        "[[4,4],4,4,4]"
        ""
        "[7,7,7,7]"
        "[7,7,7]"
        ""
        "[]"
        "[3]"
        ""
        "[[[]]]"
        "[[]]"
        ""
        "[1,[2,[3,[4,[5,6,7]]]],8,9]"
        "[1,[2,[3,[4,[5,6,0]]]],8,9]"
    );
    test!(
        DAY13.part1,
        example: EXAMPLE => 13,
        input: 5_659,
    );
    test!(
        DAY13.part2,
        example: EXAMPLE => 140,
        input: 22_110,
    );
}
