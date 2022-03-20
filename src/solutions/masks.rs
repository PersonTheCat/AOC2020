use crate::solution_template::Solution;
use crate::solutions::masks::Assignment::*;
use crate::solutions::masks::Override::*;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

lazy_static! {
    /// The pattern used for parsing mask assignments.
    static ref MASK_PATTERN: Regex = Regex::new(r"mask = ([01X]{36})").unwrap();
    /// The pattern used for parsing memory assignments.
    static ref MEM_PATTERN: Regex = Regex::new(r"mem\[(\d*)\] = (\d*)").unwrap();
}

pub struct MaskSolution;

/// The computer system where the docker is landing isn't compatible with
/// the software it's running. You need to decode the instructions.
///
/// They finally got me. This solution almost required me to start over
/// on part two, but not quite. I chose redundant code just out of spite.
///
/// Just kidding.
impl Solution for MaskSolution {
    type Data = Vec<Assignment>;
    type Output = usize;

    const MESSAGE_A: &'static str = "Total of values in memory (v1)";
    const MESSAGE_B: &'static str = "Total of values in memory (v2)";

    fn from_string(s: &str) -> Vec<Assignment> {
        Self::map_lines(s, |l| Assignment::parse(l).expect(l))
    }

    /// Apply the current mask to each memory assignment. Get the sum.
    fn get_solution_a(data: &Vec<Assignment>) -> Option<usize> {
        Some(MemDecoder::new(data).run().mem_sum())
    }

    /// Expand each X in the every mask. Apply it to the memory addresses.
    fn get_solution_b(data: &Vec<Assignment>) -> Option<usize> {
        Some(MemDecoder::new(data).run_v2().mem_sum())
    }
}

pub struct MemDecoder<'a> {
    assignments: &'a Vec<Assignment>,
    mem: HashMap<usize, usize>,
}

impl<'a> MemDecoder<'a> {
    fn new(assignments: &'a Vec<Assignment>) -> Self {
        Self {
            mem: HashMap::new(),
            assignments,
        }
    }

    fn run(mut self) -> Self {
        let reference = [Unset; 36];
        let mut mask = &reference;
        for a in self.assignments {
            match a {
                Mask(m) => mask = m,
                Mem(i, v) => {
                    let decoded = Override::decode_all(mask, *v);
                    self.mem.insert(*i, decoded);
                }
            }
        }
        self
    }

    fn run_v2(mut self) -> Self {
        let mut masks = Vec::new();
        for a in self.assignments {
            match a {
                Mask(m) => masks = Override::expand(m),
                Mem(i, v) => {
                    for m in &masks {
                        let decoded = Override::decode_all(m, *i);
                        self.mem.insert(decoded, *v);
                    }
                }
            }
        }
        self
    }

    fn mem_sum(&self) -> usize {
        self.mem.values().sum()
    }
}

#[derive(Debug)]
pub enum Assignment {
    Mask([Override; 36]),
    Mem(usize, usize),
}

impl Assignment {
    fn parse(s: &str) -> Option<Assignment> {
        if MASK_PATTERN.is_match(s) {
            let captures = MASK_PATTERN.captures(s)?;
            let raw_mask = captures.get(1)?.as_str();
            let mut mask = [Unset; 36];
            for (i, ch) in raw_mask.chars().enumerate() {
                match ch {
                    '1' => mask[i] = Set,
                    'X' => mask[i] = X,
                    '0' => (),
                    _ => return None,
                }
            }
            return Some(Mask(mask));
        } else if MEM_PATTERN.is_match(s) {
            let captures = MEM_PATTERN.captures(s)?;
            let idx: usize = captures.get(1)?.as_str().parse().unwrap();
            let val: usize = captures.get(2)?.as_str().parse().unwrap();
            return Some(Mem(idx, val));
        }
        None
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Override {
    Set,
    Unset,
    X,
}

impl Override {
    fn decode(&self, idx: usize, mut num: usize) -> usize {
        match self {
            Set => num |= 0x800000000 >> idx,
            Unset => num &= !(0x800000000 >> idx),
            X => (),
        }
        num
    }

    fn decode_all(mask: &[Override], mut num: usize) -> usize {
        for (idx, v) in mask.iter().enumerate() {
            num = v.decode(idx, num);
        }
        num
    }

    fn expand(mask: &[Override; 36]) -> Vec<[Self; 36]> {
        let mut masks = Vec::new();
        let wildcards: Vec<usize> = mask
            .iter()
            .enumerate()
            .filter(|&(_, m)| *m == X)
            .map(|(i, _)| i)
            .collect();

        for i in 0..2usize.pow(wildcards.len() as u32) {
            let mut m = Self::fill_xs(mask);
            for (j, x) in wildcards.iter().enumerate() {
                if (i & (1 << j)) != 0 {
                    m[*x] = Set;
                } else {
                    m[*x] = Unset;
                }
            }
            masks.push(m)
        }
        masks
    }

    fn fill_xs(mask: &[Self; 36]) -> [Self; 36] {
        let mut m = [Set; 36];
        for (i, o) in mask.iter().enumerate() {
            if let Unset = o {
                m[i] = X;
            }
        }
        m
    }
}

#[test]
fn test_solution_a() {
    let example = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
        mem[8] = 11
        mem[7] = 101
        mem[8] = 0";
    let data = example
        .lines()
        .map(|l| Assignment::parse(&l.trim()).expect(l))
        .collect();
    assert_eq!(MaskSolution::get_solution_a(&data).unwrap(), 165)
}

#[test]
fn test_solution_b() {
    let example = "mask = 000000000000000000000000000000X1001X
        mem[42] = 100
        mask = 00000000000000000000000000000000X0XX
        mem[26] = 1";
    let data = example
        .lines()
        .map(|l| Assignment::parse(&l.trim()).expect(l))
        .collect();
    assert_eq!(MaskSolution::get_solution_b(&data).unwrap(), 208)
}

#[test]
fn test_combinations() {
    assert_contains_all(
        get_all_combinations(&[1, 2, 3, 4]),
        vec![
            vec![],
            vec![1],
            vec![2],
            vec![3],
            vec![4],
            vec![1, 2],
            vec![2, 3],
            vec![3, 4],
            vec![1, 3],
            vec![1, 4],
            vec![2, 4],
            vec![1, 2, 3],
            vec![2, 3, 4],
            vec![1, 3, 4],
            vec![1, 2, 4],
            vec![1, 2, 3, 4],
        ],
    )
}

// A variant of one of the trickier methods above
// to make sure it's working how I expect.
#[cfg(test)]
fn get_all_combinations<T: Copy + PartialEq>(of: &[T]) -> Vec<Vec<T>> {
    let mut cache = Vec::new();
    for i in 0..2usize.pow(of.len() as u32) {
        let mut inner = Vec::new();
        for (j, t) in of.iter().enumerate() {
            if (i & (1 << j)) != 0 {
                inner.push(*t)
            }
        }
        cache.push(inner);
    }
    cache
}

#[cfg(test)]
fn assert_contains_all<T: PartialEq>(m: Vec<Vec<T>>, o: Vec<Vec<T>>) {
    for slice in o {
        assert!(m.contains(&slice));
    }
}
