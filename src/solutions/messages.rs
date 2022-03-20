use crate::solution_template::Solution;
use crate::solutions::messages::MessageRule::{Definition, Pointer};
use crate::solutions::messages::Rule::{Either, Only};
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::BTreeMap;

lazy_static! {
    /// The pattern used by definition rules.
    static ref DEFINITION_PATTERN: Regex = Regex::new(r#""(\w)""#).unwrap();
}

pub struct MessageSolution {
    rules: Vec<MessageRule>,
    messages: Vec<String>,
}

/// The elves sent you some messages. They got corrupted and for
/// some reason, they can send you a list of rules explaining how
/// the messages should look, but they can't just resend the rules
/// themselves. You have to figure out which ones are valid.
impl Solution for MessageSolution {
    type Data = Self;
    type Output = usize;

    const MESSAGE_A: &'static str = "Number matching rule 0";
    const MESSAGE_B: &'static str = "Number after update";

    fn from_string(s: &str) -> Self {
        let mut split = s.split("\n\n");
        let rules = split.next().expect("No content.");
        let messages = split.next().expect("No messages.");
        Self {
            rules: MessageRule::parse_all(rules),
            messages: Self::map_lines(messages, str::to_string),
        }
    }

    /// Find the number of messages matching rule 0.
    fn get_solution_a(data: &Self) -> Option<usize> {
        let pat = format!("^{}$", &data.rules[0].expand(&data.rules));
        let regex = Regex::new(&pat).expect(&pat);
        let matches = data.messages.iter().filter(|m| regex.is_match(m)).count();
        Some(matches)
    }

    /// It turns out, these two lines are wrong. Replace them.
    /// 8: 42 | 42 8
    /// 11: 42 31 | 42 11 31
    /// Deduction:
    /// 8: 42+
    /// 11: 42 (42 (42 (42 ... 31)? 31)? 31)? 31
    /// 0: 42+ (42)n (31)n
    /// Reference:
    /// 42: 20 51 | 39 120
    /// 31: 39 43 | 20 118
    /// 0: 8 11
    fn get_solution_b(data: &Self) -> Option<usize> {
        // Selected this number because it was the smallest
        // number that stopped producing changes.
        const HACK_DEPTH: usize = 5;
        let r42 = data.rules[42].expand(&data.rules);
        let r31 = data.rules[31].expand(&data.rules);
        // Regex doesn't support recursion. Bit of a hack, here.
        // Start with at least one 42 for 8.
        let mut exp_0 = String::from("^(");
        exp_0.push_str(&r42);
        exp_0.push_str(")+");
        // Finish with "recursive" `42(?R)31`s.
        exp_0.push_str(&r42);
        for _ in 0..HACK_DEPTH {
            exp_0.push('(');
            exp_0.push_str(&r42);
        }
        for _ in 0..HACK_DEPTH {
            exp_0.push_str(&r31);
            exp_0.push_str(")?")
        }
        exp_0.push_str(&r31);
        exp_0.push('$');

        let regex = Regex::new(&exp_0).expect(&exp_0);
        let matches = data.messages.iter().filter(|m| regex.is_match(m)).count();
        Some(matches)
    }
}

// Cloning due to borrow problems.
#[derive(Debug, Clone)]
pub enum MessageRule {
    Definition(String),
    Pointer(Rule<Vec<usize>>),
}

#[derive(Debug, Clone)]
pub enum Rule<T: Clone> {
    Either(T, T),
    Only(T),
}

impl MessageRule {
    fn expand(&self, rules: &Vec<MessageRule>) -> String {
        let ps = match self {
            Pointer(p) => p,
            Definition(s) => return s.clone(),
        };
        let mut exp = String::new();
        Self::fill_recursively(&mut exp, ps, rules);
        exp
    }

    fn fill_recursively(chars: &mut String, ps: &Rule<Vec<usize>>, rules: &Vec<MessageRule>) {
        match ps {
            Either(vec_a, vec_b) => {
                chars.push('(');
                for &idx in vec_a {
                    match &rules[idx] {
                        Pointer(r) => Self::fill_recursively(chars, r, rules),
                        Definition(s) => chars.push_str(s),
                    }
                }
                chars.push('|');
                for &idx in vec_b {
                    match &rules[idx] {
                        Pointer(r) => Self::fill_recursively(chars, r, rules),
                        Definition(s) => chars.push_str(s),
                    }
                }
                chars.push(')');
            }
            Only(vec_a) => {
                for &idx in vec_a {
                    match &rules[idx] {
                        Pointer(r) => Self::fill_recursively(chars, r, rules),
                        Definition(s) => chars.push_str(s),
                    }
                }
            }
        }
    }

    fn parse(s: &str) -> Option<(usize, Self)> {
        let mut split = s.split(": ");
        let idx: usize = split.next()?.parse().ok()?;
        let txt = split.next()?;
        if DEFINITION_PATTERN.is_match(txt) {
            return Some((idx, Self::parse_definition(txt)?));
        }
        Some((idx, Self::parse_rule(txt)?))
    }

    fn parse_definition(s: &str) -> Option<Self> {
        let captures = DEFINITION_PATTERN.captures(s)?;
        let ch = captures.get(1)?;
        Some(Definition(ch.as_str().to_string()))
    }

    fn parse_rule(s: &str) -> Option<Self> {
        let mut vec: Vec<Vec<usize>> = Vec::new();
        let mut ps = s.split(" | ");
        while let Some(p) = ps.next() {
            let mut nums = p.split_whitespace();
            let mut inner = Vec::new();
            while let Some(n) = nums.next() {
                inner.push(n.parse().ok()?);
            }
            vec.push(inner);
        }
        match vec.len() {
            1 => Some(Pointer(Only(vec[0].clone()))),
            2 => Some(Pointer(Either(vec[0].clone(), vec[1].clone()))),
            _ => panic!("Doesn't match rules: {}", s),
        }
    }

    fn parse_all(s: &str) -> Vec<Self> {
        // Using a BTreeMap to sort by key.
        let mut map = BTreeMap::new();
        for line in s.lines() {
            let (idx, rule) = Self::parse(line).expect(line);
            map.insert(idx, rule);
        }
        map.values().cloned().collect() // :(
    }
}

#[test]
fn test_solution_a() {
    // too much work today
}

#[test]
fn test_solution_b() {
    // assert_eq!(None, MessageSolution::get_solution_b(&""))
}
