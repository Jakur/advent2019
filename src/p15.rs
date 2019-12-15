use super::*;

use petgraph::algo::{astar, dijkstra};
use petgraph::graph::node_index;
use petgraph::graphmap::UnGraphMap;

#[derive(Clone, Copy, PartialEq)]
enum Direction {
    North,
    South,
    West,
    East,
}

impl Direction {
    fn opposite(self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
            Direction::East => Direction::West,
        }
    }
    fn value(self) -> i64 {
        match self {
            Direction::North => 1,
            Direction::South => 2,
            Direction::West => 3,
            Direction::East => 4,
        }
    }
}

struct State {
    machine: IntMachine,
    goal: usize,
    goal_x: i64,
    goal_y: i64,
}

impl State {
    fn new(machine: IntMachine) -> State {
        State {
            machine,
            goal: 0,
            goal_x: 0,
            goal_y: 0,
        }
    }
}

fn search(
    x: i64,
    y: i64,
    dir: Direction,
    m: &mut State,
    e: &mut Vec<((i64, i64), (i64, i64))>,
    exp: &mut HashSet<(i64, i64)>,
) {
    if !(x == 0 && y == 0) {
        let res = step(m, dir.value());
        if res == 0 {
            return;
        } else if res == 2 {
            m.goal = exp.len(); // We only care about its index in the graph
            m.goal_x = x;
            m.goal_y = y;
        }
        let (source_x, source_y) = match dir {
            Direction::North => (x, y - 1),
            Direction::South => (x, y + 1),
            Direction::West => (x + 1, y),
            Direction::East => (x - 1, y),
        };
        e.push(((source_x, source_y), (x, y)));
    }
    exp.insert((x, y));
    if !exp.contains(&(x, y + 1)) {
        search(x, y + 1, Direction::North, m, e, exp);
    }
    if !exp.contains(&(x, y - 1)) {
        search(x, y - 1, Direction::South, m, e, exp);
    }
    if !exp.contains(&(x - 1, y)) {
        search(x - 1, y, Direction::West, m, e, exp);
    }
    if !exp.contains(&(x + 1, y)) {
        search(x + 1, y, Direction::East, m, e, exp);
    }
    if !(x == 0 && y == 0) {
        step(m, dir.opposite().value());
    }
}

fn step(m: &mut State, int_dir: i64) -> i64 {
    let out = m.machine.run(vec![int_dir], false);
    out[0]
}

pub fn p15(input: &str) -> Answer {
    let mem = parse_intcode(input);
    let mut state = State::new(IntMachine::new(mem));
    let mut edges = Vec::new();
    let mut explored = HashSet::new();
    search(
        0,
        0,
        Direction::North, // Dummy
        &mut state,
        &mut edges,
        &mut explored,
    );
    let gr = UnGraphMap::<_, ()>::from_edges(&edges[..]);
    let gr = gr.into_graph::<u32>();
    let goal = state.goal;
    let path = astar(
        &gr,
        node_index(0),
        |finish| finish == node_index(goal),
        |_| 1,
        |i| {
            let (x, y) = gr[i];
            (x - state.goal_x).abs() + (y - state.goal_y).abs()
        },
    )
    .unwrap();
    let ans1 = path.0;
    let distance = dijkstra(&gr, node_index(goal), None, |_| 1)
        .into_iter()
        .map(|(_k, v)| v)
        .max();
    Answer::new(ans1, distance.unwrap())
}
