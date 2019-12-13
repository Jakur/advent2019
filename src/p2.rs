use super::*;

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