
#[derive(Clone)]
pub struct IntMachine {
    pub ptr: usize,
    pub mem: Vec<i32>,
}

impl IntMachine {
    pub fn new(instructions: Vec<i32>) -> IntMachine {
        IntMachine {
            ptr: 0,
            mem: instructions,
        }
    }

    fn f_parm(&self, offset: usize, mode: i32) -> String {
        let loc = self.ptr + offset;
        match mode {
            0 => format!("[{}]={}", loc, self.mode_fn(mode, loc).unwrap()),
            1 => format!("#{}", self.mem[loc]),
            _ => unimplemented!(),
        }
    }

    pub fn run(&mut self, input: Vec<i32>, debug: bool) -> Vec<i32> {
        let mut input = input.into_iter();
        let mut output = Vec::new();
        loop {
            let x = self.mem[self.ptr];
            let opcode = self.mem[self.ptr] % 100;
            let mode1 = (x / 100) % 10;
            let mode2 = (x / 1000) % 10;
            let v1 = self.mode_fn(mode1, self.ptr + 1);
            let v2 = self.mode_fn(mode2, self.ptr + 2);
            let loc3 = self.mem.get(self.ptr + 3).map(|v| *v as usize);
            // let v3 = mode_fn((x / 10000) % 10, self.ptr + 3, &self.mem);
            if debug {
                let s = match opcode {
                    1 => format!(
                        "ADD {} {} -> [{}]",
                        self.f_parm(1, mode1),
                        self.f_parm(2, mode2),
                        self.ptr + 3
                    ),
                    2 => format!(
                        "MUL {} {} -> [{}]",
                        self.f_parm(1, mode1),
                        self.f_parm(2, mode2),
                        self.ptr + 3
                    ),
                    3 => format!("INPUT {:?} -> [{}]", input.clone().next(), self.ptr + 1),
                    4 => format!("OUTPUT <- {}", self.f_parm(1, mode1)),
                    5 => format!("JNZ {} TO {}", self.f_parm(1, mode1), self.f_parm(2, mode2)),
                    6 => format!("JEZ {} TO {}", self.f_parm(1, mode1), self.f_parm(2, mode2)),
                    7 => format!(
                        "{} < {} -> [{}]",
                        self.f_parm(1, mode1),
                        self.f_parm(2, mode2),
                        self.ptr + 3
                    ),
                    8 => format!(
                        "{} == {} -> [{}]",
                        self.f_parm(1, mode1),
                        self.f_parm(2, mode2),
                        self.ptr + 3
                    ),
                    99 => format!("HALT"),
                    _ => {
                        println!("DUMP: {:?}", self.mem);
                        unimplemented!()
                    }
                };
                println!("{}", s);
            }
            match opcode {
                1 => {
                    self.mem[loc3.unwrap()] = v1.unwrap() + v2.unwrap();
                    self.ptr += 4;
                }
                2 => {
                    self.mem[loc3.unwrap()] = v1.unwrap() * v2.unwrap();
                    self.ptr += 4;
                }
                3 => {
                    let v = self.mem[self.ptr + 1] as usize;
                    match input.next() {
                        Some(i) => {
                            self.mem[v] = i;
                            self.ptr += 2;
                        }
                        None => {
                            return output; // Part 7
                        }
                    }
                }
                4 => {
                    output.push(v1.unwrap());
                    self.ptr += 2;
                }
                5 => {
                    if v1.unwrap() != 0 {
                        self.ptr = v2.unwrap() as usize;
                    } else {
                        self.ptr += 3;
                    }
                }
                6 => {
                    if v1.unwrap() == 0 {
                        self.ptr = v2.unwrap() as usize;
                    } else {
                        self.ptr += 3;
                    }
                }
                7 => {
                    let out = (v1.unwrap() < v2.unwrap()).into();
                    self.mem[loc3.unwrap()] = out;
                    self.ptr += 4;
                }
                8 => {
                    let out = (v1.unwrap() == v2.unwrap()).into();
                    self.mem[loc3.unwrap()] = out;
                    self.ptr += 4;

                }
                99 => break,
                _ => {
                    println!("Error with {} with self.ptr {}", opcode, self.ptr);
                    println!("Memory dump: {:?}", self.mem);
                    unimplemented!();
                }
            }
        }
        output
    }

    fn mode_fn(&self, mode: i32, pos: usize) -> Option<i32> {
        let value = self.mem.get(pos);
        let output = match mode {
            0 => value.and_then(|v| self.mem.get(*v as usize)),
            1 => value,
            _ => {
                println!("{}", mode);
                println!("{}", pos);
                println!("{:?}", self.mem);
                unimplemented!()
            }
        };
        output.map(|v| *v)
    }
}

pub fn parse_intcode(input: &str) -> Vec<i32> {
    input
        .lines()
        .next()
        .unwrap()
        .split(",")
        .map(|v| v.parse().unwrap())
        .collect()
}