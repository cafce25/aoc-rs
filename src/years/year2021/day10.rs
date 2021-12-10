pub struct DayGen;

impl crate::DayGen for DayGen {
    fn input(&self, input: &str) -> Box<dyn crate::Day> {
        Box::new(Day::from_str(input))
    }
}

type Input = Vec<Vec<char>>;

struct Day {
    input: Input,
}

impl Day {
    pub fn from_str(input: &str) -> Self {
        let input: Vec<Vec<_>> = input
            .lines()
            .map(str::chars)
            .map(Iterator::collect)
            .collect();
        Self { input }
    }
}

fn corrupt(line: &Vec<char>) -> Result<char, Vec<char>> {
    let mut parens = Vec::new();
    for par in line {
        if ['(', '[', '{', '<'].contains(par) {
            parens.push(*par);
        } else {
            match parens.pop() {
                Some('(') if *par == ')' => (),
                Some('[') if *par == ']' => (),
                Some('{') if *par == '}' => (),
                Some('<') if *par == '>' => (),
                _ => return Ok(*par)
            };
        }
    }
    Err(parens)
}
impl crate::Day for Day {
    fn part1(&self) -> String {
        self.input.iter().map(corrupt).filter_map(Result::ok).map(|c| match c {
            // <{[(
            ')' => 3,
            ']' => 57,
            '}' => 1197,
            '>' => 25137,
            _ => unreachable!(),
        }).sum::<i64>().to_string()
    }

    fn part2(&self) -> String {
        todo!()
    }
}
