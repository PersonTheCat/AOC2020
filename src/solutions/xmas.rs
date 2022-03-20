use crate::solution_template::Solution;
use std::sync::Mutex;

pub struct XmasSolution {
    nums: Vec<i64>,
    a: Option<i64>,
}

/// eXchange-Masking Addition System. There's a preamble of 25 random
/// numbers. After that, every number should be the sum of two previous
/// numbers, as most 25 indices back. Find the number(s) that aren't.
impl Solution for XmasSolution {
    type Data = Mutex<Self>;
    type Output = i64;

    const MESSAGE_A: &'static str = "First number out of sequence";
    const MESSAGE_B: &'static str = "Min + max adding to number";

    fn from_string(s: &str) -> Mutex<Self> {
        let data = Self {
            nums: Self::map_lines(s, |l| l.parse().unwrap()),
            a: None,
        };
        Mutex::new(data)
    }

    /// Find the first number that isn't a sum of the previous two.
    fn get_solution_a(data: &Mutex<Self>) -> Option<i64> {
        let data = &mut data.lock().unwrap();
        for i in 25..data.nums.len() {
            let slice = &data.nums[(i - 25)..i];
            let current = data.nums[i];
            if let None = find_addends(current, slice) {
                data.a = Some(current);
                return Some(current);
            }
        }
        None
    }

    /// Find a range of at least two numbers that sum to solution a,
    /// then add the min and max.
    fn get_solution_b(data: &Mutex<Self>) -> Option<i64> {
        let data = data.lock().unwrap();
        let a = data.a?;
        for i in 0..data.nums.len() {
            if let Some(ref sequence) = find_addend_sequence(a, i, &data.nums) {
                return Some(add_min_max(sequence));
            }
        }
        None
    }
}

fn find_addends(sum: i64, slice: &[i64]) -> Option<(i64, i64)> {
    for num1 in slice {
        for num2 in slice {
            if num1 != num2 && num1 + num2 == sum {
                return Some((*num1, *num2));
            }
        }
    }
    None
}

fn find_addend_sequence(sum: i64, start: usize, slice: &[i64]) -> Option<Vec<i64>> {
    let mut vec = Vec::new();
    let mut total = 0;
    for i in start..slice.len() {
        vec.push(slice[i]);
        total += slice[i];
        if total == sum {
            return Some(vec);
        } else if total > sum {
            return None;
        }
    }
    None
}

fn add_min_max(sequence: &[i64]) -> i64 {
    sequence.iter().min().unwrap() + sequence.iter().max().unwrap()
}

#[test]
fn test_solution_a() {
    let mut nums = (1..=25).collect::<Vec<i64>>();
    nums.append(&mut vec![26, 49, 100]);
    let data = XmasSolution { a: None, nums };
    assert_eq!(
        XmasSolution::get_solution_a(&Mutex::new(data)).unwrap(),
        100
    )
}

#[test]
fn test_solution_b() {
    let mut nums = (1..=25).collect::<Vec<i64>>();
    nums.append(&mut vec![26, 49, 100]);
    let data = XmasSolution { a: Some(100), nums };
    // sum of 9 - 16 = 100; 9 + 16 = 25 (first sequence)
    assert_eq!(XmasSolution::get_solution_b(&Mutex::new(data)).unwrap(), 25)
}
