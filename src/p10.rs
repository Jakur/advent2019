use super::Answer;

use std::collections::HashSet;

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