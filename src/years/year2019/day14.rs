use std::collections::HashMap;

pub struct DayGen;

impl crate::DayGen for DayGen {
    fn input<'a>(&self, input: &'a str) -> Box<dyn crate::Day + 'a> {
        Box::new(Day::from_str(input))
    }
}

type Recipes<'a> = HashMap<&'a str, Recipe<'a>>;
type Recipe<'a> = (Vec<Product<'a>>, Product<'a>);
type Product<'a> = (u64, &'a str);

struct Day<'a> {
    recipes: Recipes<'a>,
}

impl<'a> Day<'a> {
    pub fn from_str(input: &'a str) -> Self {
        let (_, recipes) = parsers::recipes(input).unwrap();
        assert_eq!(recipes.len(), input.lines().count());
        Self { recipes }
    }
    fn produce(&self, amount: u64) -> u64 {
        let mut to_produce: HashMap<&str, u64> = HashMap::from([(FUEL, amount)]);
        let mut spare: HashMap<&str, u64> = HashMap::new();
        while to_produce.keys().copied().filter(|p| *p != ORE).count() > 0 {
            let product = to_produce.keys().copied().find(|p| *p != ORE).unwrap();
            let mut amount = to_produce.remove(&product).unwrap();

            amount -= spare.remove(product).unwrap_or(0);

            let (required, (recipe_amount, _)) = &self.recipes[&product];
            let times = (amount + recipe_amount - 1) / recipe_amount;

            for (consumed_amount, consumed_product) in required {
                match (consumed_amount * times)
                    .overflowing_sub(spare.remove(consumed_product).unwrap_or(0))
                {
                    (over, true) => {
                        spare.insert(consumed_product, 0u64.wrapping_sub(over));
                    }
                    (required_amount, false) => {
                        *to_produce.entry(consumed_product).or_insert(0) += required_amount;
                    }
                }
            }
            let produced = times * recipe_amount;
            if produced > amount {
                spare.insert(product, produced - amount);
            }
            if produced < amount {
                unreachable!()
            }
        }
        to_produce[ORE]
    }
}

const FUEL: &str = "FUEL";
const ORE: &str = "ORE";
impl<'a> crate::Day for Day<'a> {
    fn part1(&self) -> String {
        self.produce(1).to_string()
    }

    fn part2(&self) -> String {
        const N_ORE: u64 = 1_000_000_000_000;
        let mut lower = 1;
        let mut upper = 2;
        while self.produce(upper) < N_ORE {
            lower = upper;
            upper *= 2;
        }
        while upper - lower > 1 {
            let middle = (upper + lower) / 2;
            if self.produce(middle) > N_ORE {
                upper = middle;
            } else {
                lower = middle;
            }
        }
        lower.to_string()
    }
}

mod parsers {
    use super::{Product, Recipe, Recipes};
    use nom::{
        bytes::complete::tag,
        character::complete::{alpha1, char, multispace0, u64},
        combinator::map,
        error::ParseError,
        multi::separated_list0,
        sequence::{delimited, separated_pair},
        IResult,
    };

    pub fn recipes(input: &str) -> IResult<&str, Recipes> {
        map(separated_list0(char('\n'), recipe), |recipes| {
            recipes
                .into_iter()
                .map(|recipe @ (_, (_, product))| (product, recipe))
                .collect()
        })(input)
    }

    fn recipe(input: &str) -> IResult<&str, Recipe> {
        separated_pair(products, tag(" => "), product)(input)
    }

    fn products(input: &str) -> IResult<&str, Vec<Product>> {
        separated_list0(ws(char(',')), product)(input)
    }

    fn product(input: &str) -> IResult<&str, Product> {
        separated_pair(u64, char(' '), alpha1)(input)
    }

    fn ws<'a, F, O, E>(inner: F) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
    where
        F: Fn(&'a str) -> IResult<&'a str, O, E> + 'a,
        E: ParseError<&'a str>,
    {
        delimited(multispace0, inner, multispace0)
    }
}

#[cfg(test)]
mod tests {
    use crate::Day as _;

    use super::*;

    #[test]
    fn part1_tiny_test() {
        let input = concat!(
            "9 ORE => 2 A\n",
            "8 ORE => 3 B\n",
            "7 ORE => 5 C\n",
            "3 A, 4 B => 1 AB\n",
            "5 B, 7 C => 1 BC\n",
            "4 C, 1 A => 1 CA\n",
            "2 AB, 3 BC, 4 CA => 1 FUEL\n",
        );
        let day = Day::from_str(input);
        assert_eq!("165", day.part1());
    }

    #[test]
    fn part1_larger1_test() {
        let input = concat!(
            "157 ORE => 5 NZVS\n",
            "165 ORE => 6 DCFZ\n",
            "44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL\n",
            "12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ\n",
            "179 ORE => 7 PSHF\n",
            "177 ORE => 5 HKGWZ\n",
            "7 DCFZ, 7 PSHF => 2 XJWVT\n",
            "165 ORE => 2 GPVTF\n",
            "3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT\n",
        );
        let day = Day::from_str(input);
        assert_eq!("13312", day.part1());
    }

    #[test]
    fn part1_larger2_test() {
        let input = concat!(
            "2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG\n",
            "17 NVRVD, 3 JNWZP => 8 VPVL\n",
            "53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL\n",
            "22 VJHF, 37 MNCFX => 5 FWMGM\n",
            "139 ORE => 4 NVRVD\n",
            "144 ORE => 7 JNWZP\n",
            "5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC\n",
            "5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV\n",
            "145 ORE => 6 MNCFX\n",
            "1 NVRVD => 8 CXFTF\n",
            "1 VJHF, 6 MNCFX => 4 RFSQX\n",
            "176 ORE => 6 VJHF\n",
        );
        let day = Day::from_str(input);
        assert_eq!("180697", day.part1());
    }
}
