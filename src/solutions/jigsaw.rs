use crate::solution_template::Solution;
use std::sync::Mutex;
use std::iter::Zip;
use std::slice::Iter;

pub struct JigsawSolution {
    jigsaws: Vec<Jigsaw>,
    frame: Vec<Vec<Option<Jigsaw>>>,
}

/// Explain the rules of this day
impl Solution for JigsawSolution {
    type Data = Mutex<Self>;
    type Output = u64;

    const MESSAGE_A: &'static str = "Product of corners";
    const MESSAGE_B: &'static str = "Number of monsters";

    fn from_string(s: &str) -> Mutex<Self> {
        Mutex::new(Self {
            jigsaws: Self::map_clusters(s, |c| Jigsaw::parse(c).expect(c)),
            frame: Vec::new(),
        })
    }

    /// Assemble the image. Multiple the IDs of the corners.
    fn get_solution_a(data: &Mutex<Self>) -> Option<u64> {
        let mut data = data.lock().unwrap();
        let a = data.jigsaws.len();
        // Assuming this image is relatively square.
        let l = ((a as f32).sqrt() * 2.0 + 1.0) as usize;
        let c = l / 2 + 1;
        let mut frame = vec![vec![None; l]; l];
        // Start in the center of the frame and work
        // around it so we don't have to shift.
        frame[c][c] = data.jigsaws.pop();
        let mut n = a;

        // The current line of thought is that, when we're searching in order,
        // if we ever *don't* find a validate candidate to place, then we've
        // probably hit the edge of the image.

        // Look into creating a "frame iterator" with functionalities for
        // skipping to the next row. Could be useful. Then, you would just
        // need to determine direction using current / last coordinates.

        for i in 0..a {
            let n2 = data.jigsaws.len();
            if n != n2 {
                println!("num left: {}", n2);
            }
            n = n2;
            if data.jigsaws.is_empty() {
                println!("complete @ {}", i);
                break;
            }
            for y in (0..(l - 1)).step_by(2) {
                let mut last = (0, y);
                // First check down.
                if frame[last.1][last.0].is_some() && frame[y][0].is_none() {
                    let r = frame[last.1][last.0].as_ref().unwrap();
                    if let Some(idx) = r.find_down(&mut data.jigsaws) {
                        frame[y][0] = Some(data.jigsaws.remove(idx));
                    }
                }
                last = (0, y);
                // And then right.
                for x in 1..l {
                    if frame[last.1][last.0].is_some() && frame[y][x].is_none() {
                        let r = frame[last.1][last.0].as_ref().unwrap();
                        if let Some(idx) = r.find_right(&mut data.jigsaws) {
                            frame[y][x] = Some(data.jigsaws.remove(idx));
                        } else {
                            break;
                        }
                    }
                    last = (x, y);
                }
                // And then down again.
                let y = y + 1;
                if frame[last.1][last.0].is_some() && frame[y][last.0].is_none() {
                    let r = frame[last.1][last.0].as_ref().unwrap();
                    if let Some(idx) = r.find_down(&mut data.jigsaws) {
                        frame[y][0] = Some(data.jigsaws.remove(idx));
                    }
                }
                last = (last.0, y);
                // And then left.
                for x in (1..last.0).rev() {
                    if frame[last.1][last.0].is_some() && frame[y][x].is_none() {
                        let r = frame[last.1][last.0].as_ref().unwrap();
                        if let Some(idx) = r.find_left(&mut data.jigsaws) {
                            frame[y][x] = Some(data.jigsaws.remove(idx));
                        } else {
                            break;
                        }
                    }
                    last = (x, y);
                }
            }
        }


        // let mut pos = (c, c);
        // let mut last = frame[c][c].as_ref().unwrap();
        // for _ in 0..(data.jigsaws.len().pow(2)) {
        //     let mut piece = data.jigsaws.remove(0);
        //     let found = piece.for_each_translation(|p| {
        //         if pos.0 + 1 < l && frame[pos.1][pos.0 + 1].is_none() {
        //             if last.check_right(p) {
        //                 println!("b {:?}", pos);
        //                 pos.0 += 1;
        //                 println!("a {:?}", pos);
        //                 return true;
        //             }
        //         } else if pos.0 > 0 && frame[pos.1][pos.0 - 1].is_none() {
        //             if last.check_left(p) {
        //                 pos.0 -= 1;
        //                 return true;
        //             }
        //         } else if pos.1 + 1 > l && frame[pos.1 + 1][pos.0].is_none() {
        //             if last.check_up(p) {
        //                 pos.1 += 1;
        //                 return true;
        //             }
        //         } else if pos.1 > 0 && frame[pos.1 - 1][pos.0].is_none() {
        //             if last.check_down(p) {
        //                 pos.1 -= 1;
        //                 return true;
        //             }
        //         }
        //         false
        //     });
        //     if found {
        //         frame[pos.1][pos.0] = Some(piece);
        //         last = frame[pos.1][pos.0].as_ref().unwrap();
        //     } else {
        //         data.jigsaws.push(piece);
        //     }
        // }
        display_frame(&frame);
        None
    }

