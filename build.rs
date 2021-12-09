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
    println!("cargo:rerun-if-changed=src/years");
    let client = client();
    let now = Utc::now().with_timezone(&Eastern);

    let mut years_mod_file = File::create("src/years/mod.rs").unwrap();
    let _ = writeln!(years_mod_file, "use std::collections::BTreeMap;");
    for year in FIRST_YEAR..=now.year() {
        let _ = writeln!(years_mod_file, "pub mod year{:04};", year);
    }
    let _ = writeln!(
        years_mod_file,
        concat!(
            "\n",
            "type DayInfo = (Box<dyn crate::DayGen + Sync>, &'static str, &'static str);\n\n",
            "lazy_static::lazy_static! {{\n",
            "    pub static ref YEARS: BTreeMap<i32, BTreeMap<u32, DayInfo>> = {{\n",
            "        let mut map = BTreeMap::new();\n",
        ),
    );
    for year in FIRST_YEAR..=now.year() {
        if let Some(year_entries) = generate_year(
            year,
            now,
            &client,
            "        ".to_string(),
        ) {
            let _ = writeln!(years_mod_file, "{entries}", entries = year_entries);
        }
    }

    let _ = writeln!(years_mod_file, "        map\n    }};\n}}");
}

fn generate_year(
    year: i32,
    now: impl Datelike,
    client: &Client,
    indent: String,
) -> Option<String> {
    let year_directory_name = format!("src/years/year{}", year);
    let year_path = Path::new(&year_directory_name);
    fs::create_dir_all(year_path).unwrap();
    let year_mod_path = year_path.join("mod.rs");
    if !year_mod_path.is_file() && !year_mod_path.exists() {
        let mut year_mod_file = File::create(year_mod_path).unwrap();
        let _ = writeln!(year_mod_file, "include!(\"./day_mods.rs\");");
    }

    (now.year() > year || now.year() == year && now.month() == 12).then(|| {
        let max_day = if now.year() > year || now.day() > 24 {
            24
        } else {
            now.day()
        };

        let (mods, entries): (Vec<_>, Vec<_>) = (1..=max_day)
            .filter_map(|day| {
                generate_day(
                    day,
                    year,
                    year_path,
                    &year_directory_name,
                    client,
                    format!("{}    ", indent),
                )
            })
            .unzip();
        let year_day_mods_path = year_path.join("day_mods.rs");
        let mut year_day_mods_file = File::create(year_day_mods_path).unwrap();
        let _ = writeln!(year_day_mods_file, "{mods}", mods = mods.join("\n"));
        (!entries.is_empty()).then(|| {
            format!(
                concat!(
                    "{indent}map.insert({year}, {{\n",
                    "{indent}    let mut map = BTreeMap::<u32, DayInfo>::new();\n",
                    "{entries}\n",
                    "{indent}    map\n",
                    "{indent}}});",
                ),
                year = year,
                entries = entries.join("\n"),
                indent = indent,
            )
        })
    }).flatten()
}

fn generate_day<S: AsRef<str>>(
    day: u32,
    year: i32,
    year_path: &Path,
    year_directory_name: S,
    client: &Client,
    indent: String,
) -> Option<(String, String)> {
    let input_file_name = format!("input{:02}.txt", day);
    let input_path = year_path.join(&input_file_name);

    if !input_path.is_file() {
        let mut input_file = File::create(&input_path).unwrap();
        let url = format!("https://adventofcode.com/{}/day/{}/input", year, day)
            .parse::<Url>()
            .unwrap();

        let _ = writeln!(
            input_file,
            "{}",
            client.get(url).send().unwrap().text().unwrap()
        );
    }

    let year_directory_name = year_directory_name.as_ref();
    let source_path = Path::new(year_directory_name).join(format!("day{:02}.rs", day));
    source_path.is_file().then(|| {
        let sample_file_name = format!("sample{:02}.txt", day);
        let sample_path = Path::new(year_directory_name).join(&sample_file_name);
        (
            format!("pub mod day{:02};", day),
            format!(
                concat!(
                    "{indent}map.insert({1}, (\n",
                    "{indent}    Box::new(year{0:04}::day{1:02}::DayGen),\n",
                    "{indent}    {2},\n",
                    "{indent}    {3},\n",
                    "{indent}));",
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
                indent = indent,
            ),
        )
    })
}

fn client() -> Client {
    let cookie = include_str!("cookie");

    let url = "https://adventofcode.com/".parse::<Url>().unwrap();

    let jar = Jar::default();
    jar.add_cookie_str(cookie, &url);
    Client::builder()
        .cookie_provider(Arc::new(jar))
        .build()
        .unwrap()
}
