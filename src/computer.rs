use lazy_static::lazy_static;
use regex::Regex;

use crate::computer::Instruction::*;
use crate::computer::InstructionParseError::*;
use crate::computer::ProcessError::*;
use std::collections::HashSet;

lazy_static! {
    /// The pattern used for parsing [`Instruction`]s.
    static ref INSTRUCTION_PATTERN: Regex = Regex::new(r"^\s*(\S{3})\s([+-]\d{1,3})\s*$").unwrap();
}

/// A data structure capable of processing various sequences of instructions.
pub struct Computer<BreakPoint: FnMut(i64, usize) -> bool> {
    /// A global data store mutated by [`Instruction::Acc`]
    acc: i64,
    /// Tracks the current line number.
    lin: usize,
    /// A predicate accepting two parameters: accumulator value and line number,
    /// which determines when to halt execution and yield the current value in `acc`.
    brk: BreakPoint,
    /// A list of suspicious jumps to "watch." Not at all optimal.
    wch: HashSet<usize>,
}

/// Constructs a new computer with no special conditions.
pub fn new_computer() -> Computer<impl FnMut(i64, usize) -> bool> {
    Computer {
        acc: 0,
        lin: 0,
        brk: |_, _| false,
        wch: HashSet::new(),
    }
}

impl<B: FnMut(i64, usize) -> bool> Computer<B> {
    /// Constructs a new computer with a required breakpoint.
    pub fn with_break(b: B) -> Self {
        Self {
            acc: 0,
            lin: 0,
            brk: b,
            wch: HashSet::new(),
        }
    }

    /// Attempts to process the given instructions. Will panic if any errors are
    /// encountered. Yields the value held by `acc`.
    pub fn throwing_process(&mut self, instructions: &Vec<Instruction>) -> i64 {
        match Self::process(self, instructions) {
            Ok(acc) => acc,
            Err(e) => panic!(e.get_message()),
        }
    }

    /// Executes the given instructions until completion or break, yielding the
    /// current value held in `acc`.w
    pub fn process(&mut self, instructions: &Vec<Instruction>) -> Result<i64, ProcessError> {
        while self.lin < instructions.len() {
            if self.check_break()? {
                break;
            }
            match &instructions[self.lin] {
                Acc(num) => self.accumulate(*num),
                Nop(_) => (),
                Jmp(num) => {
                    self.jump(*num)?;
                    continue;
                }
            }
            self.lin += 1;
        }
        Ok(self.acc)
    }

    pub fn reset(&mut self) {
        self.acc = 0;
        self.lin = 0;
        self.wch.clear();
    }

    fn check_break(&mut self) -> Result<bool, ProcessError> {
        if self.wch.get(&self.lin).is_some() {
            return Err(NoExitCondition(self.lin));
        } else if (self.brk)(self.acc, self.lin) {
            return Ok(true);
        }
        Ok(false)
    }

    fn accumulate(&mut self, num: i32) {
        self.acc += num as i64
    }

    fn jump(&mut self, num: i32) -> Result<(), ProcessError> {
        if num < 0 {
            // jumping back, may loop infinitely.
            if self.lin as i32 - num < 0 {
                return Err(OutOfBounds(self.lin, self.lin as i32 - num));
            }
            self.wch.insert(self.lin);
        }
        self.lin = self.lin.wrapping_add(num as usize);
        Ok(())
    }
}

/// Any errors encountered when executing instructions.
pub enum ProcessError {
    /// A recurring jump was detected at this line.
    NoExitCondition(usize),
    /// An instruction jumped to x, where x < 0.
    OutOfBounds(usize, i32),
}

impl ProcessError {
    pub fn get_message(&self) -> String {
        match self {
            NoExitCondition(ln) => format!("No exit condition from #{}", ln),
            OutOfBounds(ln, jmp) => format!("Jumped out of bounds: {} -> {}", ln, jmp),
        }
    }
}

/// All of the known instructions that can be processed by [`Computer<B>`].
#[derive(Debug)]
pub enum Instruction {
    /// Mutates a global accumulator by the given amount.
    Acc(i32),
    /// Jumps to a *relative* line number.
    Jmp(i32),
    /// Perform no operation on this line.
    Nop(i32),
}

impl Instruction {
    /// Attempts to parse the given text as instructions. Will panic if any errors
    /// are encountered.
    pub fn throwing_parse_all(s: &str) -> Vec<Instruction> {
        match Self::parse_all(s) {
            Ok(instructions) => instructions,
            Err(e) => panic!(e.get_message()),
        }
    }

    /// Parse a full list of instructions, separated by line.
    pub fn parse_all(s: &str) -> Result<Vec<Instruction>, InstructionParseError> {
        s.lines()
            .enumerate()
            .map(|(ln, txt)| Self::parse_internal(txt, ln as u32))
            .collect()
    }

    /// Internal variant of [`Self::parse`] which accounts for the current line number.
    fn parse_internal(s: &str, ln: u32) -> Result<Instruction, InstructionParseError> {
        if s.is_empty() {
            return Err(Whitespace);
        } else if !INSTRUCTION_PATTERN.is_match(s) {
            return Err(SyntaxErr(ln));
        }
        let captures = INSTRUCTION_PATTERN.captures(s).unwrap();
        let instruction = captures.get(1).unwrap().as_str();
        let argument = captures.get(2).unwrap().as_str().parse::<i32>().unwrap();
        match instruction {
            "acc" => Ok(Acc(argument)),
            "jmp" => Ok(Jmp(argument)),
            "nop" => Ok(Nop(argument)),
            _ => Err(UnknownOp(ln, instruction.to_owned())),
        }
    }
}

/// Any errors encountered when parsing instructions from text.
pub enum InstructionParseError {
    SyntaxErr(u32),
    UnknownOp(u32, String),
    Whitespace,
}

impl InstructionParseError {
    pub fn get_message(&self) -> String {
        match self {
            SyntaxErr(ln) => format!("Syntax error on line #{}", ln),
            UnknownOp(ln, op) => format!("Unknown operation: {}@{}", op, ln),
            Whitespace => "Unable to process whitespace.".to_owned(),
        }
    }
}
