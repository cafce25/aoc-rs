use std::{
    cmp::Ordering,
    collections::HashSet,
    ops::{Deref, DerefMut},
};

use itertools::Itertools;

pub struct DayGen;

type Input = Vec<Report>;

#[derive(Debug, Clone)]
pub struct Report {
    // id: u8,
    beacons: HashSet<[i64; 3]>,
}

impl Report {
    fn signatures(&self) -> Signatures {
        Signatures(array_init::array_init(|i| {
            let sig = self.beacons.iter().map(|b| b[i % 3]).sorted();
            if i / 3 == 0 {
                sig.collect()
            } else {
                sig.rev().map(|x| -x).collect()
            }
        }))
    }

    fn join_with_hints(
        &mut self,
        other: &Report,
        offset: [i64; 3],
        rotation: [(i64, usize); 3],
    ) -> bool {
        let rotated: Vec<[i64; 3]> = other
            .beacons
            .iter()
            .map(|b| array_init::array_init(|i| rotation[i].0 * b[rotation[i].1] + offset[i]))
            .collect();
        if rotated.iter().filter(|&b| self.beacons.contains(b)).count() >= 12 {
            let mut matched = 0;
            rotated.into_iter().for_each(|b| {
                if !self.beacons.insert(b) {
                    matched += 1;
                }
            });
            assert!(matched >= 12);
            return true;
        }
        false
    }
}

#[derive(Debug)]
struct Signatures([Vec<i64>; 6]);

impl Signatures {
    fn matches(&self, signature: &[i64]) -> Vec<(usize, i64)> {
        self.0
            .iter()
            .enumerate()
            .flat_map(|(p, sig)| matching_sigs(p, sig, signature))
            .collect()
    }
}

