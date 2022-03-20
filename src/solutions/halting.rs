use crate::computer::Instruction::*;
use crate::computer::{new_computer, Computer, Instruction};
use crate::solution_template::Solution;
use std::sync::Mutex;

pub struct HaltingSolution;

/// You're inspecting some corrupted boot code from a game console.
impl Solution for HaltingSolution {
    type Data = Mutex<Vec<Instruction>>;
    type Output = i64;

    const MESSAGE_A: &'static str = "Number before infinite loop";
    const MESSAGE_B: &'static str = "Number after bruteforce swap";

    fn from_string(s: &str) -> Mutex<Vec<Instruction>> {
        Mutex::new(Instruction::throwing_parse_all(s))
    }

    /// Stop before processing any instruction a second time, get the value.
    fn get_solution_a(data: &Mutex<Vec<Instruction>>) -> Option<i64> {
        let mut lines = Vec::new();
        let mut computer = Computer::with_break(move |_, line| {
            let small = line as u16;
            if lines.contains(&small) {
                return true;
            }
            lines.push(small);
            false
        });
        Some(computer.throwing_process(&data.lock().unwrap()))
    }

    /// Find and swap the last Jmp <-> Nop until it works. Bruteforce. :(
    fn get_solution_b(data: &Mutex<Vec<Instruction>>) -> Option<i64> {
        let mut data = data.lock().unwrap();
        let mut computer = new_computer();
        let mut store = None;
        let mut i = data.len() - 2;
        while i > 0 {
            let (last, inst) = flip_last_jmp_nop(&mut data, i, store);
            if let Ok(acc) = computer.process(&data) {
                return Some(acc);
            }
            computer.reset();
            i = last;
            store = inst;
        }
        None
    }
}

/// Swaps the last Jmp / Nop command in reverse order, starting at index `from`.
fn flip_last_jmp_nop(
    data: &mut Vec<Instruction>,
    from: usize,
    original: Option<Instruction>,
) -> (usize, Option<Instruction>) {
    if let Some(instruction) = original {
        data[from] = instruction;
    }
    for i in (0..from).rev() {
        if let &Jmp(a) = &data[i] {
            data[i] = Nop(a);
            return (i, Some(Jmp(a)));
        } else if let &Nop(a) = &data[i] {
            data[i] = Jmp(a);
            return (i, Some(Nop(a)));
        }
    }
    (0, None)
}

#[test]
fn test_solution_a() {
    let instructions = vec![
        Acc(2),  // 2
        Acc(2),  // 4
        Jmp(-2), // -> L1 (should stop)
        Jmp(1),  // unreachable
    ];
    assert_eq!(
        HaltingSolution::get_solution_a(&Mutex::new(instructions)).unwrap(),
        4
    )
}

#[test]
fn test_solution_b() {
    let instructions = vec![
        Acc(2),  // 2
        Acc(2),  // 4
        Jmp(-2), // -> L1 (should get flipped)
        Jmp(2),  // -> L6
        Acc(9),  // -
        Acc(9),  // 13
        Jmp(1),  // exit
    ];
    assert_eq!(
        HaltingSolution::get_solution_b(&Mutex::new(instructions)).unwrap(),
        13
    )
}
