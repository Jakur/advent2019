use super::Answer;
use crate::intcode::{parse_intcode, IntMachine};

use std::collections::HashMap;

enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn rotate(self, dir: i64) -> Direction {
        assert!(dir == 0 || dir == 1);
        if dir == 0 {
            match self {
                Direction::Up => Direction::Left,
                Direction::Left => Direction::Down,
                Direction::Down => Direction::Right,
                Direction::Right => Direction::Up,
            }
        } else {
            match self {
                Direction::Up => Direction::Right,
                Direction::Right => Direction::Down,
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Up,
            }
        }
    }
    fn move_robot(&self, x: i32, y: i32) -> (i32, i32) {
        match self {
            Direction::Up => (x, y + 1),
            Direction::Right => (x + 1, y),
            Direction::Down => (x, y - 1),
            Direction::Left => (x - 1, y),
        }
    }
}

fn drive_car(code: Vec<i64>, mut tile_color: i64) -> HashMap<(i32, i32), i64> {
    let mut map = HashMap::new();
    let mut machine = IntMachine::new(code);
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut dir = Direction::Up;
    while !machine.halted() {
        let out = machine.run(vec![tile_color], false);
        if out.len() < 2 {
            break;
        }
        map.insert((x, y), out[0]);
        dir = dir.rotate(out[1]);
        let tuple = dir.move_robot(x, y);
        x = tuple.0;
        y = tuple.1;
        tile_color = *map.entry((x, y)).or_default();
    }
    map
}

pub fn p11(input: &str) -> Answer {
    let code = parse_intcode(input);
    let ans1 = drive_car(code.clone(), 0).len();
    let map = drive_car(code, 1);
    let mut x_min = std::i32::MAX;
    let mut x_max = std::i32::MIN;
    let mut y_min = std::i32::MAX;
    let mut y_max = std::i32::MIN;
    for ((x, y), _c) in map.iter() {
        let x = *x;
        let y = *y;
        if x > x_max {
            x_max = x;
        } else if x < x_min {
            x_min = x;
        }
        if y > y_max {
            y_max = y;
        } else if y < y_min {
            y_min = y;
        }
    }
    let mut out = vec![vec![' '; (x_max - x_min + 1) as usize]; (y_max - y_min + 1) as usize];
    for ((x, y), c) in map.into_iter() {
        if c == 1 {
            let x_index = x - x_min;
            let y_index = y - y_min;
            out[y_index as usize][x_index as usize] = '#';
        }
    }
    let mut ans2 = "\n".to_string();
    for scan_line in out.into_iter().rev() {
        ans2.extend(scan_line.into_iter());
        ans2.push_str("\n");
    }
    Answer::new(ans1, ans2)
}