impl Deref for Signatures {
    type Target = [Vec<i64>; 6];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Signatures {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

fn matching_sigs(p: usize, a: &[i64], b: &[i64]) -> Vec<(usize, i64)> {
    let l = a[a.len() - 1] - b[0];
    let big_l = b[b.len() - 1] - a[0];
    (-l..big_l)
        .map(|d| (d, matching_points(d, a, b)))
        .filter(|(_, matching)| *matching >= 12)
        .map(|(d, _)| (p, d))
        .collect()
}
#[inline]
fn matching_points(d: i64, a: &[i64], b: &[i64]) -> usize {
    let mut matching = 0;
    let mut i = 0;
    let mut j = 0;
    while i < a.len() && j < b.len() {
        match (a[i] + d).cmp(&b[j]) {
            Ordering::Less => i += 1,
            Ordering::Greater => j += 1,
            Ordering::Equal => {
                while i + 1 < a.len() && (a[i + 1] + d) == b[j] {
                    matching += 1;
                    i += 1;
                }
                while j + 1 < b.len() && (a[i] + d) == b[j + 1] {
                    matching += 1;
                    j += 1;
                }
                matching += 1;
                i += 1;
                j += 1;
            }
        }
    }
    matching
}

struct Day {
    input: Input,
}

impl Day {
    pub fn from_str(input: &str) -> Self {
        Self {
            input: parsers::parse_input(input).unwrap(),
        }
    }
}
fn matches_to_hints(
    x_matches: &[(usize, i64)],
    y_matches: &[(usize, i64)],
    z_matches: &[(usize, i64)],
) -> Vec<([i64; 3], [(i64, usize); 3])> {
    let mut hints = Vec::new();
    for (x_idx, dx) in x_matches {
        for (y_idx, dy) in y_matches {
            for (z_idx, dz) in z_matches {
                let permut = [x_idx % 3, y_idx % 3, z_idx % 3];
                let flip = [
                    1 - 2 * (*x_idx as i64 / 3),
                    1 - 2 * (*y_idx as i64 / 3),
                    1 - 2 * (*z_idx as i64 / 3),
                ];
                if [[0, 1, 2], [1, 2, 0], [2, 0, 1]].contains(&permut)
                    == (flip.iter().copied().product::<i64>() > 0)
                {
                    hints.push((
                        [*dx, *dy, *dz],
                        array_init::array_init(|i| (flip[i], permut[i])),
                    ))
                }
            }
        }
    }
    hints
}

impl crate::Day for Day {
    fn part1(&self) -> String {
        let mut base: Vec<_> = self
            .input
            .iter()
            .cloned()
            .map(|report| {
                let sigs = report.signatures();
                (report, sigs)
            })
            .collect();
        'over: while base.len() > 1 {
            for i in 0..base.len() {
                for j in i + 1..base.len() {
                    let Signatures([x_sig, y_sig, z_sig, ..]) = &base[i].1;
                    let x_matches = base[j].1.matches(x_sig);
                    if !x_matches.is_empty() {
                        let y_matches = base[j].1.matches(y_sig);
                        let z_matches = base[j].1.matches(z_sig);
                        if !y_matches.is_empty() && !z_matches.is_empty() {
                            let hints = matches_to_hints(&x_matches, &y_matches, &z_matches);
                            for (offset, rotation) in hints {
                                let base_j = base.remove(j).0;
                                if base[i].0.join_with_hints(&base_j, offset, rotation) {
                                    base[i].1 = base[i].0.signatures();
                                    continue 'over;
                                } else {
                                    let sig = base_j.signatures();
                                    base.insert(j, (base_j, sig));
                                }
                            }
                        }
                    }
                }
            }
        }

        base[0].0.beacons.len().to_string()
    }

    fn part2(&self) -> String {
        let mut joins = Vec::with_capacity(self.input.len());
        let mut base: Vec<_> = self
            .input
            .iter()
            .cloned()
            .map(|report| {
                let sigs = report.signatures();
                (report, sigs)
            })
            .collect();
        'over: while base.len() > 1 {
            for i in 0..base.len() {
                for j in i + 1..base.len() {
                    let Signatures([x_sig, y_sig, z_sig, ..]) = &base[i].1;
                    let x_matches = base[j].1.matches(x_sig);
                    if !x_matches.is_empty() {
                        let y_matches = base[j].1.matches(y_sig);
                        let z_matches = base[j].1.matches(z_sig);
                        if !y_matches.is_empty() && !z_matches.is_empty() {
                            let hints = matches_to_hints(&x_matches, &y_matches, &z_matches);
                            for (offset, rotation) in hints {
                                let base_j = base.remove(j).0;
                                if base[i].0.join_with_hints(&base_j, offset, rotation) {
                                    joins.push((i, j, offset, rotation));
                                    base[i].1 = base[i].0.signatures();
                                    continue 'over;
                                } else {
                                    let sig = base_j.signatures();
                                    base.insert(j, (base_j, sig));
                                }
                            }
                        }
                    }
                }
            }
        }

        Iterator::max(joins
            .iter()
            .map(|(_, _, [x, y, z], _)| [x, y, z])
            .permutations(2)
            .map(|pair| (pair[0], pair[1]))
            .map(|([x, y, z], [x2, y2, z2])| (x-x2).abs() + (y-y2).abs() + (z-z2).abs()))
            .unwrap()
            .to_string()
    }
}

impl crate::DayGen for DayGen {
    fn input(&self, input: &str) -> Box<dyn crate::Day> {
        Box::new(Day::from_str(input))
    }
}

mod parsers {
    use super::*;
    use anyhow::{anyhow, Result};
    use nom::{
        bytes::complete::tag,
        character::complete::{char, i64, u8},
        combinator::{all_consuming, map, map_res},
        multi::separated_list1,
        sequence::{delimited, tuple},
        IResult,
    };

    pub fn parse_input(input: &str) -> Result<Input> {
        let (_, reports) = reports(input)
            .map_err(|e| anyhow!(format!("failed parsing {:?}", e)))?
            .clone();
        Ok(reports)
    }

    fn reports(input: &str) -> IResult<&str, Vec<Report>> {
        all_consuming(separated_list1(tag("\n\n"), report))(input)
    }

    fn report(input: &str) -> IResult<&str, Report> {
        map(tuple((report_id, beacons)), |(_id, beacons)| Report {
            // id,
            beacons,
        })(input)
    }

    fn report_id(input: &str) -> IResult<&str, u8> {
        delimited(tag("--- scanner "), u8, tag(" ---\n"))(input)
    }

