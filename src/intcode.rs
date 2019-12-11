
#[derive(Clone)]
pub struct IntMachine {
    pub ptr: usize,
    pub mem: Vec<i64>,
    pub rel_base: i64,
}

impl IntMachine {
    pub fn new(instructions: Vec<i64>) -> IntMachine {
        IntMachine {
            ptr: 0,
            mem: instructions,
            rel_base: 0,
        }
    }

    pub fn halted(&self) -> bool {
        self.mem[self.ptr] == 99
    }

    fn f_parm(&self, offset: usize, mode: i64) -> String {
        let loc = self.ptr + offset;
        match mode {
            0 => format!("[{}]={}", loc, self.mode_fn(mode, loc).unwrap()),
            1 => format!("#{}", self.mem[loc]),
            2 => format!(
                "[{}]={}",
                (loc as i64) + self.rel_base,
                self.mode_fn(mode, loc).unwrap()
            ),
            _ => unimplemented!(),
        }
    }

    fn mem_write(&mut self, index: usize, value: i64) {
        if index < self.mem.len() {
            self.mem[index] = value;
        } else {
            let offset = index - self.mem.len() + 1;
            let iter = std::iter::repeat(0).take(offset);
            self.mem.extend(iter);
            self.mem[index] = value;
        }
    }

    pub fn run(&mut self, input: Vec<i64>, debug: bool) -> Vec<i64> {
        let mut input = input.into_iter();
        let mut output = Vec::new();
        loop {
            let x = self.mem[self.ptr];
            let opcode = self.mem[self.ptr] % 100;
            let mode1 = (x / 100) % 10;
            let mode2 = (x / 1000) % 10;
            let mode3 = (x / 10000) % 10;
            let v1 = self.mode_fn(mode1, self.ptr + 1);
            let v2 = self.mode_fn(mode2, self.ptr + 2);
            let loc3 = self.mem.get(self.ptr + 3).map(|v| match mode3 {
                0 => *v as usize,
                2 => (*v + self.rel_base) as usize,
                _ => unimplemented!(),
            });
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
                    9 => format!("(Relative Base += Something)"),
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
                    self.mem_write(loc3.unwrap(), v1.unwrap_or(0) + v2.unwrap_or(0));
                    self.ptr += 4;
                }
                2 => {
                    self.mem_write(loc3.unwrap(), v1.unwrap_or(0) * v2.unwrap_or(0));
                    self.ptr += 4;
                }
                3 => {
                    let v = {
                        match mode1 {
                            0 => self.mem[self.ptr + 1] as usize,
                            2 => (self.mem[self.ptr + 1] + self.rel_base) as usize,
                            _ => unimplemented!(),
                        }
                    };
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
                9 => {
                    self.rel_base += v1.unwrap();
                    self.ptr += 2;
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

    fn mode_fn(&self, mode: i64, pos: usize) -> Option<i64> {
        let value = self.mem.get(pos);
        let output = match mode {
            0 => value.and_then(|v| self.mem.get(*v as usize)),
            1 => value,
            2 => value.and_then(|v| self.mem.get((*v + self.rel_base) as usize)),
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

pub fn parse_intcode(input: &str) -> Vec<i64> {
    input
        .lines()
        .next()
        .unwrap()
        .split(",")
        .map(|v| v.parse().unwrap())
        .collect()
}