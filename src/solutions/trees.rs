use crate::solution_template::Solution;

pub struct TreeSolution;

/// Day 3: count the number of trees in a matrix according to a given slope.
impl Solution for TreeSolution {
    type Data = Vec<Vec<bool>>;
    type Output = i64;

    const MESSAGE_A: &'static str = "Number of trees";
    const MESSAGE_B: &'static str = "Product of rows";

    fn from_string(s: &str) -> Vec<Vec<bool>> {
        Self::map_chars(s, is_tree)
    }

    /// Counts the number of trees encountered travelling a distance of three per row.
    fn get_solution_a(data: &Vec<Vec<bool>>) -> Option<i64> {
        Some(count_trees(data, 3, 1))
    }

    /// Multiplies the number of trees found in a predefined list of rows.
    fn get_solution_b(data: &Vec<Vec<bool>>) -> Option<i64> {
        let a = count_trees(data, 1, 1);
        let b = count_trees(data, 3, 1);
        let c = count_trees(data, 5, 1);
        let d = count_trees(data, 7, 1);
        let e = count_trees(data, 1, 2);
        println!("a: {}, b: {}, c: {}, d: {}, e: {}", a, b, c, d, e);
        Some(a * b * c * d * e)
    }
}

/// Counts the number of trees encountered according to a slope.
fn count_trees(matrix: &Vec<Vec<bool>>, over: usize, down: usize) -> i64 {
    let mut count = 0;
    let mut x = 0;
    let mut y = 0;
    while y < matrix.len() {
        let vec = &matrix[y];
        if vec[x % vec.len()] {
            count += 1;
        }
        x += over;
        y += down;
    }
    count
}

/// Determines whether a given character represents a tree.
fn is_tree(c: char) -> bool {
    match c {
        '.' => false,
        '#' => true,
        _ => panic!("Unexpected character: '{}'", c),
    }
}

#[test]
fn test_count_trees() {
    // (#). . # . . # # . . # . . # 1
    //  . # .(.)# . # . # . . # . # 0
    //  . . # . # #(.). . # . # # . 0
    //  # # . # . . . # #(.)# . . . 0
    //  # . . . # # . # . . . #(#). 1
    let trees1 = "#..#..#\n.#..#.#\n..#.##.\n##.#...\n#...##.";
    // (#). . . . . . # . . . . . . 1
    //  . . .(#). . . . . . # . . . 1
    //  . . . . . .(#). . . . . . # 1
    //  . . # . . . . . .(#). . . . 1
    //  . . . . . # . . . . . .(#). 1
    let trees2 = "#......\n...#...\n......#\n..#....\n.....#.";
    // (#). . . . . . # . . . . . . 1
    //  . . . . . . . . . . . . . . 0
    //  .(#). . . . . . # . . . . . 1
    //  . . . . . . . . . . . . . . 0
    //  . .(#). . . . . . # . . . . 1
    let trees3 = "#......\n.......\n.#.....\n.......\n..#....";

    let matrix1 = TreeSolution::from_string(trees1);
    let matrix2 = TreeSolution::from_string(trees2);
    let matrix3 = TreeSolution::from_string(trees3);
    assert_eq!(count_trees(&matrix1, 3, 1), 2);
    assert_eq!(count_trees(&matrix2, 3, 1), 5);
    assert_eq!(count_trees(&matrix3, 1, 2), 3);
}
