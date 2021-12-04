use chrono::{Datelike, Utc};
use chrono_tz::US::Eastern;
use reqwest::{blocking::Client, cookie::Jar, Url};
use std::{
    fs::{self, File},
    io::Write,
    path::Path,
    sync::Arc,
};

const FIRST_YEAR: i32 = 2015;

fn main() {
    let cookie = include_str!("cookie");

    let url = "https://adventofcode.com/".parse::<Url>().unwrap();

    let jar = Jar::default();
    jar.add_cookie_str(cookie, &url);
    let client = Client::builder()
        .cookie_provider(Arc::new(jar))
        .build()
        .unwrap();

    let now = Utc::now().with_timezone(&Eastern);

    let mut years_mod_file = File::create("src/years/mod.rs").unwrap();
    for year in FIRST_YEAR..=now.year() {
        writeln!(years_mod_file, "pub mod year{:04};", year).unwrap();
    }
    write!(
        years_mod_file,
        concat!(
            "\n",
            "type DayInfo = (\n",
            "        Box<dyn crate::DayGen + Sync>,\n",
            "        &'static str,\n",
            "        &'static str,\n",
            "    );\n",
            "\n",
            "lazy_static::lazy_static! {{\n",
            "    pub static ref YEARS: Vec<Vec<DayInfo>> = vec![\n",
        ),
    ).unwrap();
    for year in FIRST_YEAR..=now.year() {
        let year_directory_name = format!("src/years/year{}", year);
        let year_path = Path::new(&year_directory_name);
        fs::create_dir_all(year_path).unwrap();
        let mut year_days_mod_file = File::create(year_path.join("mod.rs")).unwrap();
        if now.year() > year || now.year() == year && now.month() == 12 {
            let max_day = if now.year() > year || now.day() > 24 {
                24
            } else {
                now.day()
            };

            (1..=max_day).for_each(|day| {
                writeln!(year_days_mod_file, "pub mod day{:02};", day).unwrap();
            });
            // write!(
            //     year_days_mod_file,
            //     concat!(
            //         "\n",
            //         "lazy_static::lazy_static! {{\n",
            //         "    pub static ref DAYS: Vec<(\n",
            //         "        Box<dyn crate::DayGen + Sync>,\n",
            //         "        &'static str,\n",
            //         "        &'static str,\n",
            //         "    )> = vec![\n",
            //     )
            // )
            // .unwrap();
            writeln!(
                years_mod_file,
                "        vec![",
            )
            .unwrap();
            for day in 1..=max_day {
                let input_file_name = format!("input{:02}.txt", day);
                let input_path = year_path.join(&input_file_name);
                let sample_file_name = format!("sample{:02}.txt", day);
                let sample_path = Path::new(&year_directory_name).join(&sample_file_name);
                let source_file_name = format!("day{:02}.rs", day);
                let source_path = Path::new(&year_directory_name).join(&source_file_name);

                if !input_path.is_file() {
                    let mut input_file = File::create(&input_path).unwrap();
                    let url = format!("https://adventofcode.com/{}/day/{}/input", year, day)
                        .parse::<Url>()
                        .unwrap();

                    writeln!(
                        input_file,
                        "{}",
                        client.get(url).send().unwrap().text().unwrap()
                    )
                    .unwrap();
                }

                // write!(
                //     year_days_mod_file,
                //     concat!(
                //         "        (\n",
                //         "            Box::new(day{0:02}::DayGen),\n",
                //         "            {1},\n",
                //         "            {2},\n",
                //         "        ),\n",
                //     ),
                //     day,
                //     if input_path.is_file() {
                //         format!("include_str!(\"./{}\")", input_file_name)
                //     } else {
                //         format!("\"no input for {day}\"", day = day)
                //     },
                //     if sample_path.is_file() {
                //         format!("include_str!(\"./{}\")", sample_file_name)
                //     } else {
                //         format!("\"no example for {day}\"", day = day)
                //     }
                // )
                // .unwrap();
                write!(
                    years_mod_file,
                    concat!(
                        "            (\n",
                        "                Box::new(year{:04}::day{:02}::DayGen),\n",
                        "                {},\n",
                        "                {},\n",
                        "            ),\n",
                    ),
                    year,
                    day,
                    if input_path.is_file() {
                        format!("include_str!(\"./year{}/{}\")", year, input_file_name)
                    } else {
                        format!("\"no input for {day}\"", day = day)
                    },
                    if sample_path.is_file() {
                        format!("include_str!(\"./year{}/{}\")", year, sample_file_name)
                    } else {
                        format!("\"no example for {day}\"", day = day)
                    },
                )
                .unwrap();
                if !source_path.is_file() {
                    let mut source_file = File::create(source_path).unwrap();
                    write!(source_file, r#"#![allow(dead_code)]
pub struct DayGen;

impl crate::DayGen for DayGen {{
    fn input(&self, input: &str) -> Box<dyn crate::Day> {{
        let input = input.split('\n').map(|line| line.to_owned()).collect();
        Box::new(Day::new(input))
    }}
}}

type Input = Vec<String>;

struct Day {{
    input: Input,
}}

impl Day {{
    pub fn new(input: Input) -> Self {{
        Self {{ input }}
    }}
}}

impl crate::Day for Day {{
    fn part1(&self) -> String {{
        todo!()
    }}

    fn part2(&self) -> String {{
        todo!()
    }}
}}"#).unwrap();
                }
            }
            writeln!(years_mod_file, "        ],").unwrap();
            // write!(year_days_mod_file, concat!("    ];\n}}\n",)).unwrap();
        }
    }
    writeln!(years_mod_file, "    ];\n}}").unwrap();
}
