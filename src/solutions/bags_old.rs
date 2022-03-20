use crate::solution_template::Solution;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    /// The pattern used for parsing bag quantities.
    static ref BAG_RULE_PATTERN: Regex = Regex::new(r"^(\d)\s(.*)\sbags?\.?$").unwrap();
}

pub struct BagSolution;

/// Explain the rules of this day
impl Solution for BagSolution {
    /// Contains only the top-level instructions for each color.
    /// Filling this data out completely is very challenging in
    /// Rust. Instead, we keep a reference to this source material
    /// to verify a given bag's contents recursively.
    type Data = Vec<ColoredBag>;
    type Output = usize;

    const MESSAGE_A: &'static str = "Number containing gold";
    const MESSAGE_B: &'static str = "Number gold contains";

    fn from_string(s: &str) -> Vec<ColoredBag> {
        let vec = Self::map_lines(s, |l| ColoredBag::parse(l).expect(l));
        vec
    }

    /// Explain solution A
    fn get_solution_a(data: &Vec<ColoredBag>) -> Option<usize> {
        let count = data.iter()
            .filter(|bag| bag.can_hold("shiny gold", data))
            .count();
        Some(count)
    }

    /// Explain solution B
    fn get_solution_b(data: &Vec<ColoredBag>) -> Option<usize> {
        let gold = data.iter().find(|bag| &bag.color == "shiny gold").unwrap();
        Some(gold.count_contents(data))
    }
}

/// Rules for any given color bag and which colors it must contain.
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct ColoredBag {
    color: String,
    contains: Vec<BagRule>
}

impl ColoredBag {
    /// Create new rules for a given color with contents.
    fn new(name: &str, contains: Vec<BagRule>) -> Self {
        Self {
            color: name.to_string(),
            contains
        }
    }

    /// Create new rules for a given color before establishing its contents.
    fn new_empty(name: &str) -> Self {
        Self {
            color: name.to_string(),
            contains: Vec::new()
        }
    }

    /// Attempts to parse a ColoredBag rule from the expected format,
    /// e.g. `<name> bags contain <#> <name1> bag(s), <#> <name2> bag(s)`.
    fn parse(s: &str) -> Option<Self> {
        let mut rule_split = s.split(" bags contain ");
        let owner = rule_split.next()?.trim();
        let contains = rule_split.next()?
            .split(", ")
            .flat_map(BagRule::parse)
            .collect();
        let bag = Self::new(owner, contains);
        Some(bag)
    }

    /// Fills the contents of this bag using the rules in `all`.
    #[allow(unused)] // Just in case we do need to fill out completely.
    fn fill_from(&mut self, all: &Vec<ColoredBag>) {
        if !self.contains.is_empty() {
            panic!("{} was already full.", self.color);
        }
        let me = all.iter()
            .find(|b| b.color == self.color)
            .expect(&self.color);
        for bag in &me.contains {
            let mut clone = bag.clone();
            clone.bag.fill_from(all);
            self.contains.push(clone);
        }
    }

    /// Recursively determines whether this bag can hold another bag
    /// with the given name using the top-level source data.
    /// Apparently, this is very inefficient.
    fn can_hold(&self, name: &str, source: &Vec<ColoredBag>) -> bool {
        for rule in &self.contains {
            if rule.bag.color == name {
                return true;
            }
            for bag in source {
                // Find matching bag from self.contains in source.
                if rule.bag.color == bag.color && bag.can_hold(name, source) {
                    return true;
                }
            }
        }
        false
    }

    /// Counts the number of bags inside of `self` recursively.
    fn count_contents(&self, source: &Vec<ColoredBag>) -> usize {
        let mut count = 0;
        for rule in &self.contains {
            count += rule.quantity as usize;
            let from_source = source.iter()
                .find(|b| &b.color == &rule.bag.color)
                .unwrap();
            count += rule.quantity as usize * from_source.count_contents(source)
        }
        count
    }
}

/// A rule defining a specific quantity of bags.
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct BagRule {
    quantity: i8,
    bag: ColoredBag
}

impl BagRule {
    /// Attempts to parse a quantity and color from the expected format.
    /// e.g. `<#> <color>
    fn parse(s: &str) -> Option<Self> {
        if s == "no other bags." {
            return None;
        } else if !BAG_RULE_PATTERN.is_match(s) {
            panic!("{} did not match", s);
        }
        let captures = BAG_RULE_PATTERN.captures(s).unwrap();
        let color = captures.get(2).unwrap().as_str();
        let rule = BagRule {
            quantity: captures.get(1).unwrap().as_str().parse().unwrap(),
            bag: ColoredBag::new_empty(color)
        };
        Some(rule)
    }
}

// dim silver bags contain 2 shiny chartreuse bags, 4 dull magenta bags.
#[test]
fn test_can_hold() {
    let rules = "red bags contain 3 green bags, 1 blue bag.
        brown bags contain 2 red bags.
        blue bags contain no other bags.
        green bags contain no other bags.";
    let bags = BagSolution::from_string(rules);
    let brown = bags.iter().find(|bag| bag.color == "brown").unwrap();
    let red = bags.iter().find(|bag| bag.color == "red").unwrap();
    assert!(brown.can_hold(&red.color, &bags));
    assert!(!red.can_hold(&brown.color, &bags));
}

#[test]
fn test_solution_a() {
    let rules = "shiny gold bags contain 3 green bags, 1 blue bag.
        brown bags contain 2 shiny gold bags.
        blue bags contain no other bags.
        green bags contain no other bags.
        purple bags contain 3 brown bags";
    let bags = BagSolution::from_string(rules);
    assert_eq!(2, BagSolution::get_solution_a(&bags).unwrap())
}

#[test]
fn test_solution_b() {
    // shiny_gold[
    //     green[], green[], green[],
    //     blue[],
    //     pink[
    //         yellow[ sapphire[], sapphire[] ]
    //         yellow[ sapphire[], sapphire[] ]
    //     ]
    // ]
    let rules = "shiny gold bags contain 3 green bags, 1 blue bag, 1 pink bag.
        brown bags contain 2 shiny gold bags.
        blue bags contain no other bags.
        green bags contain no other bags.
        purple bags contain 3 brown bags
        pink bags contain 2 yellow bags.
        yellow bags contain 2 sapphire bags.
        sapphire bags contain no other bags.";
    let bags = BagSolution::from_string(rules);
    assert_eq!(11, BagSolution::get_solution_b(&bags).unwrap())
}
