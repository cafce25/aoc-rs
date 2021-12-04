pub struct DayGen;

impl crate::DayGen for DayGen {
    fn input(&self, input: &str) -> Box<dyn crate::Day> {
        let input = input
            .split(',')
            .filter_map(|line| line.trim().parse().ok())
            .collect();
        Box::new(Day::new(input))
    }
}

type Input = Vec<i64>;

struct Day {
    input: Input,
}

impl Day {
    pub fn new(input: Input) -> Self {
        Self { input }
    }
}

impl crate::Day for Day {
    fn part1(&self) -> String {
        let mut input = self.input.clone();
        let mut i = 0;
        input[1] = 12;
        input[2] = 2;
        loop {
            let (in1, in2, out) = (input[i+1], input[i+2], input[i+3]);
            match input[i] {
                1 => input[out as usize] = input[in1 as usize] + input[in2 as usize],
                2 => input[out as usize] = input[in1 as usize] * input[in2 as usize],
                99 => break,
                _ => unreachable!(),
            }
            i += 4;
        }
        format!("{}", input[0])
    }

    fn part2(&self) -> String {
        todo!()
    }
}
