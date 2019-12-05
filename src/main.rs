use advent::{get_input, p1, p2, p3, p4, p5};

fn main() {
    let problem = 2;
    let input = get_input(problem);
    let output = match problem {
        1 => p1(&input),
        2 => p2(&input),
        3 => p3(&input),
        4 => p4(&input),
        5 => p5(&input),
        _ => unimplemented!(),
    };
    println!("{}", output);
}