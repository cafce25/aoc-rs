use std::collections::HashMap;

pub struct DayGen;

impl crate::DayGen for DayGen {
    fn input<'a>(&self, input: &'a str) -> Box<dyn crate::Day + 'a> {
        Box::new(Day::from_str(input))
    }
}

type Input<'a> = HashMap<&'a str, Vec<&'a str>>;

struct Day<'a> {
    input: Input<'a>,
}

impl<'a> Day<'a> {
    pub fn from_str(input: &'a str) -> Self {
        let edges: Vec<_> = input.lines().map(|x| x.split_once('-').unwrap()).collect();
        let mut input = HashMap::new();
        for (a, b) in edges {
            input.entry(a).or_insert_with(Vec::new).push(b);
            input.entry(b).or_insert_with(Vec::new).push(a);
        }
        Self { input }
    }
}

fn visit<'a, F>(
    skip: F,
    map: &Input<'a>,
    paths: &mut Vec<Vec<&'a str>>,
    partial: &mut Vec<&'a str>,
    node: &'a str,
) where
    F: Fn(&mut Vec<&'a str>, &'a str) -> bool + Copy,
{
    if skip(partial, node) {
        return;
    }
    if node == "end" {
        let mut path = partial.clone();
        path.push("end");
        paths.push(path);
        return;
    }
    partial.push(node);
    for next in &map[&node] {
        visit(skip, map, paths, partial, next);
    }
    let _ = partial.pop();
}

impl<'a> crate::Day for Day<'a> {
    fn part1(&self) -> String {
        let mut paths = Vec::new();
        let mut partial = Vec::new();
        visit(
            |partial, node| node.chars().any(char::is_lowercase) && partial.contains(&node),
            &self.input,
            &mut paths,
            &mut partial,
            "start",
        );
        paths.len().to_string()
    }

    fn part2(&self) -> String {
        let mut paths = Vec::new();
        let mut partial = Vec::new();
        visit(
            |partial, node| {
                let mut seen = HashMap::new();
                for visited in partial.iter() {
                    *seen.entry(visited).or_insert(0) += 1;
                }
                let seen_this = *seen.get(&node).unwrap_or(&0);
                let has_lower = node.chars().any(char::is_lowercase);
                let any_twice = seen
                    .iter()
                    .filter_map(|(n,v)| n.chars().any(char::is_lowercase).then(||v))
                    .any(|v| *v >= 2);
                let second_start = seen.contains_key(&"start") && node == "start";
                second_start || seen_this > 0 && has_lower && any_twice
            },
            &self.input,
            &mut paths,
            &mut partial,
            "start",
        );
        paths.len().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Day as _;

    #[test]
    fn part1_1_test() {
        let input = concat![
            "start-A\n",
            "start-b\n",
            "A-c\n",
            "A-b\n",
            "b-d\n",
            "A-end\n",
            "b-end",
        ];
        let day = Day::from_str(input);
        assert_eq!("10", day.part1());
    }

    #[test]
    fn part1_2_test() {
        let input = concat![
            "dc-end\n",
            "HN-start\n",
            "start-kj\n",
            "dc-start\n",
            "dc-HN\n",
            "LN-dc\n",
            "HN-end\n",
            "kj-sa\n",
            "kj-HN\n",
            "kj-dc",
        ];
        let day = Day::from_str(input);
        assert_eq!("19", day.part1());
    }

    #[test]
    fn part1_3_test() {
        let input = concat![
            "fs-end\n",
            "he-DX\n",
            "fs-he\n",
            "start-DX\n",
            "pj-DX\n",
            "end-zg\n",
            "zg-sl\n",
            "zg-pj\n",
            "pj-he\n",
            "RW-he\n",
            "fs-DX\n",
            "pj-RW\n",
            "zg-RW\n",
            "start-pj\n",
            "he-WI\n",
            "zg-he\n",
            "pj-fs\n",
            "start-RW",
        ];
        let day = Day::from_str(input);
        assert_eq!("226", day.part1());
    }

    #[test]
    fn part2_1_test() {
        let input = concat![
            "start-A\n",
            "start-b\n",
            "A-c\n",
            "A-b\n",
            "b-d\n",
            "A-end\n",
            "b-end",
        ];
        let day = Day::from_str(input);
        assert_eq!("36", day.part2());
    }

    #[test]
    fn part2_2_test() {
        let input = concat![
            "dc-end\n",
            "HN-start\n",
            "start-kj\n",
            "dc-start\n",
            "dc-HN\n",
            "LN-dc\n",
            "HN-end\n",
            "kj-sa\n",
            "kj-HN\n",
            "kj-dc",
        ];
        let day = Day::from_str(input);
        assert_eq!("103", day.part2());
    }

    #[test]
    fn part2_3_test() {
        let input = concat![
            "fs-end\n",
            "he-DX\n",
            "fs-he\n",
            "start-DX\n",
            "pj-DX\n",
            "end-zg\n",
            "zg-sl\n",
            "zg-pj\n",
            "pj-he\n",
            "RW-he\n",
            "fs-DX\n",
            "pj-RW\n",
            "zg-RW\n",
            "start-pj\n",
            "he-WI\n",
            "zg-he\n",
            "pj-fs\n",
            "start-RW",
        ];
        let day = Day::from_str(input);
        assert_eq!("3509", day.part2());
    }
}
