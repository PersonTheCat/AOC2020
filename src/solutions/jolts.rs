use crate::solution_template::Solution;

pub struct JoltSolution;

/// You have a bag of "jolt" adapters and need to find valid combinations
/// for charging your devices.
impl Solution for JoltSolution {
    type Data = Vec<u64>;
    type Output = u64;

    const MESSAGE_A: &'static str = "Diff 1s * Diff 3s";
    const MESSAGE_B: &'static str = "Number of valid combinations :c ";

    fn from_string(s: &str) -> Vec<u64> {
        let mut vec = Self::map_lines(s, |l| l.parse().expect(l));
        vec.sort();
        vec.push(*vec.last().unwrap() + 3);
        vec
    }

    /// Multiply the quantity of numbers with a difference of 1 by
    /// the quantity of numbers with a difference of 3.
    fn get_solution_a(data: &Vec<u64>) -> Option<u64> {
        let mut diff1 = 0;
        let mut diff3 = 0;
        let mut last = 0; // prepend 0
        for num in data {
            let diff = *num - last;
            match diff {
                1 => diff1 += 1,
                3 => diff3 += 1,
                _ => panic!("Unexpected gap of {}", diff),
            }
            last = *num;
        }
        Some(diff1 * diff3)
    }

    /// ??? No idea
    fn get_solution_b(data: &Vec<u64>) -> Option<u64> {
        // Solution copied from Reddit:
        let mut mem = Vec::with_capacity(data.len() + 1);
        mem.push((0, 1usize));

        for &num in data {
            let sum = mem
                .iter()
                .rev()
                .take(3)
                .take_while(|&&(c, _)| c + 3 >= num)
                .map(|&(_, s)| s)
                .sum();
            mem.push((num, sum));
        }
        // Take last sum
        Some(mem.last().unwrap().1 as u64)
    }
}

#[test]
fn test_solution_a() {
    let mut data1 = vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4, 22];
    let mut data2 = vec![
        28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35, 8,
        17, 7, 9, 4, 2, 34, 10, 3, 52,
    ];
    data1.sort();
    data2.sort();
    assert_eq!(JoltSolution::get_solution_a(&data1).unwrap(), 35);
    assert_eq!(JoltSolution::get_solution_a(&data2).unwrap(), 220);
}

#[test]
fn test_solution_b() {
    let mut data1 = vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4, 22];
    let mut data2 = vec![
        28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35, 8,
        17, 7, 9, 4, 2, 34, 10, 3, 52,
    ];
    data1.sort();
    data2.sort();
    // Undo the band-aid fix to get the normal answer here.
    assert_eq!(JoltSolution::get_solution_b(&data1).unwrap(), 8);
    assert_eq!(JoltSolution::get_solution_b(&data2).unwrap(), 19208);
}
