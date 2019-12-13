use super::*;

pub fn p9(input: &str) -> Answer {
    let mem = parse_intcode(input);
    let mut machine = IntMachine::new(mem.clone());
    let out1 = machine.run(vec![1], false);
    machine = IntMachine::new(mem);
    let out2 = machine.run(vec![2], false);
    Answer::new(out1[0], out2[0])
}