use crate::calculations::trend::Trend;
use std::cmp::Ordering;
use std::collections::VecDeque;

// This struct takes stream of measurements and counts
// occurances of interesting events that occur within
// the stream. It can be used to track trends, patterns,
// maximums or mimimums of the last N measurements.
pub struct Measurements {
    // Number of individual measurement items to retain
    pub size: usize,
    // A window of the most recent measurements ordered newest to oldest
    pub items: VecDeque<usize>,
}

impl Measurements {
    // Add a new measurement. If the new measurement causes `items` to exceed `size`,
    // the oldest measurement is removed to make space for the new item.
    #[allow(dead_code)]
    pub fn push(&mut self, item: usize) -> &Self {
        self.items.insert(0, item);
        self
    }
}

impl Trend for Measurements {
    fn trend(&self) -> Option<Ordering> {
        if self.items.len() < 2{
            return None;
        }

        let next = self.items.get(0).unwrap();
        let prev = self.items.get(1).unwrap();

        Some(next.cmp(prev))
    }
}

impl Default for Measurements {
    fn default() -> Self {
        let default_size = 2;

        Self {
            size: default_size,
            items: VecDeque::with_capacity(default_size),
        }
    }
}

impl Clone for Measurements {
    fn clone(&self) -> Self {
        Self {
            size: self.size,
            items: self.items.clone()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::measurements::Measurements;
    use crate::calculations::trend::Trend;

    use std::cmp::Ordering;
    use std::ops::Add;

    fn count_trend(mut measurements: Measurements, ordering: Ordering, inputs: Vec<usize>) -> i32
    {
        let mut count: i32 = 0;

        for datum in inputs {
            measurements.push(datum);
            let trend = measurements.trend();

            match trend {
                Some(ord) => {
                    if ord == ordering {
                        count = count.add(1);
                    }
                }
                _ => ()
            }
        }

        count
    }

    #[test]
    // Source: https://adventofcode.com/2021/day/1
    fn test_linear_upward_trend() {
        let measurements = Measurements { size: 2, ..Default::default() };
        let input = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        let expected = 7;
        let actual = count_trend(measurements, Ordering::Greater, input);

        assert_eq!(expected, actual, "Expected exactly {} occurances of an upward trend", expected);
    }

    #[test]
    fn test_linear_downward_trend() {
        let measurements = Measurements { size: 2, ..Default::default() };
        let input = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        let expected = 2;
        let actual = count_trend(measurements, Ordering::Less, input);

        assert_eq!(expected, actual, "Expected exactly {} occurances of an downward trend", expected);
    }
}