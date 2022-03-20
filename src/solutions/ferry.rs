use crate::solution_template::Solution;
use crate::solutions::ferry::SeatStatus::*;
use std::fmt::{Debug, Formatter};

pub struct FerrySolution;

/// People are taking seats on a ferry. They take seats on predictable increments
/// in predictable places based on a set of rules.
impl Solution for FerrySolution {
    type Data = Vec<Vec<SeatStatus>>;
    type Output = i32;

    const MESSAGE_A: &'static str = "Final number occupied (surrounding)";
    const MESSAGE_B: &'static str = "Final number occupied (direction)";

    fn from_string(s: &str) -> Vec<Vec<SeatStatus>> {
        Self::map_chars(s, |c| SeatStatus::parse(c).expect(&c.to_string()))
    }

    /// Figure out how many seats are occupied when no more seats can be taken.
    fn get_solution_a(data: &Vec<Vec<SeatStatus>>) -> Option<i32> {
        // Using clones so we don't tamper with the conditions.
        let mut matrix = data.clone();
        loop {
            let (m, count) = update_once(&matrix);
            matrix = m;
            if count == 0 {
                break;
            }
        }
        Some(total_occupied(&matrix))
    }

    /// Check in the surrounding *directions* for occupied seats.
    fn get_solution_b(data: &Vec<Vec<SeatStatus>>) -> Option<i32> {
        let mut matrix = data.clone();
        loop {
            let (m, count) = update_once_direction(&matrix);
            matrix = m;
            if count == 0 {
                break;
            }
        }
        Some(total_occupied(&matrix))
    }
}

fn update_once(matrix: &Vec<Vec<SeatStatus>>) -> (Vec<Vec<SeatStatus>>, i32) {
    let mut clone = matrix.clone();
    let mut num_changed = 0;
    for y in 0..matrix.len() {
        for x in 0..matrix[y].len() {
            let occupied = check_surrounding(&matrix, x, y);
            let seat = &matrix[y][x];
            if *seat == Vacant && occupied == 0 {
                clone[y][x] = Occupied;
                num_changed += 1;
            } else if *seat == Occupied && occupied >= 4 {
                clone[y][x] = Vacant;
                num_changed += 1;
            }
        }
    }
    (clone, num_changed)
}

fn check_surrounding(matrix: &Vec<Vec<SeatStatus>>, x: usize, y: usize) -> i32 {
    // Avoid subtracting with overflow.
    let x = x as isize;
    let y = y as isize;
    let positions = [
        (x + 1, y + 1),
        (x + 1, y),
        (x, y + 1),
        (x - 1, y - 1),
        (x - 1, y),
        (x, y - 1),
        (x + 1, y - 1),
        (x - 1, y + 1),
    ];
    let mut occupied = 0;
    for &(x, y) in &positions {
        if x >= 0 && y >= 0 {
            occupied += check(matrix, x as usize, y as usize);
        }
    }
    occupied
}

fn check(matrix: &Vec<Vec<SeatStatus>>, x: usize, y: usize) -> i32 {
    if let Some(vec) = matrix.get(y) {
        if let Some(Occupied) = vec.get(x) {
            return 1;
        }
    }
    0
}

fn update_once_direction(matrix: &Vec<Vec<SeatStatus>>) -> (Vec<Vec<SeatStatus>>, i32) {
    let mut clone = matrix.clone();
    let mut num_changed = 0;
    for y in 0..matrix.len() {
        for x in 0..matrix[y].len() {
            let occupied = check_surrounding_directions(&matrix, x, y);
            let seat = &matrix[y][x];
            if *seat == Vacant && occupied == 0 {
                clone[y][x] = Occupied;
                num_changed += 1;
            } else if *seat == Occupied && occupied >= 5 {
                clone[y][x] = Vacant;
                num_changed += 1;
            }
        }
    }
    (clone, num_changed)
}

fn check_surrounding_directions(matrix: &Vec<Vec<SeatStatus>>, x: usize, y: usize) -> i32 {
    // Avoid subtracting with overflow.
    let slopes = [
        (1, 1),
        (1, 0),
        (0, 1),
        (-1, -1),
        (-1, 0),
        (0, -1),
        (1, -1),
        (-1, 1),
    ];
    let mut occupied = 0;
    for &(sx, sy) in &slopes {
        occupied += check_direction(matrix, (x, y), (sx, sy));
    }
    occupied
}

fn check_direction(matrix: &Vec<Vec<SeatStatus>>, i: (usize, usize), s: (isize, isize)) -> i32 {
    let mut x = i.0 as isize + s.0;
    let mut y = i.1 as isize + s.1;
    while x >= 0 && x < matrix[0].len() as isize && y >= 0 && y < matrix.len() as isize {
        match matrix[y as usize][x as usize] {
            Floor => {
                x += s.0;
                y += s.1;
                continue;
            }
            Occupied => return 1,
            Vacant => return 0,
        }
    }
    0
}

fn total_occupied(matrix: &Vec<Vec<SeatStatus>>) -> i32 {
    let mut total = 0;
    for vec in matrix {
        for seat in vec {
            if let Occupied = *seat {
                total += 1;
            }
        }
    }
    total
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum SeatStatus {
    Occupied,
    Vacant,
    Floor,
}

impl SeatStatus {
    fn parse(c: char) -> Option<SeatStatus> {
        match c {
            'L' => Some(Vacant),
            '#' => Some(Occupied),
            '.' => Some(Floor),
            _ => None,
        }
    }
}

impl Debug for SeatStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Occupied => write!(f, "#"),
            Vacant => write!(f, "L"),
            Floor => write!(f, "."),
        }
    }
}

#[test]
fn test_solution_a() {
    assert_eq!(FerrySolution::get_solution_a(&get_example()).unwrap(), 37)
}

#[test]
fn test_solution_b() {
    assert_eq!(FerrySolution::get_solution_b(&get_example()).unwrap(), 26)
}

#[test]
fn test_update_direction() {
    let example = ".......#.
        ...#.....
        .#.......
        .........
        ..#L....#
        ....#....
        .........
        #........
        ...#.....";
    let example = example.replace(" ", "");
    let data = FerrySolution::from_string(&example);
    let surrounding = check_surrounding_directions(&data, 3, 4);
    assert_eq!(surrounding, 8);
}

#[cfg(test)]
fn get_example() -> Vec<Vec<SeatStatus>> {
    let example = "L.LL.LL.LL
        LLLLLLL.LL
        L.L.L..L..
        LLLL.LL.LL
        L.LL.LL.LL
        L.LLLLL.LL
        ..L.L.....
        LLLLLLLLLL
        L.LLLLLL.L
        L.LLLLL.LL";
    FerrySolution::from_string(&example.replace(" ", ""))
}