    /// Explain solution B
    fn get_solution_b(_: &Mutex<Self>) -> Option<u64> {
        None
    }
}

fn display(jigsaw: &Jigsaw) {
    for line in &jigsaw.jig {
        print!("[ ");
        for b in line {
            if *b {
                print!("# ")
            } else {
                print!(". ")
            }
        }
        println!("]")
    }
}

fn display_frame(frame: &Vec<Vec<Option<Jigsaw>>>) {
    let c = frame.len() / 2 + 1;
    let a = frame[c][c].as_ref().unwrap().jig.len();
    let w = a * frame.len();
    for x in 0..w {
        print!("[ ");
        for y in 0..w {
            if let Some(jig) = &frame[x / a][y / a] {
                if jig.jig[x % a][y % a] {
                    print!("# ");
                } else {
                    print!("  ");
                }
            } else {
                print!("  ");
            }
        }
        println!("]")
    }
}

#[derive(Debug, Clone)]
pub struct Jigsaw {
    id: u64,
    jig: Vec<Vec<bool>>,
}

impl Jigsaw {
    fn parse(s: &str) -> Option<Self> {
        let mut lines = s.lines();
        let title = &lines.next()?[5..9];
        let id = title.parse().ok()?;
        let jig = lines.map(|s| s.chars().map(is_pixel).collect()).collect();
        Some(Self { id, jig })
    }

    // Check every possible translation until one returns true.
    fn any_trans<F: FnMut(&Self) -> bool>(&mut self, mut f: F) -> bool {
        for _ in 0..4 {
            if f(self) {
                return true;
            }
            self.rotate();
        }
        self.flip_x();
        if f(self) {
            return true;
        }
        self.flip_x();
        self.flip_y();
        if f(self) {
            return true;
        }
        self.flip_y();
        false
    }

    fn rotate(&mut self) {
        let n = self.jig.len();
        for x in 0..(n / 2) {
            for y in x..(n - x - 1) {
                let old = self.jig[x][y];
                self.jig[x][y] = self.jig[y][n - 1 - x];
                self.jig[y][n - 1 - x] = self.jig[n - 1 - x][n - 1 - y];
                self.jig[n - 1 - x][n - 1 - y] = self.jig[n - 1 - y][x];
                self.jig[n - 1 - y][x] = old;
            }
        }
    }

    fn flip_x(&mut self) {
        let n = self.jig.len();
        for x in 0..(n / 2) {
            for y in 0..n {
                let old = self.jig[x][y];
                self.jig[x][y] = self.jig[n - 1 - x][y];
                self.jig[n - 1 - x][y] = old;
            }
        }
    }

    fn flip_y(&mut self) {
        let n = self.jig.len();
        for y in 0..(n / 2) {
            for x in 0..n {
                let old = self.jig[x][y];
                self.jig[x][y] = self.jig[x][n - 1 - y];
                self.jig[x][n - 1 - y] = old;
            }
        }
    }

    fn find_down(&self, from: &mut Vec<Self>) -> Option<usize> {
        for (i, jig) in from.iter_mut().enumerate() {
            if jig.any_trans(|f| self.check_down(f)) {
                return Some(i);
            }
        }
        None
    }

    fn find_left(&self, from: &mut Vec<Self>) -> Option<usize> {
        for (i, jig) in from.iter_mut().enumerate() {
            if jig.any_trans(|f| self.check_left(f)) {
                return Some(i);
            }
        }
        None
    }

    fn find_right(&self, from: &mut Vec<Self>) -> Option<usize> {
        for (i, jig) in from.iter_mut().enumerate() {
            if jig.any_trans(|f| self.check_right(f)) {
                return Some(i);
            }
        }
        None
    }

    fn check_left(&self, other: &Self) -> bool {
        return self.zip(other)
            .all(|(s_vec, o_vec)| s_vec[0] == o_vec[o_vec.len() - 1]);
    }

    fn check_right(&self, other: &Self) -> bool {
        return self.zip(other)
            .all(|(s_vec, o_vec)| s_vec[s_vec.len() - 1] == o_vec[0]);
    }

    fn check_up(&self, other: &Self) -> bool {
        self.jig[0] == other.jig[other.jig.len() - 1]
    }

    fn check_down(&self, other: &Self) -> bool {
        self.jig[self.jig.len() - 1] == other.jig[0]
    }

    fn zip<'a>(&'a self, other: &'a Self) -> Zip<Iter<'a, Vec<bool>>, Iter<'a, Vec<bool>>> {
        self.jig.iter().zip(other.jig.iter())
    }
}

fn is_pixel(c: char) -> bool {
    match c {
        '.' => false,
        '#' => true,
        _ => panic!("Unexpected character: '{}'", c),
    }
}

#[test]
fn test_solution_a() {
    // assert_eq!(None, JigsawSolution::get_solution_a(&""))
}

#[test]
fn test_solution_b() {
    // assert_eq!(None, JigsawSolution::get_solution_b(&""))
}
