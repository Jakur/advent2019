pub use std::cmp::{max, min};

pub use std::collections::{HashMap, HashSet};
pub use std::fmt;
pub mod intcode;
pub use intcode::{parse_intcode, IntMachine};

pub mod p1;
pub mod p10;
pub mod p11;
pub mod p12;
pub mod p13;
pub mod p14;
pub mod p15;
pub mod p2;
pub mod p3;
pub mod p4;
pub mod p5;
pub mod p6;
pub mod p7;
pub mod p8;
pub mod p9;

use std::fs::File;
use std::io::prelude::*;

pub fn problem_multiplex(problem: i32) -> fn(&str) -> Answer {
    match problem {
        1 => p1::p1,
        2 => p2::p2,
        3 => p3::p3,
        4 => p4::p4,
        5 => p5::p5,
        6 => p6::p6,
        7 => p7::p7,
        8 => p8::p8,
        9 => p9::p9,
        10 => p10::p10,
        11 => p11::p11,
        12 => p12::p12,
        13 => p13::p13,
        14 => p14::p14,
        15 => p15::p15,
        _ => unimplemented!(),
    }
}

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
