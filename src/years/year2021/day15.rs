use std::{
    collections::BinaryHeap,
    ops::{Index, IndexMut},
};

pub struct DayGen;

impl crate::DayGen for DayGen {
    fn input(&self, input: &str) -> Box<dyn crate::Day> {
        Box::new(Day::from_str(input))
    }
}

type Input<T> = Vec<Vec<T>>;

struct Day {
    input: Input<usize>,
}

impl Day {
    pub fn from_str(input: &str) -> Self {
        let input = input
            .lines()
            .map(|l| l.chars().map(|c| c as usize - '0' as usize).collect())
            .collect();
        Self { input }
    }
}

impl crate::Day for Day {
    fn part1(&self) -> String {
        let mut map = Astar::with(
            &self.input,
            (0, 0),
            (self.input[0].len() - 1, self.input.len() - 1),
        );

        map.astar().to_string()
    }

    fn part2(&self) -> String {
        let height = self.input.len();
        let width = self.input[0].len();
        let mut real_input = vec![vec![0; width * 5]; height * 5];
        for mx in 0..5 {
            for my in 0..5 {
                for x in 0..width {
                    for y in 0..height {
                        real_input[y + height * my][x + width * mx] =
                            ((self.input[y][x] + my + mx - 1) % 9) + 1;
                    }
                }
            }
        }
        let mut map = Astar::with(&real_input, (0, 0), (width * 5 - 1, height * 5 - 1));

        map.astar().to_string()
    }
}

struct Astar {
    tile_costs: Input<usize>,
    path_costs: Input<Option<usize>>,
    finish: Coord,
    pos: Coord,
}

impl Astar {
    fn with(tile_costs: &Input<usize>, pos: Coord, finish: Coord) -> Astar {
        let tile_costs = tile_costs.clone();
        let mut path_costs = vec![vec![None; tile_costs[0].len()]; tile_costs.len()];
        path_costs[pos.0 as usize][pos.1 as usize] = Some(0);

        Self {
            tile_costs,
            path_costs,
            finish,
            pos,
        }
    }

    fn astar(&mut self) -> usize {
        let mut candidates: BinaryHeap<(usize, Coord)> = self
            .neighbours(self.pos)
            .into_iter()
            .map(|coord| (usize::MAX - self.estimate_cost(coord), coord))
            .collect();

        while self[self.finish].is_none() {
            let (_, candidate) = candidates.pop().expect("candidate");
            if self[candidate].is_some() {
                continue;
            }
            let neighbour = self.min_neighbour(candidate);
            let path = neighbour + self.get_tile(candidate).unwrap();
            self[candidate] = Some(path);
            candidates.extend(self.neighbours(candidate).into_iter().filter_map(|coord| {
                self[coord]
                    .is_none()
                    .then(|| (usize::MAX - self.estimate_cost(coord) - path, coord))
            }));
        }
        self[self.finish].unwrap()
    }

    #[inline]
    fn estimate_cost(&self, (y, x): Coord) -> usize {
        (self.finish.0 - y + self.finish.1 - x) as usize
    }

    fn neighbours(&self, (y, x): Coord) -> Vec<Coord> {
        [(-1, 0), (1, 0), (0, 1), (0, -1)]
            .into_iter()
            .filter_map(|(dy, dx)| Some((y.checked_add_signed(dy)?, x.checked_add_signed(dx)?)))
            .filter(|coord| self.in_bounds(*coord))
            .collect()
    }

    fn min_neighbour(&self, coord: Coord) -> usize {
        self.neighbours(coord)
            .into_iter()
            .filter_map(|n_coord| self.get_path(n_coord).and_then(|v| v.map(|v| (v, n_coord))))
            .fold(
                (usize::MAX, (0, 0)),
                |p @ (vmin, _), t @ (v, _)| {
                    if vmin > v {
                        t
                    } else {
                        p
                    }
                },
            )
            .0
    }

    #[inline]
    fn get_path(&self, coord @ (y, x): Coord) -> Option<&Option<usize>> {
        if !self.in_bounds(coord) {
            return None;
        }
        Some(&self.path_costs[y][x])
    }

    #[inline]
    fn get_path_mut(&mut self, coord @ (y, x): Coord) -> Option<&mut Option<usize>> {
        if !self.in_bounds(coord) {
            return None;
        }
        Some(&mut self.path_costs[y][x])
    }

    #[inline]
    fn get_tile(&self, coord @ (y, x): Coord) -> Option<&usize> {
        if !self.in_bounds(coord) {
            return None;
        }
        Some(&self.tile_costs[y][x])
    }

    #[inline]
    fn in_bounds(&self, (y, x): Coord) -> bool {
        y < self.height() && x < self.width() as usize
    }

    #[inline]
    fn height(&self) -> usize {
        self.tile_costs.len()
    }

    #[inline]
    fn width(&self) -> usize {
        self.tile_costs[0].len()
    }
}

type Coord = (usize, usize);

impl Index<Coord> for Astar {
    type Output = Option<usize>;

    fn index(&self, coords: Coord) -> &Self::Output {
        self.get_path(coords).expect("valid index")
    }
}

impl IndexMut<Coord> for Astar {
    fn index_mut(&mut self, coords: Coord) -> &mut Self::Output {
        self.get_path_mut(coords).expect("valid index")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Day as _;
    extern crate test;

    #[bench]
    fn part2(b: &mut test::Bencher) {
        let input = &crate::YEARS[&2021][&15];
        let day = Day::from_str(input.1);
        b.iter(|| {
            day.part2();
        })
    }
}
