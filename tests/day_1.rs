mod test_helpers;

use std::io::BufRead;
use std::cmp::Ordering;
use std::ops::Add;

use advent_of_code_2021::{measurements::Measurements, calculations::trend::Trend};
use test_helpers::Resource;

// Source: https://adventofcode.com/2021/day/1
#[test]
fn test_day_1() {
    let input = Resource::new("day_1_input.txt");
    let reader = input.reader();
    let mut measurements = Measurements::default();
    let mut count: usize = 0;

    for line in reader.lines() {
        let data = line.unwrap().parse::<usize>().unwrap();
        measurements.push(data);

        let trend = measurements.trend();
        if let Some(Ordering::Greater) = trend {
            count = count.add(1);
        }
    };

    assert_eq!(count, 1692);
}