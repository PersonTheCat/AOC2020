use crate::solution_template::Solution;
use lazy_static::lazy_static;
use regex::Regex;
use std::borrow::Borrow;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};

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
    /// to verify a given bag's contents recursively. This is a
    /// very substantial performance improvement.
    type Data = HashSet<ColoredBag>;
    type Output = usize;

    const MESSAGE_A: &'static str = "Number containing gold";
    const MESSAGE_B: &'static str = "Number gold contains";

    fn from_string(s: &str) -> HashSet<ColoredBag> {
        s.lines().map(|l| ColoredBag::parse(l).expect(l)).collect()
    }

    /// Count the number of bags that can hold "shiny gold" bags.
    fn get_solution_a(data: &HashSet<ColoredBag>) -> Option<usize> {
        let count = data
            .iter()
            .filter(|bag| bag.can_hold("shiny gold", data))
            .count();
        Some(count)
    }

    /// Count the number of bags that "shiny gold" bags can hold.
    fn get_solution_b(data: &HashSet<ColoredBag>) -> Option<usize> {
        let gold = data.get("shiny gold").unwrap();
        Some(gold.count_contents(data))
    }
}

/// Rules for any given color bag and which colors it must contain.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ColoredBag {
    color: String,
    contains: Vec<BagRule>,
}

impl ColoredBag {
    /// Create new rules for a given color with contents.
    fn new(name: &str, contains: Vec<BagRule>) -> Self {
        Self {
            color: name.to_string(),
            contains,
        }
    }

    /// Create new rules for a given color before establishing its contents.
    fn new_empty(name: &str) -> Self {
        Self {
            color: name.to_string(),
            contains: Vec::new(),
        }
    }

    /// Attempts to parse a ColoredBag rule from the expected format,
    /// e.g. `<name> bags contain <#> <name1> bag(s), <#> <name2> bag(s)`.
    fn parse(s: &str) -> Option<Self> {
        let mut rule_split = s.split(" bags contain ");
        let owner = rule_split.next()?.trim();
        let contains = rule_split
            .next()?
            .split(", ")
            .flat_map(BagRule::parse)
            .collect();
        let bag = Self::new(owner, contains);
        Some(bag)
    }

    /// Fills the contents of this bag using the rules in `all`.
    #[allow(unused)] // Just in case we do need to fill out completely.
    fn fill_from(&mut self, source: &HashSet<ColoredBag>) {
        if !self.contains.is_empty() {
            panic!("{} was already full.", self.color);
        }
        let me = source.get(&self.color).unwrap();
        for bag in &me.contains {
            let mut clone = bag.clone();
            clone.bag.fill_from(source);
            self.contains.push(clone);
        }
    }

    /// Recursively determines whether this bag can hold another bag
    /// with the given name using the top-level source data.
    /// Apparently, this is very inefficient.
    fn can_hold(&self, name: &str, source: &HashSet<ColoredBag>) -> bool {
        for rule in &self.contains {
            if rule.bag.color == name {
                return true;
            }
            let from_source = source.get(&rule.bag.color).unwrap();
            if from_source.can_hold(name, source) {
                return true;
            }
        }
        false
    }

    /// Counts the number of bags inside of `self` recursively.
    fn count_contents(&self, source: &HashSet<ColoredBag>) -> usize {
        let mut count = 0;
        for rule in &self.contains {
            count += rule.quantity as usize;
            let from_source = source.get(&rule.bag.color).unwrap();
            count += rule.quantity as usize * from_source.count_contents(source)
        }
        count
    }
}

/// It's quicker to hash by color, since the rules are unique to each
/// color. This implementation also allows bags to be retrieved from
/// a HashSet by their color alone, so that no mapping is needed.
impl Hash for ColoredBag {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.color.hash(state)
    }
}

/// Allows bags to be retrieved by string slices. Mainly for testing.
impl Borrow<str> for ColoredBag {
    fn borrow(&self) -> &str {
        &self.color
    }
}

/// Allows bags to be retrieved by regular string references, i.e. by
/// another bag's color.
impl Borrow<String> for ColoredBag {
    fn borrow(&self) -> &String {
        &self.color
    }
}

/// A rule defining a specific quantity of bags.
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct BagRule {
    quantity: i8,
    bag: ColoredBag,
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
            bag: ColoredBag::new_empty(color),
        };
        Some(rule)
    }
}

#[test]
fn test_can_hold() {
    let rules = "red bags contain 3 green bags, 1 blue bag.
        brown bags contain 2 red bags.
        blue bags contain no other bags.
        green bags contain no other bags.";
    let bags = BagSolution::from_string(rules);
    let brown = bags.get("brown").unwrap();
    let red = bags.get("red").unwrap();
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

#[test]
fn test_bag_hash() {
    let rule = "test bags contain no other bags.";
    let mut set = HashSet::new();
    set.insert(ColoredBag::parse(rule).unwrap());
    assert!(set.get("test").is_some());
}
