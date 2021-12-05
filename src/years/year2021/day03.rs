type Input = Vec<BitNumber>;
#[derive(Copy, Clone)]
struct BitNumber {
    num: u64,
    len: usize,
}

impl std::fmt::Debug for BitNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("{:01$b}", self.num, self.len).as_str())
    }
}

impl BitNumber {
    fn len(&self) -> usize {
        self.len
    }
    fn get(&self, index: usize) -> bool {
        let index = self.len - index - 1;
        self.num & 1 << index > 0
    }
}

impl From<&BitNumber> for u64 {
    fn from(bn: &BitNumber) -> Self {
        bn.num
    }
}
impl From<BitNumber> for u64 {
    fn from(bn: BitNumber) -> Self {
        bn.num
    }
}
impl FromIterator<bool> for BitNumber {
    fn from_iter<T: IntoIterator<Item = bool>>(iter: T) -> Self {
        let mut num = 0;
        let mut len = 0;
        for b in iter.into_iter() {
            num *= 2;
            num += u64::from(b);
            len += 1;
        }
        Self { num, len }
    }
}

pub struct DayGen;
impl crate::DayGen for DayGen {
    fn input(&self, input: &str) -> Box<dyn crate::Day> {
        let input = input
            .split('\n')
            .filter_map(|n| {
                if !n.is_empty() {
                    Some(
                        n.chars()
                            .map(|c| match c {
                                '1' => true,
                                '0' => false,
                                _ => panic!("error in input"),
                            })
                            .collect(),
                    )
                } else {
                    None
                }
            })
            .collect();
        Box::new(Day::new(input))
    }
}

struct Day {
    input: Input,
}

impl Day {
    fn new(input: Input) -> Self {
        Self { input }
    }

    fn calc_o2_co2(&self, co2: bool) -> u64 {
        let n = self.input[0].len();
        let arr = (0..n).fold(self.input.to_owned(), |input, i| {
            if input.len() <= 1 {
                return input;
            }
            let mut ones = 0;
            let mut zeros = 0;

            input.iter().for_each(|line| match line.get(i) {
                true => ones += 1,
                false => zeros += 1,
            });
            input
                .into_iter()
                .filter(|line| {
                    if (ones >= zeros) != co2 {
                        line.get(i)
                    } else {
                        !line.get(i)
                    }
                })
                .collect()
        });
        arr[0].into()
    }
}

impl crate::Day for Day {
    fn part1(&self) -> String {
        let n = self.input[0].len();
        let mut ones = vec![0; n];
        let mut zeros = vec![0; n];

        self.input.iter().for_each(|line| {
            for i in 0..n {
                match line.get(i) {
                    true => ones[i] += 1,
                    false => zeros[i] += 1,
                }
            }
        });

        let mut gamma = 0;
        let mut epsilon = 0;
        for i in 0..n {
            gamma *= 2;
            epsilon *= 2;
            if ones[i] > zeros[i] {
                gamma += 1;
            } else {
                epsilon += 1;
            }
        }

        format!(
            "The power consumption is {} = {} x {}",
            epsilon * gamma,
            gamma,
            epsilon
        )
    }
    fn part2(&self) -> String {
        let oxygen: u64 = self.calc_o2_co2(false);
        let co2: u64 = self.calc_o2_co2(true);
        format!("{} = {} x {}", oxygen * co2, oxygen, co2)
    }
}
