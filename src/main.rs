use advent::*;

fn main() {
    let problem = 11;
    let input = get_input(problem);
    let output = problem_multiplex(problem)(&input);
    println!("{}", output);
}
