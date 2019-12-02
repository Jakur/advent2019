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
    // let mut file = File::open("src/inputs/1.txt").unwrap();
    // let mut input = String::new();
    // file.read_to_string(&mut input).unwrap();
    // println!("{}", p1(&input));
    let mut file = File::open("src/inputs/2.txt").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();
    // let input = "1,9,10,3,2,3,11,0,99,30,40,50\n".to_string();
    println!("{}", p2(&input));
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
