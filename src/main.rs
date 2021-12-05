use chrono::{Datelike, Utc};
use std::str::FromStr;
use structopt::StructOpt;
// the timezone used by AoC
use chrono_tz::US::Eastern;

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(short, long)]
    day: Option<u32>,
    #[structopt(short, long)]
    year: Option<i32>,
    #[structopt(short, long)]
    sample: bool,
    #[structopt(short, long)]
    all: bool,
    #[structopt(short, long, default_value = "both")]
    part: Part,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
enum Part {
    Both,
    P1,
    P2,
}
impl Part {
    fn p1(&self) -> bool {
        self == &Self::P1 || self == &Self::Both
    }
    fn p2(&self) -> bool {
        self == &Self::P2 || self == &Self::Both
    }
}

impl FromStr for Part {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "1" => Part::P1,
            "2" => Part::P2,
            _ => Part::Both,
        })
    }
}

fn main() {
    let opt = Opt::from_args();
    let aoc_time = Utc::now().with_timezone(&Eastern);
    let year = match opt.year {
        Some(y) => {
            if y < 2000 {
                y + 2000
            } else {
                y
            }
        }
        None => aoc_time.year(),
    };
    if !opt.all {
        // run a single challenge
        // EST/UTC-5
        let day = match opt.day {
            Some(d) => d,
            None => aoc_time.day(),
        };
        let (day_gen, day_input, day_sample) = &aoc::YEARS[&year][&day];
        run_day(
            day_gen.as_ref(),
            day_input,
            day_sample,
            day,
            opt.sample,
            opt.part,
        )
    } else {
        for (day, (day_gen, day_input, day_sample)) in aoc::YEARS[&year].iter() {
            run_day(
                day_gen.as_ref(),
                day_input,
                day_sample,
                *day,
                opt.sample,
                opt.part,
            )
        }
    }
}

fn run_day(
    day: &(dyn aoc::DayGen + Sync),
    input_str: &str,
    sample_str: &str,
    day_num: u32,
    sample: bool,
    part: Part,
) {
    let day = day.input(if sample { sample_str } else { input_str });
    println!("The solution for day {} is:", day_num,);
    if part.p1() {
        println!("part 1: {}", day.part1(),);
    }
    if part.p2() {
        println!("part 2: {}", day.part2(),);
    }
}
