use super::*;

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