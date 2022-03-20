use crate::solution_template::Solution;

#[cfg(test)]
use std::{collections::HashSet, hash::Hash};

pub struct Sum2020Solution;

/// Day 1 challenge: Find the product of 2 | 3 numbers that sum to 2020.
impl Solution for Sum2020Solution {
    type Data = Vec<i32>;
    type Output = i32;

    const MESSAGE_A: &'static str = "2020 product (doubles)";
    const MESSAGE_B: &'static str = "2020 product (triples)";

    fn from_string(s: &str) -> Vec<i32> {
        Self::map_lines(s, |line| line.parse().unwrap())
    }

    /// In all unique pairs, find first sum of 2020 -> get product.
    fn get_solution_a(data: &Vec<i32>) -> Option<i32> {
        for (i, num1) in data.iter().enumerate() {
            for num2 in data[i + 1..].iter() {
                if num1 + num2 == 2020 {
                    return Some(num1 * num2);
                }
            }
        }
        None
    }

    /// In all unique triplets, find first sum of 2020 -> get product.
    fn get_solution_b(data: &Vec<i32>) -> Option<i32> {
        for i in 0..data.len() - 2 {
            for j in 1..data.len() - 1 {
                for k in 2..data.len() {
                    if &data[i] + &data[j] + &data[k] == 2020 {
                        return Some(&data[i] * &data[j] * &data[k]);
                    }
                }
            }
        }
        None
    }
}

#[test]
fn test_unique_doubles() {
    let nums = (0..50).collect::<Vec<i32>>();
    let mut unique = Vec::with_capacity(nums.len().pow(2));
    for (i, num1) in nums.iter().enumerate() {
        for num2 in nums[i + 1..].iter() {
            unique.push((num1, num2));
        }
    }
    assert!(all_unique(&unique));
}

#[test]
fn test_unique_triplets() {
    let nums = 0..50;
    let mut unique = Vec::with_capacity(nums.len().pow(3));
    for i in 0..nums.len() - 2 {
        for j in 1..nums.len() - 1 {
            for k in 2..nums.len() {
                unique.push((i, j, k));
            }
        }
    }
    assert!(all_unique(&unique));
}

#[test]
fn test_all_unique() {
    assert!(all_unique(&[1, 2, 3]));
    assert!(!all_unique(&[1, 1, 2]));
}

#[cfg(test)]
fn all_unique<T: Eq + Hash>(arr: &[T]) -> bool {
    let unique = arr.iter().collect::<HashSet<&T>>();
    arr.len() == unique.len()
}
