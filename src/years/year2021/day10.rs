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

fn corrupt(line: &[char]) -> Result<char, Vec<char>> {
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
        self.input.iter().map(|c| corrupt(c)).filter_map(Result::ok).map(|c| match c {
            // <{[(
            ')' => 3,
            ']' => 57,
            '}' => 1197,
            '>' => 25137,
            _ => unreachable!(),
        }).sum::<i64>().to_string()
    }

    fn part2(&self) -> String {
        let mut points: Vec<_> = self.input.iter().filter_map(|c| if let Err(c) = corrupt(c) {
            Some(c)
        } else {
            None
        }).map(|c| c.into_iter().rev().fold(0u64, |acc, p| 5*acc + match p {
            '(' => 1,
            '[' => 2,
            '{' => 3,
            '<' => 4,
            _ => unreachable!(),
        })).collect();
        points.sort_unstable();
        points[(points.len()-1)/2].to_string()
    }
}


mod tests {
    #![allow(unused_imports)]
    use crate::Day as _;
    use super::*;
    #[test]
    fn sample_part2_test() {
        let input = concat!(
            "[({(<(())[]>[[{[]{<()<>>\n",
            "[(()[<>])]({[<{<<[]>>(\n",
            "{([(<{}[<>[]}>{[]{[(<()>\n",
            "(((({<>}<{<{<>}{[]{[]{}\n",
            "[[<[([]))<([[{}[[()]]]\n",
            "[{[{({}]{}}([{[{{{}}([]\n",
            "{<[[]]>}<{[{[{[]{()[[[]\n",
            "[<(<(<(<{}))><([]([]()\n",
            "<{([([[(<>()){}]>(<<{{\n",
            "<{([{{}}[<[[[<>{}]]]>[]]",
        );
        let day = Day::from_str(input);
        assert_eq!(day.input.len(), 10);
        assert_eq!(day.input[0].len(), 24);
        assert_eq!(day.part2(), 288957u64.to_string());
    }
}
