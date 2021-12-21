use std::{collections::HashSet, ops::{Deref, DerefMut}};

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
#[derive(PartialEq, Clone)]
struct Image {
    image: HashSet<(i64, i64)>,
    inverted: bool,
}

impl Deref for Image {
    type Target=HashSet<(i64, i64)>;

    fn deref(&self) -> &Self::Target {
        &self.image
    }
}

impl DerefMut for Image{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.image
    }
}

impl FromIterator<(i64, i64)> for Image {
    fn from_iter<T: IntoIterator<Item = (i64, i64)>>(iter: T) -> Self {
        let image = HashSet::from_iter(iter);
        Self { image, inverted: false }
    }
}


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

    fn step(&self, image: &Image) -> Image {
        let [min_y, max_y, min_x, max_x] = image.iter().fold(
            [i64::MAX, i64::MIN, i64::MAX, i64::MIN],
            |[min_y, max_y, min_x, max_x], (y, x)| {
                [min_y.min(*y), max_y.max(*y), min_x.min(*x), max_x.max(*x)]
            },
        );

        let next_inverted = if image.inverted {
            self.convert[511]
        } else {
            self.convert[0]
        };
        let mut new: Image = (min_y - 1..=max_y + 1)
            .flat_map(|y| {
                (min_x - 1..=max_x + 1).filter_map(move |x| {
                    (next_inverted
                        ^ self.convert[bits_to_num(
                            &DIRECTIONS
                                .into_iter()
                                .map(|(dy, dx)| image.inverted ^ image.contains(&(y + dy, x + dx)))
                                .collect::<Vec<_>>()[..],
                        )])
                    .then(|| (y, x))
                })
            })
            .collect();
        new.inverted = next_inverted;
        new
    }

    fn run(&self, n: usize) -> usize {
        let mut image: Image = self.image.clone();
        for _ in 0..n {
            image = self.step(&image);
        }
        image.len()
    }

}

impl crate::Day for Day {
    fn part1(&self) -> String {
        self.run(2).to_string()
    }

    fn part2(&self) -> String {
        self.run(50).to_string()
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
        assert_eq!(day.image.image, HashSet::from([(0, 0)]));
    }

    #[test]
    fn odd_step_test() {
        let input = concat!(
            "#..#.##########################################################################################################################################################################################################################################################################################################################################################################################################################################################################################################################.\n",
            "\n",
            "#",
        );

        let day = Day::from_str(input);
        assert_eq!(day.image.image, HashSet::from([(0, 0)]));

        let image2 = day.step(&day.image);
        assert_eq!(image2.image, HashSet::from([(-1, -1), (-1, 0), (-1, 1)]));
    }

    #[test]
    fn even_step_test() {
        let input = concat!(
            "###############################################################################################################################################################################################################################################################################################################################################################################################################################################################################################################################.\n",
            "\n",
            "#",
        );

        let mut day = Day::from_str(input);
        assert_eq!(day.image.image, HashSet::from([(0, 0)]));

        day.image.inverted = true;
        let image2 = day.step(&day.image);
        assert_eq!(
            image2.image,
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
