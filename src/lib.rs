#![feature(drain_filter)]
mod years;

pub const MIN_YEAR: i32 = 2015;
pub use years::YEARS;

pub trait DayGen {
    fn input(&self, input: &str) -> Box<dyn Day>;
}
pub trait Day {
    fn part1(&self) -> String;
    fn part2(&self) -> String;
}
