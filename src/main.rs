use std::cmp::{max, min};
use std::fmt;

use std::fs::File;
use std::io::prelude::*;

struct Answer {
    p1: String,
    p2: String,
}

impl Answer {
    pub fn new<R, T>(p1: R, p2: T) -> Answer
    where
        R: fmt::Display,
        T: fmt::Display,
    {
        Answer {
            p1: p1.to_string(),
            p2: p2.to_string(),
        }
    }
}

impl fmt::Display for Answer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Part 1: {}\nPart 2: {}", self.p1, self.p2)
    }
}

fn main() {
    let problem = 4;
    let mut file = File::open(format!("src/inputs/{}.txt", problem)).unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();
    let output = match problem {
        1 => p1(&input),
        2 => p2(&input),
        3 => p3(&input),
        4 => p4(&input),
        _ => unimplemented!(),
    };
    println!("{}", output);
}

fn p1(input: &str) -> Answer {
    let vec: Vec<_> = input
        .lines()
        .map(|s| (s.parse::<i32>().unwrap() / 3) - 2)
        .collect();
    let simple_sum: i32 = vec.iter().sum();
    let total: i32 = vec
        .iter()
        .map(|x| {
            let mut partial = *x;
            let mut next = (x / 3) - 2;
            while next > 0 {
                partial += next;
                next = (next / 3) - 2;
            }
            partial
        })
        .sum();
    Answer::new(simple_sum, total)
}

fn run_intcode(mut vec: Vec<i32>) -> i32 {
    let mut ptr = 0;
    loop {
        let v1 = vec[ptr + 1] as usize;
        let v2 = vec[ptr + 2] as usize;
        let v3 = vec[ptr + 3] as usize;
        match vec[ptr] {
            1 => vec[v3] = vec[v1] + vec[v2],
            2 => vec[v3] = vec[v1] * vec[v2],
            99 => break,
            _ => unimplemented!(),
        }
        ptr += 4;
    }
    vec[0]
}

fn p2(input: &str) -> Answer {
    let mut vec: Vec<i32> = input
        .lines()
        .next()
        .unwrap()
        .split(",")
        .map(|v| v.parse().unwrap())
        .collect();
    // Alter initial input as requested
    vec[1] = 12;
    vec[2] = 2;
    let ans1 = run_intcode(vec.clone());
    for i in 0..100 {
        for j in 0..100 {
            let mut attempt = vec.clone();
            attempt[1] = i;
            attempt[2] = j;
            let ans2 = run_intcode(attempt);
            // Check for requested magic number
            if ans2 == 19690720 {
                return Answer::new(ans1, 100 * i + j);
            }
        }
    }
    unreachable!() // With proper input
}

#[derive(Clone, Copy, PartialEq)]
enum Orientation {
    Vertical,
    Horizontal,
}

#[derive(Clone, Copy, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }
}

struct Segment {
    start: Point,
    end: Point,
    orient: Orientation,
    steps_from_start: i32,
}

fn p3(input: &str) -> Answer {
    let mut paths = Vec::new();
    for raw_path in input.lines() {
        let mut path = Vec::new();
        let mut x = 0;
        let mut y = 0;
        let mut total_steps = 0;
        for mov in raw_path.split(",") {
            let start = Point::new(x, y);
            let mut chars = mov.chars();
            let dir = chars.next().unwrap();
            let val: i32 = chars.as_str().parse().unwrap(); // Remaining chars
            match dir {
                'R' => x += val,
                'L' => x -= val,
                'U' => y += val,
                'D' => y -= val,
                _ => unimplemented!(),
            }
            let orient = match dir {
                'R' | 'L' => Orientation::Horizontal,
                'U' | 'D' => Orientation::Vertical,
                _ => unimplemented!(),
            };
            let end = Point::new(x, y);
            path.push(Segment {
                start,
                end,
                orient,
                steps_from_start: total_steps,
            });
            total_steps += val;
        }
        paths.push(path);
    }
    let mut intersections = Vec::new();
    let mut steps = Vec::new();
    let path1 = paths.pop().unwrap();
    let path2 = paths.pop().unwrap();
    for s1 in path1.iter() {
        for s2 in path2.iter() {
            if s1.orient != s2.orient {
                let (h, v) = {
                    if let Orientation::Horizontal = s1.orient {
                        (s1, s2)
                    } else {
                        (s2, s1)
                    }
                };
                let hmax = max(h.start.x, h.end.x);
                let hmin = min(h.start.x, h.end.x);
                let vmax = max(v.start.y, v.end.y);
                let vmin = min(v.start.y, v.end.y);
                if v.start.x > hmin && v.start.x < hmax {
                    if h.start.y > vmin && h.start.y < vmax {
                        let h_steps = (v.start.x - h.start.x).abs();
                        let v_steps = (h.start.y - v.start.y).abs();
                        let step_count =
                            h_steps + v_steps + s1.steps_from_start + s2.steps_from_start;
                        let intersect = Point::new(v.start.x, h.start.y);
                        intersections.push(intersect);
                        steps.push(step_count);
                    }
                }
            }
        }
    }
    let min_dist = intersections
        .into_iter()
        .map(|pt| pt.x.abs() + pt.y.abs())
        .min()
        .unwrap();
    let min_steps = steps.into_iter().min().unwrap();
    Answer::new(min_dist, min_steps)
}

fn digits(
    digit: i32,
    has_rep: bool,
    last_rep: bool,
    depth: u8,
    number: i32,
    start: i32,
    end: i32,
    blacklist: i32,
    part2: bool,
) -> i32 {
    if depth == 6 {
        if (has_rep || (last_rep && blacklist != digit)) && number >= start && number <= end {
            return 1;
        } else {
            return 0;
        }
    }
    let mut total = 0;
    // Only consider digits of equal of greater value
    for i in digit..10 {
        let mut update_rep = false;
        // Blacklist means we've seen 3 or more in a row
        let update_blacklist = {
            if last_rep && (digit == i) {
                digit
            } else {
                -1
            }
        };
        if part2 {
            if last_rep && (digit != i) && (digit != blacklist) {
                update_rep = true;
            }
        } else if digit == i {
            update_rep = true;
        }
        total += digits(
            i,
            has_rep || update_rep,
            digit == i,
            depth + 1,
            10 * number + i,
            start,
            end,
            update_blacklist,
            part2,
        );
    }
    total
}

fn p4(input: &str) -> Answer {
    let mut split = input.split("-");
    let start: i32 = split.next().unwrap().parse().unwrap();
    let end: i32 = split.next().unwrap().parse().unwrap();
    let mut sums = Vec::new();
    for part2 in [false, true].into_iter() {
        let mut sum = 0;
        for initial in 1..10 {
            if initial >= (start / 100000) && initial <= (end / 100000) {
                sum += digits(initial, false, false, 1, initial, start, end, -1, *part2);
            }
        }
        sums.push(sum);
    }

    Answer::new(sums[0], sums[1])
}