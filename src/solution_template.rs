use std::fmt::Debug;
use std::fs;
use std::path::Path;
use std::process;

use async_trait::async_trait;

/// A setting indicating a constant length for each message output.
const MESSAGE_LENGTH: usize = 40;

#[async_trait]
pub trait Solution {
    /// The type of data consumed by this solution.
    type Data;
    /// The type of result yielded by this solution.
    type Output: Debug;

    /// The message to display when rendering solution a.
    const MESSAGE_A: &'static str;
    /// The message to display when rendering solution b.
    const MESSAGE_B: &'static str;

    /// Instructions for generating your data from a string.
    fn from_string(s: &str) -> Self::Data;

    /// The first solution for this day.
    fn get_solution_a(data: &Self::Data) -> Option<Self::Output>;

    /// The second solution for this day.
    fn get_solution_b(data: &Self::Data) -> Option<Self::Output>;

    /// A utility function for mapping lines in a string to `Vec<T>`.
    fn map_lines<T, F: Fn(&str) -> T>(s: &str, mapper: F) -> Vec<T> {
        s.lines().map(mapper).collect()
    }

    /// A utility function for mapping clusters separated by empty lines
    /// in a string into a `Vec<T>`.
    fn map_clusters<T, F: Fn(&str) -> T>(s: &str, mapper: F) -> Vec<T> {
        s.split_terminator("\n\n").map(mapper).collect()
    }

    /// A utility function for mapping every single character in a file
    /// into a `Vec<Vec<T>>`, where nested arrays correspond to each line.
    fn map_chars<T, F: Fn(char) -> T>(s: &str, mapper: F) -> Vec<Vec<T>> {
        s.lines()
            .map(|line| line.chars().map(&mapper).collect())
            .collect()
    }

    /// Generates a new set of data from a file.
    fn from_file(file: &str) -> Self::Data {
        let path = format!("input/{}", file);
        if !Path::new(&path).exists() {
            println!("You need to place the program input inside of {}", path);
            process::exit(-1);
        }
        Self::from_string(&fs::read_to_string(path).unwrap())
    }

    /// Executes this solution using the path to its data. **This function
    /// is expected to panic** if the length of a message is too long.
    async fn run(path: &str) {
        let data = Self::from_file(path);
        let sep_a = ".".repeat(MESSAGE_LENGTH - Self::MESSAGE_A.len());
        let sep_b = ".".repeat(MESSAGE_LENGTH - Self::MESSAGE_B.len());
        println!(
            "{}: {}{:.^15?}",
            Self::MESSAGE_A,
            sep_a,
            Self::get_solution_a(&data)
        );
        println!(
            "{}: {}{:.^15?}",
            Self::MESSAGE_B,
            sep_b,
            Self::get_solution_b(&data)
        );
    }
}
