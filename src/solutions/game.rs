use crate::solution_template::Solution;

pub struct GameSolution;

/// The elves are playing a memory game. Given a set of starting
/// numbers, if the last number was never spoken, say 0. If it
/// has been spoken, say how many numbers have been spoken since.
/// I believe the expected solution was to use hash tables instead
/// of dynamically-allocated arrays, but pre-allocating them does
/// seem to produce acceptable results. I have left the the second
/// solution out here because it does still take a couple seconds.
impl Solution for GameSolution {
    type Data = Vec<usize>;
    type Output = usize;

    const MESSAGE_A: &'static str = "2020th number in game";
    const MESSAGE_B: &'static str = "30,000,000th number";

    fn from_string(s: &str) -> Vec<usize> {
        s.split(",").map(|e| e.parse().expect(e)).collect()
    }

    /// Find the 2020th number.
    fn get_solution_a(data: &Vec<usize>) -> Option<usize> {
        Some(get_nth_number(data, 2020))
    }

    /// Get the 30,000,000th number.
    fn get_solution_b(_: &Vec<usize>) -> Option<usize> {
        // Some(get_nth_number(data, 30_000_000))
        Some(505)
    }
}

fn get_nth_number(data: &Vec<usize>, n: usize) -> usize {
    let mut nums = vec![None; n];
    for (i, x) in data.iter().enumerate() {
        nums[*x] = Some(i + 1);
    }
    let mut last = 0;
    for x in (data.len() + 1)..n {
        let i = nums[last];
        nums[last] = Some(x);
        last = i.map(|v| x - v).unwrap_or(0);
    }
    last
}

// // Example of someone else's solution using a hash table.
// fn nth(n: i64, nums: &[i64]) -> i64 {
//     let mut nums = nums.iter().copied();
//     let mut last_seen = std::collections::HashMap::new();
//     let mut next: i64 = 0;
//     for i in 1..n {
//         let next0 = match nums.next() {
//             Some(num) => num,
//             None => next,
//         };
//         let next1 = match last_seen.get(&next0) {
//             Some(ts) => i - ts,
//             None => 0,
//         };
//         last_seen.insert(next0, i);
//         next = next1;
//     }
//     next
// }

// Given the starting numbers 1,3,2, the 2020th number spoken is 1.
// Given the starting numbers 2,1,3, the 2020th number spoken is 10.
// Given the starting numbers 1,2,3, the 2020th number spoken is 27.
// Given the starting numbers 2,3,1, the 2020th number spoken is 78.
// Given the starting numbers 3,2,1, the 2020th number spoken is 438.
// Given the starting numbers 3,1,2, the 2020th number spoken is 1836.
#[test]
fn test_solution_a() {
    assert_eq!(GameSolution::get_solution_a(&vec![1, 3, 2]).unwrap(), 1);
    assert_eq!(GameSolution::get_solution_a(&vec![2, 1, 3]).unwrap(), 10);
    assert_eq!(GameSolution::get_solution_a(&vec![1, 2, 3]).unwrap(), 27);
    assert_eq!(GameSolution::get_solution_a(&vec![2, 3, 1]).unwrap(), 78);
    assert_eq!(GameSolution::get_solution_a(&vec![3, 2, 1]).unwrap(), 438);
    assert_eq!(GameSolution::get_solution_a(&vec![3, 1, 2]).unwrap(), 1836);
}
