use crate::solution_template::Solution;

pub struct SeatSolution;

/// Day 5: determine seating info according to binary partitions.
impl Solution for SeatSolution {
    type Data = Vec<SeatInfo>;
    type Output = i32;

    const MESSAGE_A: &'static str = "Highest seat id";
    const MESSAGE_B: &'static str = "Missing seat id";

    fn from_string(s: &str) -> Vec<SeatInfo> {
        let mut data = Self::map_lines(s, |line| SeatInfo::from_partition(line).expect(line));
        data.sort_by(|s1, s2| s1.id.partial_cmp(&s2.id).unwrap());
        data
    }

    /// The seats have already been sorted. Just return the last one.
    fn get_solution_a(data: &Vec<SeatInfo>) -> Option<i32> {
        data.last().map(|seat| seat.id)
    }

    /// Finds a missing seat ID in a sorted array of `SeatInfo`.
    fn get_solution_b(data: &Vec<SeatInfo>) -> Option<i32> {
        let mut last = data.first()?.id;
        for seat in data {
            let id = seat.id;
            if id - last > 1 {
                return Some(id - 1);
            }
            last = id;
        }
        None
    }
}

#[derive(PartialEq, Debug)]
pub struct SeatInfo {
    pub row: i32,
    pub col: i32,
    pub id: i32,
}

impl SeatInfo {
    pub fn from_partition(bin_space: &str) -> Option<Self> {
        if bin_space.len() != 10 || !Self::is_partition(bin_space) {
            return None;
        }
        let row = Self::bin_search(&bin_space[0..7], 0, 127);
        let col = Self::bin_search(&bin_space[7..], 0, 7);
        let id = row * 8 + col;
        Some(Self { row, col, id })
    }

    fn is_partition(s: &str) -> bool {
        for c in s.chars() {
            if !['F', 'B', 'L', 'R'].contains(&c) {
                return false;
            }
        }
        true
    }

    fn bin_search(bin: &str, min: i32, max: i32) -> i32 {
        if bin.len() == 0 {
            return min; // == max
        }
        let mid = (max as f32 + min as f32) / 2.0;
        return match bin.chars().next().unwrap() {
            'F' | 'L' => Self::bin_search(&bin[1..], min, mid.floor() as i32),
            'B' | 'R' => Self::bin_search(&bin[1..], mid.ceil() as i32, max),
            _ => panic!("Unexpected character: [..{}]", bin),
        };
    }
}

#[test]
fn test_from_partition() {
    let info1 = SeatInfo::from_partition("FBFBBFFRLR").unwrap();
    let info2 = SeatInfo::from_partition("BFFFBBFRRR").unwrap();
    let info3 = SeatInfo::from_partition("FFFBBBFRRR").unwrap();
    let info4 = SeatInfo::from_partition("BBFFBBFRLL").unwrap();
    assert_eq!(
        SeatInfo {
            row: 44,
            col: 5,
            id: 357
        },
        info1
    );
    assert_eq!(
        SeatInfo {
            row: 70,
            col: 7,
            id: 567
        },
        info2
    );
    assert_eq!(
        SeatInfo {
            row: 14,
            col: 7,
            id: 119
        },
        info3
    );
    assert_eq!(
        SeatInfo {
            row: 102,
            col: 4,
            id: 820
        },
        info4
    );
}
