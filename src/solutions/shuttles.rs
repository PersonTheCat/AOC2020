use crate::solution_template::Solution;

pub struct ShuttleSolution;

/// You are given a schedule and need to find
/// the soonest bus to leave for the airport.
impl Solution for ShuttleSolution {
    type Data = ShuttleData;
    type Output = i64;

    const MESSAGE_A: &'static str = "Earliest shuttle (ID)";
    const MESSAGE_B: &'static str = "Earliest shuttle (pos) :c ";

    fn from_string(s: &str) -> ShuttleData {
        ShuttleData::parse(s).expect(s)
    }

    /// Each number represents the bus' schedule.
    fn get_solution_a(data: &ShuttleData) -> Option<i64> {
        const MAX: i32 = 1000;
        let mut check = data.time;
        // It should never take this long. This
        // would just prevent an infinite loop.
        for _ in 0..MAX {
            for &(_, shuttle) in &data.shuttles {
                if check % shuttle == 0 {
                    let eta = check - data.time;
                    return Some(eta * shuttle);
                }
            }
            check += 1;
        }
        None
    }

    /// Each position represents the bus' schedule. In the comments,
    /// you can see my original bruteforce solution which was
    /// incredibly slow, though it did work. I discovered that other
    /// users on Reddit were using this "Chinese Remainder Theorem,"
    /// and copied some code from there to do the same. You can see
    /// my lack of advanced math skills leaking through.
    fn get_solution_b(data: &ShuttleData) -> Option<i64> {
        // // Get an estimated range to save time.
        // let max: i64 = data.shuttles.iter()
        //     .map(|&(_, shuttle)| shuttle)
        //     .product();
        // let min = 10i64.pow(max.to_string().len() as u32 - 2);
        // let min = first_mod_over(min, data.shuttles[0].1);
        //
        // for check in (min..max).step_by(data.shuttles[0].1 as usize) {
        //     if shuttles_depart_in_order(check, &data.shuttles) {
        //         return Some(check);
        //     }
        // }
        // None

        let eta_mapped: Vec<(i64, i64)> = data
            .shuttles
            .iter()
            .map(|&(i, shuttle)| (shuttle, shuttle - i))
            .collect();
        Some(chi_rem(&eta_mapped))

        // let max: i64 = data.shuttles.iter()
        //     .map(|&(_, shuttle)| shuttle)
        //     .product();
        // let min = 10i64.pow(max.to_string().len() as u32 - 2);
        // println!("checking {}..{}", min, max);
        // let mut time = first_mod_over(min, data.shuttles[0].1);
        // let mut step = data.shuttles[0].1;
        // let mut skip = 1;
        // while time < max {
        //     match check_time(time, skip, &data.shuttles) {
        //         Ok(_) => return Some(time),
        //         Err(found) => {
        //             if found > 0 {
        //                 step += lcm(step, time);
        //                 skip += found;
        //                 println!("found {} ({}) at {}", found, skip, time);
        //             }
        //         }
        //     }
        //     time += step;
        // }
        // None
    }
}

fn egcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x, y) = egcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}

fn mod_inv(x: i64, n: i64) -> i64 {
    let (_, x, _) = egcd(x, n);
    (x % n + n) % n
}

fn chi_rem(pairs: &[(i64, i64)]) -> i64 {
    let product: i64 = pairs.iter().map(|&(i, _)| i).product();

    pairs.iter().fold(0, |acc, &(i, j)| {
        let p = product / i;
        acc + j * mod_inv(p, i) * p
    }) % product
}

// fn first_mod_over(num: i64, of: i64) -> i64 {
//     for i in num..(num + of) {
//         if i % of == 0 {
//             return i;
//         }
//     }
//     panic!("None found");
// }
//
// fn shuttles_depart_in_order(time: i64, shuttles: &Vec<(i64, i64)>) -> bool {
//     for &(i, shuttle) in shuttles.iter().skip(1) {
//         if (time + i) % shuttle != 0 {
//             return false;
//         }
//     }
//     true
// }

// fn first_mod_over(num: i64, of: i64) -> i64 {
//     for i in num..(num + of) {
//         if i % of == 0 {
//             return i;
//         }
//     }
//     panic!("None found");
// }
//
// fn check_time(time: i64, skip: usize, shuttles: &[(i64, i64)]) -> Result<(), usize> {
//     let mut found = 0;
//     for &(i, shuttle) in shuttles.iter().skip(skip) {
//         if (time + i) % shuttle == 0 {
//             found += 1;
//         } else {
//             return Err(found)
//         }
//     }
//     Ok(())
// }
//
// fn lcm(a: i64, b: i64) -> i64 {
//     a * b / gcd(a, b)
// }
//
// fn gcd(a: i64, b: i64) -> i64 {
//     let mut max = a;
//     let mut min = b;
//     if min > max {
//         let val = max;
//         max = min;
//         min = val;
//     }
//     loop {
//         let res = max % min;
//         if res == 0 {
//             return min;
//         }
//         max = min;
//         min = res;
//     }
// }

pub struct ShuttleData {
    time: i64,
    shuttles: Vec<(i64, i64)>,
}

impl ShuttleData {
    fn parse(s: &str) -> Option<Self> {
        let mut lines = s.lines();
        Some(Self {
            time: lines.next()?.parse().ok()?,
            shuttles: Self::read_shuttles(lines.next()?),
        })
    }

    fn read_shuttles(s: &str) -> Vec<(i64, i64)> {
        let mut vec = Vec::new();
        for (i, shuttle) in s.split(",").enumerate() {
            if shuttle == "x" {
                continue;
            }
            match shuttle.parse::<i64>() {
                Ok(num) => vec.push((i as i64, num)),
                Err(_) => panic!("Unexpected characters: {}", shuttle),
            }
        }
        vec
    }
}

#[test]
fn test_solution_a() {
    let example = "939\n7,13,x,x,59,x,31,19";
    let data = ShuttleSolution::from_string(example);
    assert_eq!(ShuttleSolution::get_solution_a(&data).unwrap(), 295)
}

// The earliest timestamp that matches the list 17,x,13,19 is 3417.
// 67,7,59,61 first occurs at timestamp 754018.
// 67,x,7,59,61 first occurs at timestamp 779210.
// 67,7,x,59,61 first occurs at timestamp 1261476.
// 1789,37,47,1889 first occurs at timestamp 1202161486.
#[test]
fn test_solution_b() {
    check_solution_b("939\n7,13,x,x,59,x,31,19", 1068781);
    check_solution_b("0\n17,x,13,19", 3417);
    check_solution_b("0\n67,7,59,61", 754018);
    check_solution_b("0\n67,x,7,59,61", 779210);
    check_solution_b("0\n67,7,x,59,61", 1261476);
    check_solution_b("0\n1789,37,47,1889", 1202161486);
}

#[cfg(test)]
fn check_solution_b(example: &str, expected: i64) {
    let data = ShuttleSolution::from_string(example);
    assert_eq!(ShuttleSolution::get_solution_b(&data).unwrap(), expected);
}
