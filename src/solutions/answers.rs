use crate::solution_template::Solution;
use std::collections::HashSet;

pub struct AnswerSolution {
    /// Maps to each raw cluster.
    raw: Vec<String>,
    /// Maps to every unique char in each cluster.
    set: Vec<HashSet<char>>,
}

/// Passengers on a plane respond in groups to a survey. Count the number of
/// unique responses in each group.
impl Solution for AnswerSolution {
    type Data = Self;
    type Output = usize;

    const MESSAGE_A: &'static str = "Unique yes in group";
    const MESSAGE_B: &'static str = "Common yes in group";

    fn from_string(s: &str) -> Self {
        Self {
            raw: Self::map_clusters(s, str::to_string),
            set: Self::map_clusters(s, |c| c.chars().filter(|ch| *ch != '\n').collect()),
        }
    }

    /// Count the number of unique responses in each set.
    fn get_solution_a(data: &Self) -> Option<usize> {
        Some(data.set.iter().map(|c| c.len()).sum())
    }

    /// Count the number of common responses in each set.
    fn get_solution_b(data: &Self) -> Option<usize> {
        let num = data
            .raw
            .iter()
            .zip(data.set.iter())
            .map(|(raw, set)| count_common(raw, set))
            .sum();
        Some(num)
    }
}

/// Count the number of characters that exists in each line.
fn count_common(raw: &str, set: &HashSet<char>) -> usize {
    set.iter()
        .filter(|ch| each_line_contains(raw, **ch))
        .count()
}

/// Determines whether each line in `raw` contains `ch`.
fn each_line_contains(raw: &str, ch: char) -> bool {
    raw.lines().all(|s| s.contains(ch))
}

#[test]
fn test_solution_a() {
    let example = "abc\n\na\nb\nc\n\nab\nac\n\na\na\na\na\n\nb";
    let data = AnswerSolution::from_string(example);
    assert_eq!(AnswerSolution::get_solution_a(&data).unwrap(), 11);
}

#[test]
fn test_solution_b() {
    let example = "abc\n\na\nb\nc\n\nab\nac\n\na\na\na\na\n\nb";
    let data = AnswerSolution::from_string(example);
    assert_eq!(AnswerSolution::get_solution_b(&data).unwrap(), 6);
}