    fn beacons(input: &str) -> IResult<&str, HashSet<[i64; 3]>> {
        map(separated_list1(char('\n'), beacon), |s| {
            s.into_iter().collect()
        })(input)
    }

    fn beacon(input: &str) -> IResult<&str, [i64; 3]> {
        map_res(separated_list1(char(','), i64), |xyz| {
            if xyz.len() != 3 {
                Err("invalid_length")
            } else {
                Ok([xyz[0], xyz[1], xyz[2]])
            }
        })(input)
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn simple_test() {
            let input = concat!(
                "--- scanner 0 ---\n",
                "-1,-1,1\n",
                "-2,-2,2\n",
                "-3,-3,3\n",
                "-2,-3,1\n",
                "5,6,-4\n",
                "8,0,7"
            );
            let reports = parse_input(input).unwrap();
            assert_eq!(reports.len(), 1);
            // assert_eq!(reports[0].id, 0);
            assert_eq!(reports[0].beacons.len(), 6);
            assert!(reports[0].beacons.contains(&[-1, -1, 1]));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn join_test() {
        let input = concat!(
            "--- scanner 0 ---\n",
            "404,-588,-901\n",
            "528,-643,409\n",
            "-838,591,734\n",
            "390,-675,-793\n",
            "-537,-823,-458\n",
            "-485,-357,347\n",
            "-345,-311,381\n",
            "-661,-816,-575\n",
            "-876,649,763\n",
            "-618,-824,-621\n",
            "553,345,-567\n",
            "474,580,667\n",
            "-447,-329,318\n",
            "-584,868,-557\n",
            "544,-627,-890\n",
            "564,392,-477\n",
            "455,729,728\n",
            "-892,524,684\n",
            "-689,845,-530\n",
            "423,-701,434\n",
            "7,-33,-71\n",
            "630,319,-379\n",
            "443,580,662\n",
            "-789,900,-551\n",
            "459,-707,401\n",
            "\n",
            "--- scanner 1 ---\n",
            "686,422,578\n",
            "605,423,415\n",
            "515,917,-361\n",
            "-336,658,858\n",
            "95,138,22\n",
            "-476,619,847\n",
            "-340,-569,-846\n",
            "567,-361,727\n",
            "-460,603,-452\n",
            "669,-402,600\n",
            "729,430,532\n",
            "-500,-761,534\n",
            "-322,571,750\n",
            "-466,-666,-811\n",
            "-429,-592,574\n",
            "-355,545,-477\n",
            "703,-491,-529\n",
            "-328,-685,520\n",
            "413,935,-424\n",
            "-391,539,-444\n",
            "586,-435,557\n",
            "-364,-763,-893\n",
            "807,-499,-711\n",
            "755,-354,-619\n",
            "553,889,-390"
        );

        let data = parsers::parse_input(input).unwrap();
        let mut a = data[0].clone();
        let a_sig = a.signatures();
        let b = data[1].clone();
        let b_sig = b.signatures();

        let x_matches = b_sig.matches(&a_sig[0]);
        let y_matches = b_sig.matches(&a_sig[1]);
        let z_matches = b_sig.matches(&a_sig[2]);
        assert_eq!(x_matches, [(3, 68)]);
        assert_eq!(y_matches, [(1, -1246)]);
        assert_eq!(z_matches, [(5, -43)]);
        let hints = matches_to_hints(&x_matches, &y_matches, &z_matches);
        assert_eq!(hints, [([68, -1246, -43], [(-1, 0), (1, 1), (-1, 2)])]);
        assert_eq!(hints.len(), 1);
        for (offset, rotation) in hints {
            assert!(a.join_with_hints(&b, offset, rotation));
        }
    }
    #[test]
    fn matching_points_test() {
        assert_eq!(matching_points(0, &[1, 2], &[1, 2]), 2);
        assert_eq!(matching_points(1, &[1, 2], &[2, 3]), 2);
    }

    #[test]
    fn matching_sigs_test() {
        assert_eq!(
            matching_sigs(
                0,
                &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
                &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]
            ),
            [(0, 0)]
        );
        assert_eq!(
            matching_sigs(
                0,
                &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
                &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13]
            ),
            [(0, 0), (0, 1)]
        );
    }
}
