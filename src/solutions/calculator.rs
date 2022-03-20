use crate::solution_template::Solution;

pub struct CalculatorSolution;

/// A child next to you on the plane asks for help with math homework.
/// The math involves a series of addition and multiplication expressions
/// which are calculated from left to right, ignoring order of operations.
impl Solution for CalculatorSolution {
    type Data = String;
    type Output = u64;

    const MESSAGE_A: &'static str = "Sum of each expression";
    const MESSAGE_B: &'static str = "Sum of each (ordered)";

    // Didn't feel like making a better data structure today.
    fn from_string(s: &str) -> String {
        s.to_string()
    }

    /// Get the sum of each solution.
    fn get_solution_a(data: &String) -> Option<u64> {
        let sum = data.lines().map(|l| calculate(l, false).expect(l)).sum();
        Some(sum)
    }

    /// Get the sum with reverse order of operations (+ over *).
    fn get_solution_b(data: &String) -> Option<u64> {
        let sum = data.lines().map(|l| calculate(l, true).expect(l)).sum();
        Some(sum)
    }
}

// fn calculate(expression: &str) -> Result<u64, ParseIntError> {
//     get_val(&mut expression.chars(), '\n')
// }
//
// // Left to right, ignoring order of operations.
// fn get_val(exp: &mut Chars, end: char) -> Result<u64, ParseIntError> {
//     let mut buffer = String::new();
//     let mut val = 0;
//     let mut op = Operation::Set;
//
//     while let Some(c) = exp.next() {
//         match c {
//             '*' => op = Operation::Multiply,
//             '+' => op = Operation::Add,
//             '(' => {
//                 let inner = get_val(exp, ')')?;
//                 do_calc(&mut val, inner, op);
//             }
//             _ if c.is_whitespace() => {
//                 if let Ok(num) = buffer.parse::<u64>() {
//                     do_calc(&mut val, num, op);
//                     buffer.clear();
//                 }
//             }
//             _ if c == end => {
//                 if let Ok(num) = buffer.parse::<u64>() {
//                     do_calc(&mut val, num, op);
//                 }
//                 return Ok(val);
//             }
//             _ if c.is_numeric() => buffer.push(c),
//             _ => panic!("Unexpected character: {}", c),
//         }
//     }
//     if let Ok(num) = buffer.parse::<u64>() {
//         do_calc(&mut val, num, op);
//     }
//     Ok(val)
// }
//
// fn do_calc(result: &mut u64, num: u64, op: Operation) {
//     match op {
//         Operation::Set => *result = num,
//         Operation::Add => *result += num,
//         Operation::Multiply => *result *= num,
//     }
// }
//
// #[derive(Copy, Clone, Debug)]
// pub enum Operation {
//     Multiply,
//     Add,
//     Set,
// }

fn calculate(expression: &str, ord: bool) -> Option<u64> {
    let mut chars = expression.chars().peekable();
    let mut nums = Vec::new();
    let mut ops = Vec::new();

    while let Some(c) = chars.next() {
        if c.is_whitespace() {
            continue;
        } else if c.is_numeric() {
            let mut buffer = String::from(c);
            loop {
                match chars.peek() {
                    Some(peek) if peek.is_numeric() => {
                        buffer.push(*peek);
                        chars.next();
                    }
                    _ => break,
                }
            }
            nums.push(buffer.parse::<u64>().ok()?);
        } else if c == '(' {
            ops.push(c);
        } else if c == ')' {
            while ops.last() != Some(&'(') {
                do_single(&mut nums, &mut ops)?;
            }
            ops.pop();
        } else if c == '+' || c == '*' {
            while !ops.is_empty() && precedence(c, *ops.last()?, ord) {
                do_single(&mut nums, &mut ops)?;
            }
            ops.push(c);
        }
    }
    while !ops.is_empty() {
        do_single(&mut nums, &mut ops)?;
    }
    nums.last().copied()
}

fn precedence(over: char, op: char, ord: bool) -> bool {
    if op == '(' || op == ')' {
        return false;
    }
    if !ord {
        return true;
    }
    op == '+' && over == '*'
}

// for readability
fn do_single(nums: &mut Vec<u64>, ops: &mut Vec<char>) -> Option<()> {
    let res = single(nums.pop()?.clone(), nums.pop()?, ops.pop()?);
    Some(nums.push(res))
}

fn single(a: u64, b: u64, op: char) -> u64 {
    match op {
        '+' => a + b,
        '*' => a * b,
        _ => panic!("Unexpected operation: {}", op),
    }
}

#[test]
fn test_solution_a() {
    let e1 = "2 * 3 + (4 * 5)";
    let e2 = "5 + (8 * 3 + 9 + 3 * 4 * 3)";
    let e3 = "5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))";
    let e4 = "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2";
    let e5 = "2 + (2 * 4)\n5 * 6 + 1\n3 * 3";
    let e6 = "1 + 2 * 3 + 4 * 5 + 6";
    let data = CalculatorSolution::from_string(e5);

    assert_eq!(calculate(e1, false).unwrap(), 26);
    assert_eq!(calculate(e2, false).unwrap(), 437);
    assert_eq!(calculate(e3, false).unwrap(), 12240);
    assert_eq!(calculate(e4, false).unwrap(), 13632);
    assert_eq!(CalculatorSolution::get_solution_a(&data).unwrap(), 50);
    assert_eq!(calculate(e6, false).unwrap(), 71);
}

#[test]
fn test_solution_b() {
    let e1 = "2 * 3 + (4 * 5)";
    assert_eq!(calculate(e1, true).unwrap(), 46);
}
