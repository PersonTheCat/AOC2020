use crate::solution_template::Solution;

struct EmptySolution;

/// Explain the rules of this day
impl Solution for EmptySolution {
    type Data = &'static str;
    type Output = i32;

    const MESSAGE_A: &'static str = "";
    const MESSAGE_B: &'static str = "";

    fn from_string(_: &str) -> &'static str {
        "output"
    }

    /// Explain solution A
    fn get_solution_a(_: &&'static str) -> Option<i32> {
        None
    }

    /// Explain solution B
    fn get_solution_b(_: &&'static str) -> Option<i32> {
        None
    }
}

#[test]
fn test_solution_a() {
    assert_eq!(None, EmptySolution::get_solution_a(&""))
}

#[test]
fn test_solution_b() {
    assert_eq!(None, EmptySolution::get_solution_b(&""))
}
