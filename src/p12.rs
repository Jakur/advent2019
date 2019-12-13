use super::*;

use num_integer::Integer;

#[derive(Clone)]
struct Moon {
    position: Point,
    velocity: Point,
}

impl Moon {
    fn new(position: Point) -> Moon {
        Moon {
            position,
            velocity: Point::new(0, 0, 0),
        }
    }
    fn update_pos(&mut self) {
        self.position.x += self.velocity.x;
        self.position.y += self.velocity.y;
        self.position.z += self.velocity.z;
    }
    fn update_vel(&mut self, other_pos: Point) {
        if self.position.x < other_pos.x {
            self.velocity.x += 1;
        } else if self.position.x > other_pos.x {
            self.velocity.x -= 1;
        }
        if self.position.y < other_pos.y {
            self.velocity.y += 1;
        } else if self.position.y > other_pos.y {
            self.velocity.y -= 1;
        }
        if self.position.z < other_pos.z {
            self.velocity.z += 1;
        } else if self.position.z > other_pos.z {
            self.velocity.z -= 1;
        }
    }
    fn energy(&self) -> i32 {
        let pot = self.position.abs_sum();
        let kin = self.velocity.abs_sum();
        pot * kin // This is a really weird thing to call energy
    }
}

#[derive(Clone, Copy)]
struct SimpleMoon {
    pos: i32,
    vel: i32,
}

impl SimpleMoon {
    fn new(pos: i32, vel: i32) -> SimpleMoon {
        SimpleMoon { pos, vel }
    }
    fn update_pos(&mut self) {
        self.pos += self.vel;
    }
    fn update_vel(&mut self, other: SimpleMoon) {
        if self.pos < other.pos {
            self.vel += 1;
        } else if self.pos > other.pos {
            self.vel -= 1;
        }
    }
}

#[derive(Clone)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl Point {
    fn new(x: i32, y: i32, z: i32) -> Point {
        Point { x, y, z }
    }
    fn parse_line(line: &str) -> Point {
        let line = line.trim_matches(|c| c == '<' || c == '>');
        let mut sp1 = line.split(",").map(|s| s.split("=").nth(1).unwrap());
        let act = |x: Option<&str>| x.unwrap().parse::<i32>().unwrap();
        Point::new(act(sp1.next()), act(sp1.next()), act(sp1.next()))
    }
    fn abs_sum(&self) -> i32 {
        self.x.abs() + self.y.abs() + self.z.abs()
    }
}

fn triple_lcm(a: i64, b: i64, c: i64) -> i64 {
    (a.lcm(&b)).lcm(&c)
}

fn long_sim(mut moons: Vec<SimpleMoon>) -> i64 {
    let init_pos = moons.clone();
    let len = moons.len();
    // This turns out to be big enough
    for count in 1..1000000 {
        let even = (count % 2) == 0;
        for i in 0..len {
            for j in 0..len {
                let m2 = moons[j];
                let m1 = &mut moons[i];
                m1.update_vel(m2);
            }
        }
        for m in moons.iter_mut() {
            m.update_pos();
        }
        let eq = moons
            .iter()
            .zip(init_pos.iter())
            .all(|(m1, m2)| m1.pos == m2.pos);
        if even && eq {
            return count;
        }
    }
    unimplemented!()
}

pub fn p12(input: &str) -> Answer {
    let mut moons: Vec<_> = input
        .lines()
        .map(|line| Moon::new(Point::parse_line(line)))
        .collect();
    let len = moons.len();

    for _ in 0..1000 {
        for i in 0..len {
            for j in 0..len {
                let m2 = moons[j].position.clone();
                let m1 = &mut moons[i];
                m1.update_vel(m2);
            }
        }
        for m in moons.iter_mut() {
            m.update_pos();
        }
    }
    let ans1: i32 = moons.iter().map(|m| m.energy()).sum();
    let xs = moons
        .clone()
        .into_iter()
        .map(|m| SimpleMoon::new(m.position.x, m.velocity.x))
        .collect();
    let ys = moons
        .clone()
        .into_iter()
        .map(|m| SimpleMoon::new(m.position.y, m.velocity.y))
        .collect();
    let zs = moons
        .clone()
        .into_iter()
        .map(|m| SimpleMoon::new(m.position.z, m.velocity.z))
        .collect();
    let cycles = [long_sim(xs), long_sim(ys), long_sim(zs)];
    let ans2 = triple_lcm(cycles[0], cycles[1], cycles[2]);
    Answer::new(ans1, ans2)
}