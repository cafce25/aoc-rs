use std::{
    collections::HashMap,
    ops::{Index, IndexMut},
};

use itertools::Itertools as _;

pub struct DayGen;

impl crate::DayGen for DayGen {
    fn input(&self, input: &str) -> Box<dyn crate::Day> {
        Box::new(Day::from_str(input))
    }
}

type Input = Image;

struct Image {
    height: usize,
    width: usize,
    data: Vec<u8>,
}

impl Image {
    fn new(data: &[u8], width: usize, height: usize) -> Self {
        Self {
            data: data.to_vec(),
            width,
            height,
        }
    }

    fn layer_size(&self) -> usize {
        self.width * self.height
    }

    fn layer_len(&self) -> usize {
        self.data.len() / self.layer_size()
    }

    fn idx(&self, x: usize, y: usize, layer: usize) -> usize {
        x + y * self.width + layer * self.layer_size()
    }

    fn get(&self, x: usize, y: usize) -> char {
        for l in 0..self.layer_len() {
            match self[(x, y, l)] {
                0 => return ' ',
                1 => return '█',
                _ => (),
            }
        }
        unimplemented!();
    }
}

impl Index<(usize, usize, usize)> for Image {
    type Output = u8;

    fn index(&self, (x, y, layer): (usize, usize, usize)) -> &Self::Output {
        &self.data[self.idx(x, y, layer)]
    }
}

impl IndexMut<(usize, usize, usize)> for Image {
    fn index_mut(&mut self, (x, y, layer): (usize, usize, usize)) -> &mut Self::Output {
        let index = self.idx(x, y, layer);
        &mut self.data[index]
    }
}

struct Day {
    input: Input,
}

impl Day {
    pub fn from_str(input: &str) -> Self {
        let input: Vec<_> = input.chars().map(|c| c as u8 - '0' as u8).collect();
        Self {
            input: Image::new(&input, 25, 6),
        }
    }
}

impl crate::Day for Day {
    fn part1(&self) -> String {
        let hist = self
            .input
            .data
            .iter()
            .chunks(self.input.layer_size())
            .into_iter()
            .map(|layer| {
                layer.fold(HashMap::new(), |mut map: HashMap<u8, usize>, pixel| {
                    *map.entry(*pixel).or_insert(0) += 1;
                    map
                })
            })
            .min_by_key(|h| h[&0])
            .unwrap();
        (hist[&1] * hist[&2]).to_string()
    }

    fn part2(&self) -> String {
        String::from("\n")
            + &(0..6)
                .map(|y| (0..25).map(|x| self.input.get(x, y).to_string()).join(""))
                .join("\n")
    }
}
