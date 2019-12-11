use std::cmp::{max, min};
use std::fmt;
use std::fs::File;
use std::io::prelude::*;

use std::collections::{HashMap, HashSet};

mod intcode;
use intcode::{parse_intcode, IntMachine};

pub fn problem_multiplex(problem: i32) -> fn(&str) -> Answer {
    match problem {
        1 => p1,
        2 => p2,
        3 => p3,
        4 => p4,
        5 => p5,
        6 => p6,
        7 => p7,
        8 => p8,
        9 => p9,
        10 => p10,
        11 => p11,
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

pub fn p2(input: &str) -> Answer {
    let mut vec: Vec<_> = parse_intcode(input);
    // Alter initial input as requested
    vec[1] = 12;
    vec[2] = 2;
    let mut machine = IntMachine::new(vec.clone());
    machine.run(vec![], false);
    let ans1 = machine.mem[0];
    let mut first = 0;
    let mut second = 0;
    let check = |a, b| {
        let mut attempt = IntMachine::new(vec.clone());
        attempt.mem[1] = a;
        attempt.mem[2] = b;
        attempt.run(vec![], false);
        attempt.mem[0]
    };
    while first < 100 {
        // Check for requested magic number
        let ans2 = check(first, second);
        if ans2 / 10000 == 1969 {
            break;
        }
        first += 1;
    }
    while second < 100 {
        // Check for requested magic number
        let ans2 = check(first, second);
        if ans2 == 19690720 {
            return Answer::new(ans1, 100 * first + second);
        }
        second += 1;
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
    let mut part1 = IntMachine::new(vec.clone());
    let mut output1 = part1.run(vec![1], false);
    let mut part2 = IntMachine::new(vec);
    let mut output2 = part2.run(vec![5], false);
    Answer::new(output1.pop().unwrap(), output2.pop().unwrap())
}

struct Node<'a> {
    parent: &'a str, // parent
    children: Vec<&'a str>,
}

impl<'a> Node<'a> {
    fn new_empty() -> Node<'a> {
        Node {
            parent: "",
            children: Vec::new(),
        }
    }
}

pub fn p6(input: &str) -> Answer {
    let mut nodes = HashMap::new();
    let mut san_parent = "";
    let mut you_parent = "";
    for line in input.lines() {
        let mut split = line.split(")");
        let parent = split.next().unwrap();
        let child = split.next().unwrap();
        let p = nodes.entry(parent).or_insert(Node::new_empty());
        p.children.push(child);
        let mut c = nodes.entry(child).or_insert(Node::new_empty());
        c.parent = parent;
        if child == "SAN" {
            san_parent = parent;
        }
        if child == "YOU" {
            you_parent = parent;
        }
    }
    let mut total_orbits = 0;
    let mut depth: i32 = 1;
    let mut queue = vec!["COM"];
    let mut santa_depth = -1;
    let mut you_depth = -1;
    while queue.len() != 0 {
        let level: Vec<_> = queue.drain(..).collect();
        for next_str in level {
            if next_str == san_parent {
                santa_depth = depth;
            }
            if next_str == you_parent {
                you_depth = depth;
            }
            let next = &nodes[next_str];
            for child in next.children.iter() {
                queue.push(*child);
                total_orbits += depth;
            }
        }
        depth += 1;
    }
    if san_parent == you_parent {
        return Answer::new(total_orbits, 0);
    }
    let mut santa_ancestors = HashSet::new();
    let mut node = nodes.get(san_parent).unwrap();
    depth = santa_depth;
    santa_ancestors.insert(san_parent);
    while node.parent != "" {
        depth -= 1;
        santa_ancestors.insert(node.parent);
        node = nodes.get(node.parent).unwrap();
    }
    assert_ne!(santa_ancestors.get("COM"), None);
    node = nodes.get(you_parent).unwrap();
    depth = you_depth;
    let lca_depth = loop {
        depth -= 1;
        if santa_ancestors.contains(node.parent) {
            break depth;
        }
        node = nodes.get(node.parent).unwrap();
    };
    let distance = santa_depth + you_depth - 2 * lca_depth;
    Answer::new(total_orbits, distance)
}

pub fn p7(input: &str) -> Answer {
    use permutohedron::Heap;
    let instructions = parse_intcode(input);
    let mut options = vec![0, 1, 2, 3, 4];
    let perms = Heap::new(&mut options);
    let mut max1 = std::i64::MIN;
    for perm in perms {
        let mut output_signal = 0;
        for phase in perm {
            let mem = instructions.clone();
            let mut machine = IntMachine::new(mem);
            let out = machine.run(vec![phase, output_signal], false);
            output_signal = out[0];
        }
        let thrust_signal = output_signal;
        if thrust_signal > max1 {
            max1 = thrust_signal;
        }
    }
    let mut options2 = vec![5, 6, 7, 8, 9];
    let perms = Heap::new(&mut options2);
    let mut max2 = std::i64::MIN;
    for perm in perms {
        let mut halted = vec![false; 5];
        let mut amp_ptr = 0;
        let mut inputs: Vec<_> = perm.into_iter().map(|p| vec![p]).collect();
        let mut machines = vec![IntMachine::new(instructions.clone()); 5];
        inputs[0].push(0);
        while !halted.iter().all(|x| *x) {
            let active = &mut machines[amp_ptr];
            let mut out = active.run(inputs[amp_ptr].drain(..).collect(), false);
            if active.mem[active.ptr] == 99 {
                halted[amp_ptr] = true;
            }
            amp_ptr = (amp_ptr + 1) % 5;
            inputs[amp_ptr].append(&mut out);
        }
        let out_signal = inputs[0].pop().unwrap();
        if out_signal > max2 {
            max2 = out_signal;
        }
    }
    Answer::new(max1, max2)
}

pub fn p8(input: &str) -> Answer {
    let width = 25;
    let height = 6;
    let layer_len = width * height;
    let chars: Vec<_> = input.lines().next().unwrap().chars().collect();
    let (min_index, _) = chars
        .chunks(layer_len)
        .map(|x| {
            x.iter().fold(0, |acc, c| match c {
                '0' => acc + 1,
                _ => acc,
            })
        })
        .enumerate()
        .min_by_key(|&(_, item)| item)
        .unwrap();
    let mut ones = 0;
    let mut twos = 0;
    for c in chars[min_index * layer_len..(min_index + 1) * layer_len].iter() {
        match c {
            '1' => ones += 1,
            '2' => twos += 1,
            _ => {}
        }
    }
    let chunks: Vec<_> = chars.chunks(layer_len).collect();
    let mut image = Vec::with_capacity(layer_len);
    for i in 0..chunks[0].len() {
        for chunk in chunks.iter() {
            let pixel = chunk[i];
            match pixel {
                '0' => {
                    image.push(' ');
                    break;
                }
                '1' => {
                    image.push('#');
                    break;
                }
                _ => {}
            }
        }
    }
    let mut part2 = "\n".to_string();
    for scan_line in image.chunks(width) {
        part2.extend(scan_line);
        part2.push_str("\n");
    }
    Answer::new(ones * twos, part2)
}

pub fn p9(input: &str) -> Answer {
    let mem = parse_intcode(input);
    let mut machine = IntMachine::new(mem.clone());
    let out1 = machine.run(vec![1], false);
    machine = IntMachine::new(mem);
    let out2 = machine.run(vec![2], false);
    Answer::new(out1[0], out2[0])
}

#[derive(Clone, Copy, PartialEq)]
enum Square {
    Asteroid,
    Empty,
}

struct Board {
    board: Vec<Square>,
    asteroids: Vec<(i64, i64)>,
    width: usize,
}

impl Board {
    fn part1(&self) -> (usize, i64) {
        let mut max_index = 0;
        let mut max_seen = 0;
        for (index, a) in self.asteroids.iter().enumerate() {
            let mut seen = 0;
            let (x1, y1) = a;
            for b in self.asteroids.iter() {
                let (x2, y2) = b;
                if x1 == x2 && y1 == y2 {
                    continue;
                }
                if (x1 - x2).abs() == 1 || (y1 - y2).abs() == 1 {
                    seen += 1;
                    continue;
                }
                let (delta_x, delta_y) = Self::step_size(*x1, *y1, *x2, *y2);
                if self.ray_walk(*x1, *y1, *x2, *y2, delta_x, delta_y) {
                    seen += 1;
                }
            }
            if seen > max_seen {
                max_seen = seen;
                max_index = index;
            }
        }
        (max_index, max_seen)
    }
    fn part2(&mut self, start_x: i64, start_y: i64) -> (i64, i64) {
        let mut search_order: Vec<_> = self
            .asteroids
            .iter()
            .enumerate()
            .map(|(idx, (x, y))| (idx, x - start_x, start_y - y))
            .collect();
        search_order.sort_by(|a, b| {
            let (_, x1, y1) = a;
            let (_, x2, y2) = b;
            let out1 = Self::sort_2(*x1, *y1);
            let out2 = Self::sort_2(*x2, *y2);
            out1.partial_cmp(&out2).unwrap()
        });
        let mut remove_count = 0;
        let mut remove_set = HashSet::new();
        while search_order.len() > 0 {
            for (index, _weird_x, _weird_y) in search_order.iter() {
                let (x1, y1) = self.asteroids[*index];
                let (delta_x, delta_y) = Self::step_size(x1, y1, start_x, start_y);
                if self.ray_walk(x1, y1, start_x, start_y, delta_x, delta_y) {
                    remove_set.insert(*index);
                    remove_count += 1;
                    if remove_count == 200 {
                        return (x1, y1);
                    }
                }
            }
            search_order = search_order
                .into_iter()
                .filter(|(idx, weird_x, weird_y)| {
                    if remove_set.contains(idx) {
                        self.destroy(weird_x + start_x, (weird_y - start_y) * -1);
                        false
                    } else {
                        true
                    }
                })
                .collect();
        }

        (0, 0)
    }
    fn step_size(x1: i64, y1: i64, x2: i64, y2: i64) -> (i64, i64) {
        use num_rational::Ratio;
        let y_sign = (y2 - y1).signum();
        let x_sign = (x2 - x1).signum();
        let (delta_x, delta_y) = {
            if x1 == x2 {
                (0, 1)
            } else if y1 == y2 {
                (1, 0)
            } else {
                let rat = Ratio::new(x2 - x1, y2 - y1);
                (*rat.numer(), *rat.denom())
            }
        };
        (delta_x.abs() * x_sign, delta_y.abs() * y_sign)
    }
    fn ray_walk(
        &self,
        mut x: i64,
        mut y: i64,
        t_x: i64,
        t_y: i64,
        delta_x: i64,
        delta_y: i64,
    ) -> bool {
        while !(x == t_x && y == t_y) {
            x += delta_x;
            y += delta_y;
            match self.lookup(x, y) {
                Some(sq) => {
                    if let Square::Asteroid = sq {
                        return (x == t_x) && (y == t_y);
                    }
                }
                None => return false,
            }
        }
        true
    }
    fn lookup(&self, x: i64, y: i64) -> Option<&Square> {
        self.board.get((x + y * self.width as i64) as usize)
    }
    fn destroy(&mut self, x: i64, y: i64) {
        self.board[(x + y * self.width as i64) as usize] = Square::Empty;
    }
    fn sort_2(x1: i64, y1: i64) -> f64 {
        let sim = Self::similarity(0.0, 1.0, x1 as f64, y1 as f64);
        if x1 >= 0 {
            return sim * -1.0;
        } else {
            return sim + 2.0;
        }
    }
    fn similarity(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
        let top = x1 * x2 + y1 * y2;
        let bot = (x1 * x1 + y1 * y1).sqrt() * (x2 * x2 + y2 * y2).sqrt();
        (top / bot)
    }
}

pub fn p10(input: &str) -> Answer {
    let mut tiles = Vec::new();
    let mut asteroids = Vec::new();
    let mut height = 0;
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let sq = match c {
                '#' => {
                    asteroids.push((x as i64, y as i64));
                    Square::Asteroid
                }
                '.' => Square::Empty,
                _ => unimplemented!(),
            };
            tiles.push(sq);
        }
        height = y + 1;
    }
    let width = tiles.len() / height;
    let mut board = Board {
        board: tiles,
        asteroids,
        width,
    };
    let (index, ans1) = board.part1();
    let (max_x, max_y) = board.asteroids.swap_remove(index);

    board.destroy(max_x, max_y);
    let (p2_x, p2_y) = board.part2(max_x, max_y);
    Answer::new(ans1, 100 * p2_x + p2_y)
}

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