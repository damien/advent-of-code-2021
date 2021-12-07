use std::cmp::Ordering;

pub trait Trend {
    fn trend(&self) -> Option<Ordering>;
}
