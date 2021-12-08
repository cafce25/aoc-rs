## My [Rust](https://www.rust-lang.org/) solutions to [Advent of Code](https://adventofcode.com/)

### Usage
#### Binary
* `aoc` runs the current day
* `aoc -d{day}` runs the `day` specified
* `aoc -y{year} -d{day}` runs the `year`'s `day`
* `aoc -a` runs all challenges of the current year
* `aoc -s`

You can compile & run the binary using `cargo run --` instead of the binary name `aoc`

#### Build Script
`build.rs` (see [Build Scripts](https://doc.rust-lang.org/cargo/reference/build-scripts.html)) downloads the inputs using an Advent of Code session in `./cookie` it will also generate module files which include solutions you create (`src/years/year{year}/day{day}.rs`)
