use std::{collections::HashSet, error::Error, ops::Range};

use crate::Solution;

struct Sensor {
    sensor_x: i32,
    sensor_y: i32,
    beacon_x: i32,
    beacon_y: i32,
}

fn parse_sensor(line: &str) -> Result<Sensor, Box<dyn Error>> {
    let rest = line
        .strip_prefix("Sensor at x=")
        .ok_or("Expected sensor x position")?;
    let (sensor_x, rest) = rest
        .split_once(", y=")
        .ok_or("Expected sensor y position")?;
    let (sensor_y, rest) = rest
        .split_once(": closest beacon is at x=")
        .ok_or("Expected beacon x position")?;
    let (beacon_x, beacon_y) = rest
        .split_once(", y=")
        .ok_or("Expected beacon y position")?;
    Ok(Sensor {
        sensor_x: sensor_x.parse()?,
        sensor_y: sensor_y.parse()?,
        beacon_x: beacon_x.parse()?,
        beacon_y: beacon_y.parse()?,
    })
}

fn parse_sensors(input: &str) -> Result<Vec<Sensor>, Box<dyn Error>> {
    input.lines().map(parse_sensor).collect()
}

fn get_ranges(sensors: &[Sensor], row: i32) -> Vec<Range<i32>> {
    let mut ranges: Vec<Range<i32>> = Vec::new();
    for &Sensor {
        sensor_x,
        sensor_y,
        beacon_x,
        beacon_y,
    } in sensors
    {
        let distance = sensor_x.abs_diff(beacon_x) + sensor_y.abs_diff(beacon_y);
        let y_diff = row.abs_diff(sensor_y);
        let width = match distance.checked_sub(y_diff) {
            Some(0) | None => continue,
            Some(width) => width as i32,
        };
        let mut range = sensor_x - width..sensor_x + width + 1;
        let mut i = 0;
        while i < ranges.len() {
            let other_range = &mut ranges[i];
            if range.start <= other_range.start && range.end >= other_range.start
                || other_range.start <= range.start && other_range.end >= range.start
            {
                range = range.start.min(other_range.start)..range.end.max(other_range.end);
                ranges.swap_remove(i);
            } else {
                i += 1;
            }
        }
        ranges.push(range);
    }
    ranges
}

fn find_invalid_beacon_positions(sensors: &[Sensor], row: i32) -> Result<usize, Box<dyn Error>> {
    let beacons_on_row: HashSet<i32> = sensors
        .iter()
        .filter(|sensor| sensor.beacon_y == row)
        .map(|sensor| sensor.beacon_x)
        .collect();
    Ok(get_ranges(sensors, row)
        .iter()
        .map(|range| range.len())
        .sum::<usize>()
        - beacons_on_row.len())
}

fn part2<const SEARCH_SPACE: i32>(input: &str) -> Result<String, Box<dyn Error>> {
    let sensors = parse_sensors(input)?;
    for row in 0..SEARCH_SPACE {
        let ranges = get_ranges(&sensors, row);
        if let [a, b] = ranges.as_slice() {
            return Ok((i64::from(a.end.min(b.end)) * 4_000_000 + i64::from(row)).to_string());
        }
    }
    Err("Unable to find tuning frequency".into())
}

pub(super) const DAY15: Solution = Solution {
    part1: |input| {
        Ok(find_invalid_beacon_positions(&parse_sensors(input)?, 2_000_000)?.to_string())
    },
    part2: part2::<4000000>,
};

#[cfg(test)]
mod test {
    use super::{find_invalid_beacon_positions, parse_sensors, part2};
    use crate::{lines, test};
    const EXAMPLE: &str = lines!(
        "Sensor at x=2, y=18: closest beacon is at x=-2, y=15"
        "Sensor at x=9, y=16: closest beacon is at x=10, y=16"
        "Sensor at x=13, y=2: closest beacon is at x=15, y=3"
        "Sensor at x=12, y=14: closest beacon is at x=10, y=16"
        "Sensor at x=10, y=20: closest beacon is at x=10, y=16"
        "Sensor at x=14, y=17: closest beacon is at x=10, y=16"
        "Sensor at x=8, y=7: closest beacon is at x=2, y=10"
        "Sensor at x=2, y=0: closest beacon is at x=2, y=10"
        "Sensor at x=0, y=11: closest beacon is at x=2, y=10"
        "Sensor at x=20, y=14: closest beacon is at x=25, y=17"
        "Sensor at x=17, y=20: closest beacon is at x=21, y=22"
        "Sensor at x=16, y=7: closest beacon is at x=15, y=3"
        "Sensor at x=14, y=3: closest beacon is at x=15, y=3"
        "Sensor at x=20, y=1: closest beacon is at x=15, y=3"
    );
    test!(
        DAY15.part1,
        fn example() {
            assert_eq!(find_invalid_beacon_positions(&parse_sensors(EXAMPLE).unwrap(), 10).unwrap(), 26);
        }
        input: 5_256_611,
    );
    test!(
        DAY15.part2,
        fn example() {
            assert_eq!(part2::<20>(EXAMPLE).unwrap(), "56000011");
        }
        input: 13_337_919_186_981,
    );
}
