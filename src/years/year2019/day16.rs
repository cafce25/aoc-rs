pub struct DayGen;

impl crate::DayGen for DayGen {
    fn input(&self, input: &str) -> Box<dyn crate::Day> {
        Box::new(Day::from_str(input))
    }
}

type Input = Vec<i64>;

struct Day {
    input: Input,
}

impl Day {
    pub fn from_str(input: &str) -> Self {
        let input = input.chars().map(|c| c as i64 - '0' as i64).collect();
        Self { input }
    }
}

fn step(numbers: &[i64], offset: usize) -> Vec<i64> {
    let len = offset + numbers.len();
    let mut new = vec![0; numbers.len()];
    for i in (offset + 1..=len).rev() {
        if i > len / 2 {
            let i = i - offset - 1;
            if i == numbers.len() - 1 {
                new[i] = numbers[i];
            } else {
                new[i] = (new[i + 1] + numbers[i]).abs() % 10;
            }
        } else {
            let mut digits = numbers.iter().copied();
            let mut n = 0;
            digits.advance_by(i - offset - 1).unwrap();
            loop {
                n += (&mut digits).take(i).sum::<i64>();
                if digits.advance_by(i).is_err() {
                    break;
                }
                n -= (&mut digits).take(i).sum::<i64>();
                if digits.advance_by(i).is_err() {
                    break;
                }
            }
            new[i - 1 - offset] = n.abs() % 10;
        }
    }
    new
}

impl crate::Day for Day {
    fn part1(&self) -> String {
        let mut numbers = self.input.to_vec();
        for _ in 0..100 {
            numbers = step(&numbers, 0);
        }
        numbers
            .iter()
            .take(8)
            .fold(0, |acc, n| acc * 10 + n)
            .to_string()
    }

    fn part2(&self) -> String {
        let offset = self
            .input
            .iter()
            .copied()
            .take(7)
            .fold(0, |acc, n| acc * 10 + n as usize);
        let mut numbers = self
            .input
            .iter()
            .copied()
            .cycle()
            .take(self.input.len() * 10000);
        numbers.advance_by(offset).unwrap();
        let mut numbers: Vec<_> = numbers.collect();
        for _ in 0..100 {
            numbers = step(&numbers, offset);
        }

        numbers
            .iter()
            .take(8)
            .fold(0, |acc, n| acc * 10 + n)
            .to_string()
    }
}
