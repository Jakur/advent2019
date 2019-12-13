use super::*;

pub fn p5(input: &str) -> Answer {
    let vec = parse_intcode(input);
    let mut part1 = IntMachine::new(vec.clone());
    let mut output1 = part1.run(vec![1], false);
    let mut part2 = IntMachine::new(vec);
    let mut output2 = part2.run(vec![5], false);
    Answer::new(output1.pop().unwrap(), output2.pop().unwrap())
}