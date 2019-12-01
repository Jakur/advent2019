use std::io::prelude::*;
use std::fmt;
use std::fs::File;

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
    let mut file = File::open("src/inputs/1.txt").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();
    println!("{}", p1(&input));
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
