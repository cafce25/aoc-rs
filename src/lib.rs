#![feature(drain_filter, test)]
pub mod years;

pub const MIN_YEAR: i32 = 2015;
pub use years::YEARS;

pub trait DayGen {
    fn input<'a>(&'a self, input: &'a str) -> Box<dyn Day + 'a>;
}
pub trait Day {
    fn part1(&self) -> String;
    fn part2(&self) -> String;
}
