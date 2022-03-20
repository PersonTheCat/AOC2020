use crate::solution_template::Solution;

pub struct ConwaySolution;

pub type Cube = Vec<Vec<Vec<bool>>>;
pub type Time = Vec<Cube>;

/// Elves have a new power source contains an infinite, multidimensional
/// array of cubes which may be active or inactive. Those cubes change
/// state based on a rule of how many of their neighbors are also active
/// or inactive. You're creating a simulation to help the elves study
/// this new technology.
impl Solution for ConwaySolution {
    type Data = Cube;
    type Output = u32;

    const MESSAGE_A: &'static str = "Num cubes (3D)";
    const MESSAGE_B: &'static str = "Num cubes (4D)";

    fn from_string(s: &str) -> Cube {
        vec![Self::map_chars(s, is_active)]
    }

    /// Find the number of cubes inside of the box after 6 cycles.
    fn get_solution_a(data: &Cube) -> Option<u32> {
        // Expand this to be more than large enough
        // so we don't need boundary checks.
        let mut cube = expand(data, data.len() + 21);
        for _ in 0..6 {
            cube = cycle(&cube);
        }
        Some(count_total(&cube))
    }

    /// Find the number of cubes after 6 cycles in 4 dimensions.
    fn get_solution_b(data: &Cube) -> Option<u32> {
        let new_len = data.len() + 21;
        let mut time = get_time(new_len);
        time[new_len / 2] = expand(data, new_len);
        for _ in 0..6 {
            time = cycle_4d(&time);
        }
        Some(count_total_4d(&time))
    }
}

/// Determines whether a given character represents an active space.
fn is_active(c: char) -> bool {
    match c {
        '.' => false,
        '#' => true,
        _ => panic!("Unexpected character: '{}'", c),
    }
}

fn cycle(source: &Cube) -> Cube {
    let mut new = get_cube(source.len());
    for z in 1..(new.len() - 1) {
        for y in 1..(new.len() - 1) {
            for x in 1..(new.len() - 1) {
                let mut count = count_surrounding(source, x, y, z);
                let active = source[z][y][x];
                if active {
                    count -= 1;
                }
                if count == 3 || active && count == 2 {
                    new[z][y][x] = true;
                }
            }
        }
    }
    new
}

fn cycle_4d(source: &Time) -> Time {
    let mut new = get_time(source.len());
    for t in 1..(new.len() - 1) {
        for z in 1..(new.len() - 1) {
            for y in 1..(new.len() - 1) {
                for x in 1..(new.len() - 1) {
                    let mut count = count_4d(source, x, y, z, t);
                    let active = source[t][z][y][x];
                    if active {
                        count -= 1;
                    }
                    if count == 3 || active && count == 2 {
                        new[t][z][y][x] = true;
                    }
                }
            }
        }
    }
    new
}

fn count_total(cube: &Cube) -> u32 {
    let mut count = 0;
    for z in cube {
        for y in z {
            for &x in y {
                if x {
                    count += 1;
                }
            }
        }
    }
    count
}

fn count_total_4d(time: &Time) -> u32 {
    let mut count = 0;
    for cube in time {
        count += count_total(cube);
    }
    count
}

fn get_cube(len: usize) -> Cube {
    vec![vec![vec![false; len]; len]; len]
}

fn get_time(len: usize) -> Time {
    vec![vec![vec![vec![false; len]; len]; len]; len]
}

fn expand(cube: &Cube, len: usize) -> Cube {
    let mut new = get_cube(len);
    let offset_z = (len / 2) - (cube.len() / 2);
    let offset_y = (len / 2) - (cube[0].len() / 2);
    let offset_x = (len / 2) - (cube[0][0].len() / 2);
    for (z, mat) in cube.iter().enumerate() {
        for (y, vec) in mat.iter().enumerate() {
            for (x, &b) in vec.iter().enumerate() {
                new[z + offset_z][y + offset_y][x + offset_x] = b;
            }
        }
    }
    new
}

fn count_4d(source: &Time, x: usize, y: usize, z: usize, t: usize) -> u32 {
    let mut count = 0;
    for cube in &source[(t - 1)..(t + 2)] {
        count += count_surrounding(cube, x, y, z);
    }
    count
}

fn count_surrounding(source: &Cube, x: usize, y: usize, z: usize) -> u32 {
    let mut count = 0;
    for mat in &source[(z - 1)..(z + 2)] {
        for vec in &mat[(y - 1)..(y + 2)] {
            for &b in &vec[(x - 1)..(x + 2)] {
                if b {
                    count += 1;
                }
            }
        }
    }
    count
}

#[test]
fn test_solution_a() {
    let example = ".#.\n..#\n###";
    let data = ConwaySolution::from_string(example);
    assert_eq!(ConwaySolution::get_solution_a(&data).unwrap(), 112)
}

#[test]
fn test_solution_b() {
    let example = ".#.\n..#\n###";
    let data = ConwaySolution::from_string(example);
    assert_eq!(ConwaySolution::get_solution_b(&data).unwrap(), 848)
}
