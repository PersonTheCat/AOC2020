use crate::solution_template::Solution;
use crate::solutions::rain::Direction::*;

pub struct RainSolution;

/// Explain the rules of this day
impl Solution for RainSolution {
    type Data = Vec<Movement>;
    type Output = i64;

    const MESSAGE_A: &'static str = "Manhattan distance (self)";
    const MESSAGE_B: &'static str = "Manhattan distance (waypoint)";

    fn from_string(s: &str) -> Vec<Movement> {
        Self::map_lines(s, |l| Movement::parse(l).expect(l))
    }

    /// Explain solution A
    fn get_solution_a(data: &Vec<Movement>) -> Option<i64> {
        let (ns, ew) = Movement::tally(data);
        Some(ns.abs() + ew.abs())
    }

    /// Explain solution B
    fn get_solution_b(data: &Vec<Movement>) -> Option<i64> {
        let (ns, ew) = Movement::tally_waypoint(data);
        Some(ns.abs() + ew.abs())
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Direction {
    North,
    South,
    East,
    West,
    Left,
    Right,
    Forward,
}

impl Direction {
    fn right(&self, degrees: i64) -> Self {
        if degrees % 90 != 0 {
            panic!("You can only rotate by 90-degree increments.")
        }
        let mut direction = self.clone();
        for _ in 0..(degrees / 90) {
            direction = match direction {
                North => East,
                South => West,
                East => South,
                West => North,
                _ => panic!("You can only rotate from cardinals."),
            }
        }
        direction
    }

    fn left(&self, degrees: i64) -> Self {
        if degrees % 90 != 0 {
            panic!("You can only rotate by 90-degree increments.")
        }
        let mut direction = self.clone();
        for _ in 0..(degrees / 90) {
            direction = match direction {
                North => West,
                South => East,
                East => North,
                West => South,
                _ => panic!("You can only rotate from cardinals."),
            }
        }
        direction
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Waypoint {
    ns: i64,
    ew: i64,
}

impl Waypoint {
    // R90 (𝑥,𝑦) -> (𝑦,−𝑥)
    fn right(&self, degrees: i64) -> Self {
        if degrees % 90 != 0 {
            panic!("You can only rotate by 90-degree increments.")
        }
        let mut wp = self.clone();
        for _ in 0..(degrees / 90) {
            wp = Self {
                ns: -wp.ew,
                ew: wp.ns,
            }
        }
        wp
    }

    // L90 (𝑥,𝑦) -> (−𝑦,𝑥)
    fn left(&self, degrees: i64) -> Self {
        if degrees % 90 != 0 {
            panic!("You can only rotate by 90-degree increments.")
        }
        let mut wp = self.clone();
        for _ in 0..(degrees / 90) {
            wp = Self {
                ns: wp.ew,
                ew: -wp.ns,
            }
        }
        wp
    }
}

pub struct Movement {
    dir: Direction,
    amount: i64,
}

impl Movement {
    fn parse(s: &str) -> Option<Self> {
        if s.len() <= 1 {
            return None;
        }
        let direction = &s[0..1];
        let amount = s[1..].parse::<i64>().ok()?;
        let dir = match direction {
            "N" => North,
            "S" => South,
            "E" => East,
            "W" => West,
            "L" => Left,
            "R" => Right,
            "F" => Forward,
            _ => return None,
        };
        Some(Self { dir, amount })
    }

    fn tally(vec: &Vec<Movement>) -> (i64, i64) {
        let mut ns = 0;
        let mut ew = 0;
        let mut facing = East;
        for &Self { dir, amount } in vec {
            match dir {
                Left => facing = facing.left(amount),
                Right => facing = facing.right(amount),
                Forward => {
                    let mov = Self::translate(facing, amount);
                    ns += mov.0;
                    ew += mov.1;
                }
                _ => {
                    let mov = Self::translate(dir, amount);
                    ns += mov.0;
                    ew += mov.1;
                }
            }
        }
        (ns, ew)
    }

    fn tally_waypoint(vec: &Vec<Movement>) -> (i64, i64) {
        let mut ns = 0;
        let mut ew = 0;
        let mut waypoint = Waypoint { ns: 1, ew: 10 };
        for &Self { dir, amount } in vec {
            match dir {
                Left => waypoint = waypoint.left(amount),
                Right => waypoint = waypoint.right(amount),
                Forward => {
                    ns += waypoint.ns * amount;
                    ew += waypoint.ew * amount;
                }
                North => waypoint.ns += amount,
                South => waypoint.ns -= amount,
                East => waypoint.ew += amount,
                West => waypoint.ew -= amount,
            }
        }
        (ns, ew)
    }

    fn translate(dir: Direction, amount: i64) -> (i64, i64) {
        match dir {
            North => (amount, 0),
            South => (-amount, 0),
            East => (0, amount),
            West => (0, -amount),
            _ => panic!("Only cardinals equate to raw movements."),
        }
    }
}

#[test]
fn test_solution_a() {
    let example = "F10\nN3\nF7\nR90\nF11";
    let data = RainSolution::from_string(example);
    assert_eq!(RainSolution::get_solution_a(&data).unwrap(), 25)
}

#[test]
fn test_solution_b() {
    let example = "F10\nN3\nF7\nR90\nF11";
    let data = RainSolution::from_string(example);
    assert_eq!(RainSolution::get_solution_b(&data).unwrap(), 286)
}

#[test]
fn test_rotate_waypoint() {
    let waypoint = Waypoint { ew: 10, ns: 1 };
    assert_eq!(waypoint.right(90), Waypoint { ew: 1, ns: -10 });
    assert_eq!(waypoint.left(90), Waypoint { ew: -1, ns: 10 });
    assert_eq!(waypoint.right(90).left(90), waypoint);
    assert_eq!(waypoint.right(360), waypoint);
    assert_eq!(waypoint.left(360), waypoint);
}
