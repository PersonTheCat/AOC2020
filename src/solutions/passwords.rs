use crate::solution_template::Solution;
use lazy_static::lazy_static;
use regex::{Captures, Regex};
use std::convert::TryFrom;

lazy_static! {
    /// The pattern used to verify passwords for the second challenge.
    static ref PASSWORD_PATTERN: Regex = Regex::new(r"(\d{1,2})-(\d{1,2})\s(\S):\s(.*)").unwrap();
}

pub struct PasswordSolution;

/// Day 2 challenge: Count the number of valid passwords according to
/// criteria defined in a list.
impl Solution for PasswordSolution {
    type Data = Vec<PasswordData>;
    type Output = usize;

    const MESSAGE_A: &'static str = "Password data (cnt)";
    const MESSAGE_B: &'static str = "Password data (pos)";

    fn from_string(s: &str) -> Vec<PasswordData> {
        Self::map_lines(s, |line| PasswordData::try_from(line).expect(line))
    }

    /// Counts the number of valid passwords by length.
    fn get_solution_a(data: &Vec<PasswordData>) -> Option<usize> {
        Some(data.iter().filter(|p| test_password(*p)).count())
    }

    /// Counts the number of valid passwords by position.
    fn get_solution_b(data: &Vec<PasswordData>) -> Option<usize> {
        Some(data.iter().filter(|p| test_password_p(*p)).count())
    }
}

/// Determines whether a given password is valid based on a quantity of characters.
fn test_password(p: &PasswordData) -> bool {
    let count = p.password.chars().filter(|c| *c == p.ch).count() as i32;
    count >= p.min && count <= p.max
}

/// Determines whether a given password is valid based character positions (XOR).
fn test_password_p(p: &PasswordData) -> bool {
    if p.password.len() < p.max as usize {
        return false;
    }
    let first = p.password.chars().nth(p.min as usize - 1).unwrap();
    let second = p.password.chars().nth(p.max as usize - 1).unwrap();
    (first == p.ch) ^ (second == p.ch)
}

/// Contains information about one user's password and the conditions
/// it is supposed to meet. It would absolutely be more efficient to
/// produce a reference to the password slice and simply do an immediate
/// test with less memory allocation, but collecting these data into a
/// vector which will then be parsed is more consistent with how other
/// problems in this series will most likely be solved.
pub struct PasswordData {
    pub min: i32,
    pub max: i32,
    pub ch: char,
    pub password: String,
}

#[cfg(test)]
impl PasswordData {
    pub fn new(n1: i32, n2: i32, ch: char, s: &str) -> Self {
        Self {
            min: n1,
            max: n2,
            ch,
            password: s.to_string(),
        }
    }
}

impl TryFrom<&str> for PasswordData {
    type Error = ();

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let captures = PASSWORD_PATTERN.captures(s).ok_or(())?;

        Ok(Self {
            // Just ignore the specific errors. The pattern didn't match.
            min: get_capture(&captures, 1)?.parse().map_err(|_| ())?,
            max: get_capture(&captures, 2)?.parse().map_err(|_| ())?,
            ch: get_capture(&captures, 3)?.chars().next().ok_or(())?,
            password: get_capture(&captures, 4)?.to_string(),
        })
    }
}

/// Retrieves a capture by index as a `Result<&str, ()>` instead of an `Option<Match>`
fn get_capture<'a>(captures: &'a Captures, index: usize) -> Result<&'a str, ()> {
    Ok(captures.get(index).ok_or(())?.as_str())
}

#[test]
fn test_valid_password() {
    let valid1 = PasswordData::new(1, 3, 'f', "forest");
    let valid2 = PasswordData::new(2, 4, 'g', "going");
    let invalid1 = PasswordData::new(1, 2, 'l', "never");
    let invalid2 = PasswordData::new(1, 3, 'k', "bicycle");

    assert!(test_password(&valid1));
    assert!(test_password(&valid2));
    assert!(!test_password(&invalid1));
    assert!(!test_password(&invalid2));
}

#[test]
fn test_valid_password_p() {
    let valid1 = PasswordData::new(1, 3, 'b', "book");
    let valid2 = PasswordData::new(3, 7, 'o', "thought");
    let invalid1 = PasswordData::new(1, 2, 'l', "llama");
    let invalid2 = PasswordData::new(2, 3, 'f', "king");

    assert!(test_password_p(&valid1));
    assert!(test_password_p(&valid2));
    assert!(!test_password_p(&invalid1));
    assert!(!test_password_p(&invalid2));
}
