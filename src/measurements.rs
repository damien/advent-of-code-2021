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