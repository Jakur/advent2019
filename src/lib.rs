use std::cmp::{max, min};
use std::fmt;
use std::fs::File;
use std::io::prelude::*;

pub struct Answer {
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

pub fn get_input(problem: i32) -> String {
    let mut file = File::open(format!("src/inputs/{}.txt", problem)).unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();
    input
}

pub fn p1(input: &str) -> Answer {
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

fn mode_fn(mode: i32, pos: usize, vec: &Vec<i32>) -> Option<i32> {
    // assert_eq!(mode, 0);
    let value = vec.get(pos);
    let output = match mode {
        0 => value.and_then(|v| vec.get(*v as usize)),
        1 => value,
        _ => {
            println!("{}", mode);
            println!("{}", pos);
            println!("{:?}", vec);
            unimplemented!()
        }
    };
    output.map(|v| *v)
}

fn run_intcode(vec: &mut Vec<i32>, input: Vec<i32>) -> Vec<i32> {
    let mut ptr = 0;
    let mut output = Vec::new();
    let mut input = input.into_iter();

    loop {
        let x = vec[ptr];
        let opcode = vec[ptr] % 100;
        let v1 = mode_fn((x / 100) % 10, ptr + 1, &vec);
        let v2 = mode_fn((x / 1000) % 10, ptr + 2, &vec);
        let loc3 = vec.get(ptr + 3).map(|v| *v as usize);
        // let v3 = mode_fn((x / 10000) % 10, ptr + 3, &vec);
        match opcode {
            1 => {
                vec[loc3.unwrap()] = v1.unwrap() + v2.unwrap();
                ptr += 4;
            }
            2 => {
                vec[loc3.unwrap()] = v1.unwrap() * v2.unwrap();
                ptr += 4;
            }
            3 => {
                let v = vec[ptr + 1] as usize;
                vec[v] = input.next().unwrap();
                ptr += 2;
            }
            4 => {
                output.push(v1.unwrap());
                ptr += 2;
            }
            5 => {
                if v1.unwrap() != 0 {
                    ptr = v2.unwrap() as usize;
                } else {
                    ptr += 3;
                }
            }
            6 => {
                if v1.unwrap() == 0 {
                    ptr = v2.unwrap() as usize;
                } else {
                    ptr += 3;
                }
            }
            7 => {
                let out = (v1.unwrap() < v2.unwrap()).into();
                vec[loc3.unwrap()] = out;
                ptr += 4;
            }
            8 => {
                let out = (v1.unwrap() == v2.unwrap()).into();
                vec[loc3.unwrap()] = out;
                ptr += 4;

            }
            99 => break,
            _ => unimplemented!(),
        }
    }
    output
}

fn parse_intcode(input: &str) -> Vec<i32> {
    input
        .lines()
        .next()
        .unwrap()
        .split(",")
        .map(|v| v.parse().unwrap())
        .collect()
}

pub fn p2(input: &str) -> Answer {
    let mut vec: Vec<i32> = parse_intcode(input);
    // Alter initial input as requested
    vec[1] = 12;
    vec[2] = 2;
    let mut part1 = vec.clone();
    run_intcode(&mut part1, vec![]);
    let ans1 = part1[0];
    for i in 0..100 {
        for j in 0..100 {
            let mut attempt = vec.clone();
            attempt[1] = i;
            attempt[2] = j;
            run_intcode(&mut attempt, vec![]);
            // Check for requested magic number
            let ans2 = attempt[0];
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

pub fn p3(input: &str) -> Answer {
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

pub fn p4(input: &str) -> Answer {
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

pub fn p5(input: &str) -> Answer {
    let vec = parse_intcode(input);
    let mut part1 = vec.clone();
    let mut output1 = run_intcode(&mut part1, vec![1]);
    let mut part2 = vec.clone();
    let mut output2 = run_intcode(&mut part2, vec![5]);
    Answer::new(output1.pop().unwrap(), output2.pop().unwrap())
}