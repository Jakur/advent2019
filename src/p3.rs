use super::*;

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