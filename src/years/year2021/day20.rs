use std::collections::HashSet;

const DIRECTIONS: [(i64, i64); 9] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 0),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

pub struct DayGen;

impl crate::DayGen for DayGen {
    fn input(&self, input: &str) -> Box<dyn crate::Day> {
        Box::new(Day::from_str(input))
    }
}

// TODO rewrite so that Image converts to InvertedImage and back
type Image = HashSet<(i64, i64)>;
type Convert = Vec<bool>;

struct Day {
    image: Image,
    convert: Convert,
}

impl Day {
    pub fn from_str(input: &str) -> Self {
        let (convert, image) = input.split_once("\n\n").unwrap();
        let convert: Vec<_> = convert.chars().map(|x| x == '#').collect();
        let image = image
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .filter_map(move |(x, c)| (c == '#').then(|| (y as i64, x as i64)))
            })
            .collect();
        Self { convert, image }
    }

    fn step(&self, inverted: bool, image: &Image) -> Image {
        let [min_y, max_y, min_x, max_x] = image.iter().fold(
            [i64::MAX, i64::MIN, i64::MAX, i64::MIN],
            |[min_y, max_y, min_x, max_x], (y, x)| {
                [min_y.min(*y), max_y.max(*y), min_x.min(*x), max_x.max(*x)]
            },
        );

        (min_y - 1..=max_y + 1)
            .flat_map(|y| {
                (min_x - 1..=max_x + 1).filter_map(move |x| {
                    (!inverted
                        ^ self.convert[bits_to_num(
                            &DIRECTIONS
                                .into_iter()
                                .map(|(dy, dx)| inverted ^ image.contains(&(y + dy, x + dx)))
                                .collect::<Vec<_>>()[..],
                        )])
                    .then(|| (y, x))
                })
            })
            .collect()
    }

    fn step_even(&self, image: &Image) -> Image {
        self.step(true, image)
    }

    fn step_odd(&self, image: &Image) -> Image {
        self.step(false, image)
    }

    fn step2(&self, image: &Image) -> Image {
        self.step_even(&self.step_odd(image))
    }
}

impl crate::Day for Day {
    fn part1(&self) -> String {
        let image2 = self.step2(&self.image);
        image2.len().to_string()
    }

    fn part2(&self) -> String {
        (0..(50-2)/2).fold(self.step2(&self.image), |image, _| self.step2(&image)).len().to_string()
    }
}

fn bits_to_num(v: &[bool]) -> usize {
    v.iter().fold(0, |num, bit| (num << 1) | *bit as usize)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reading_test() {
        let input = concat!(
            "################################################################################################################################################################################################################################################################################################################################################################################################################################################################################################################################\n",
            "\n",
            "#",
        );

        let day = Day::from_str(input);
        assert_eq!(day.convert, [true; 512]);
        assert_eq!(day.image, HashSet::from([(0, 0)]));
    }

    #[test]
    fn odd_step_test() {
        let input = concat!(
            "#..#.##########################################################################################################################################################################################################################################################################################################################################################################################################################################################################################################################.\n",
            "\n",
            "#",
        );

        let day = Day::from_str(input);
        assert_eq!(day.image, HashSet::from([(0, 0)]));

        let image2 = day.step_odd(&day.image);
        assert_eq!(image2, HashSet::from([(-1, -1), (-1, 0), (-1, 1)]));
    }

    #[test]
    fn even_step_test() {
        let input = concat!(
            "###############################################################################################################################################################################################################################################################################################################################################################################################################################################################################################################################.\n",
            "\n",
            "#",
        );

        let day = Day::from_str(input);
        assert_eq!(day.image, HashSet::from([(0, 0)]));

        let image2 = day.step_even(&day.image);
        assert_eq!(
            image2,
            HashSet::from([
                (-1, -1),
                (-1, 0),
                (-1, 1),
                (0, -1),
                (0, 0),
                (0, 1),
                (1, -1),
                (1, 0),
                (1, 1)
            ])
        );
    }
}
