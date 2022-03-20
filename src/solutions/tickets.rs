use crate::solution_template::Solution;
use std::collections::HashMap;

pub struct TicketSolution {
    fields: Vec<TicketRule>,
    ticket: Vec<i64>,
    others: HashMap<Vec<i64>, Vec<i64>>,
}

/// You're on your way to yet another flight, but your ticket is in a
/// language you can't read. You somehow get access to a whole bunch
/// of tickets and use your data to determine a set of valid ranges
/// for each type of data. Use those ranges to determine which fields
/// are valid or invalid and which one is which.
///
/// These solutions get sloppier by the day...
impl Solution for TicketSolution {
    type Data = Self;
    type Output = i64;

    const MESSAGE_A: &'static str = "Sum of invalid fields";
    const MESSAGE_B: &'static str = "Product of departures";

    fn from_string(s: &str) -> Self {
        let mut fo = s.split("\n\nyour ticket:\n");
        let fields = fo.next().expect("No tickets.");
        let fields = TicketRule::parse_all(fields);
        let mut to = fo.next().unwrap().split("\n\nnearby tickets:\n");
        let ticket = read_ticket(&to.next().unwrap());
        let others = Self::map_lines(&to.next().unwrap(), read_ticket);

        Self {
            others: check_all(others, &fields),
            fields,
            ticket,
        }
    }

    /// Add the sum of each invalid field.
    /// This would have been a great place for a boolean array.
    fn get_solution_a(data: &Self) -> Option<i64> {
        Some(data.others.iter().map(|(_, i)| i.iter().sum::<i64>()).sum())
    }

    /// Figure out which field is which. Multiply the value of each
    /// field starting with "departure."
    /// Not super efficient. Doesn't have to be.
    fn get_solution_b(data: &Self) -> Option<i64> {
        let valid: Vec<&Vec<i64>> = data
            .others
            .iter()
            .filter(|(_, i)| i.is_empty())
            .map(|(v, _)| v)
            .collect();

        let mut rule_map: Vec<Option<&TicketRule>> = vec![None; data.fields.len()];
        for _ in 0..data.fields.len() {
            try_fill(&mut rule_map, &data.fields, &valid);
        }
        let mut product = 1;
        for (i, rule) in rule_map.into_iter().enumerate() {
            if rule?.key.starts_with("departure") {
                product *= data.ticket[i];
            }
        }
        Some(product)
    }
}

fn try_fill<'a>(
    map: &mut Vec<Option<&'a TicketRule>>,
    rules: &'a Vec<TicketRule>,
    tickets: &Vec<&Vec<i64>>,
) {
    for rule in rules {
        if map.contains(&Some(rule)) {
            continue;
        }
        let mut num_match = 0;
        let mut idx_match = 0;
        for i in 0..rules.len() {
            if map[i].is_some() {
                continue;
            }
            if tickets.iter().all(|v| rule.is_valid(v[i])) {
                num_match += 1;
                idx_match = i;
            }
        }
        if num_match == 1 {
            map[idx_match] = Some(rule);
            return;
        }
    }
}

fn read_ticket(s: &str) -> Vec<i64> {
    s.split(",").map(|s| s.parse().unwrap()).collect()
}

fn check_all(tickets: Vec<Vec<i64>>, rules: &Vec<TicketRule>) -> HashMap<Vec<i64>, Vec<i64>> {
    let mut checked = HashMap::new();
    for ticket in tickets {
        let mut invalid = Vec::new();
        for &field in &ticket {
            if !TicketRule::any_valid(rules, field) {
                invalid.push(field);
            }
        }
        checked.insert(ticket, invalid);
    }
    checked
}

#[derive(Debug, Eq, PartialEq)]
pub struct TicketRule {
    key: String,
    low: [i64; 2],
    high: [i64; 2],
}

impl TicketRule {
    fn parse(s: &str) -> Option<Self> {
        let mut kv = s.split(": ");
        let k = kv.next()?;
        let v = kv.next()?;
        let mut low = [0; 2];
        let mut high = [0; 2];
        let mut rs = v.split(" or ");
        let mut r0 = rs.next()?.split("-");
        let mut r1 = rs.next()?.split("-");
        low[0] = r0.next()?.parse().unwrap();
        low[1] = r0.next()?.parse().unwrap();
        high[0] = r1.next()?.parse().unwrap();
        high[1] = r1.next()?.parse().unwrap();

        let tr = Self {
            key: k.to_string(),
            low,
            high,
        };
        Some(tr)
    }

    fn parse_all(s: &str) -> Vec<Self> {
        s.lines().map(|l| TicketRule::parse(l).expect(l)).collect()
    }

    fn is_valid(&self, f: i64) -> bool {
        f >= self.low[0] && f <= self.low[1] || f >= self.high[0] && f <= self.high[1]
    }

    fn any_valid(rules: &Vec<Self>, f: i64) -> bool {
        for rule in rules {
            if rule.is_valid(f) {
                return true;
            }
        }
        false
    }
}

#[test]
fn test_solution_a() {
    let example = "class: 1-3 or 5-7\n\
        row: 6-11 or 33-44\n\
        seat: 13-40 or 45-50\n\n\
        your ticket:\n\
        7,1,14\n\n\
        nearby tickets:\n\
        7,3,47\n\
        40,4,50\n\
        55,2,20\n\
        38,6,12";
    let data = TicketSolution::from_string(example);
    assert_eq!(TicketSolution::get_solution_a(&data).unwrap(), 71)
}

#[test]
fn test_solution_b() {
    let example = "departure: 0-1 or 4-19\n\
        row: 0-5 or 8-19\n\
        seat: 0-13 or 16-19\n\n\
        your ticket:\n\
        11,12,13\n\n\
        nearby tickets:\n\
        3,9,18\n\
        15,1,5\n\
        5,14,9";
    let data = TicketSolution::from_string(example);
    assert_eq!(TicketSolution::get_solution_b(&data).unwrap(), 12)
}
