pub struct Process {
    memory: Vec<isize>,
    ip: usize,
    rb: usize
}

impl Process {
    pub fn new(program: &[isize]) -> Process {
        Process {
            memory: program.to_vec(),
            ip: 0,
            rb: 0
        }
    }

    fn cell(self: &mut Process, addr: usize) -> &mut isize {
        if addr >= self.memory.len() {
            self.memory.resize(addr + 1, 0);
        }
        &mut self.memory[addr]
    }

    fn value_arg(self: &mut Process, idx: u32) -> isize {
        let opcode = *self.cell(self.ip) as usize;
        let arg = *self.cell(self.ip + 1 + idx as usize);

        let mode = (opcode % 10_usize.pow(idx + 3)) / 10_usize.pow(idx + 2);

        match mode {
            0 => *self.cell(arg as usize),
            1 => arg,
            2 => *self.cell((self.rb as isize + arg) as usize),
            _ => panic!("Unsupported parameter mode")
        }
    }

    fn addr_arg(self: &mut Process, idx: u32) -> usize {
        let opcode = *self.cell(self.ip) as usize;
        let arg = *self.cell(self.ip + 1 + idx as usize);

        let mode = (opcode % 10_usize.pow(idx + 3)) / 10_usize.pow(idx + 2);

        match mode {
            0 => arg as usize,
            1 => panic!("Immediate parameter for address arguments not supported"),
            2 => (self.rb as isize + arg) as usize,
            _ => panic!("Unsupported parameter mode")
        }
    }

    pub fn step(self: &mut Process, mut put: impl FnMut(isize), mut get: impl FnMut() -> isize) -> u8 {
        let opcode = (*self.cell(self.ip) % 100) as u8;
        match opcode {
            1 => {
                let addr = self.addr_arg(2);
                *self.cell(addr) = self.value_arg(0) + self.value_arg(1);
                self.ip += 4;
            }
            2 => {
                let addr = self.addr_arg(2);
                *self.cell(addr) = self.value_arg(0) * self.value_arg(1);
                self.ip += 4;
            } 
            3 => {
                let addr = self.addr_arg(0);
                *self.cell(addr) = get();
                self.ip += 2;
            }
            4 => {
                put(self.value_arg(0));
                self.ip += 2;
            }
            5 => {
                if self.value_arg(0) != 0 {
                    self.ip = self.value_arg(1) as usize;
                } else {
                    self.ip += 3;
                }
            }
            6 => {
                if self.value_arg(0) == 0 {
                    self.ip = self.value_arg(1) as usize;
                } else {
                    self.ip += 3;
                }
            }
            7 => {
                let addr = self.addr_arg(2);
                *self.cell(addr) = if self.value_arg(0) < self.value_arg(1) { 1 } else { 0 };
                self.ip += 4
            }
            8 => {
                let addr = self.addr_arg(2);
                *self.cell(addr) = if self.value_arg(0) == self.value_arg(1) { 1 } else { 0 };
                self.ip += 4
            }
            9 => {
                self.rb = (self.rb as isize + self.value_arg(0)) as usize;
                self.ip += 2;
            }
            99 => {
            }
            n => {
                panic!("Unexpected opcode: {:?}", n);
            }
        }
        opcode
    }

    pub fn run(self: &mut Process, mut put: impl FnMut(isize), mut get: impl FnMut() -> isize) {
        while self.step(&mut put, &mut get) != 99 {
        }
    }
}

pub fn parse(input: &str) -> Option<Vec<isize>> {
    input.split(",").map(|s| s.parse().ok()).collect()
}

