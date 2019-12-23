use std::collections::HashSet;
use std::collections::VecDeque;
use intcode;


struct NIC {
    in_queue: VecDeque<isize>,
    out_buf: Vec<isize>,
    process: intcode::Process
}


impl NIC {
    fn new(program: &[isize], addr: usize) -> NIC {
        NIC {
            in_queue: [addr as isize].iter().cloned().collect(),
            out_buf: vec![],
            process: intcode::Process::new(program)
        }
    }

    fn step(self: &mut NIC, mut send: impl FnMut(usize, isize, isize)) {
        let out_buf = &mut self.out_buf;
        let in_queue = &mut self.in_queue;
        self.process.step(
            |val| {
                out_buf.push(val);
                if let &[addr, x, y] = out_buf.as_slice() {
                    send(addr as usize, x, y);
                    out_buf.clear();
                }
            },
            || in_queue.pop_front().unwrap_or(-1)
        );
    }
}


struct Network {
    computers: Vec<NIC>,
    nat: Option<(isize,isize)>
}


impl Network {
    fn new(program: &[isize], size: usize) -> Network {
        Network {
            computers: (0..size).map(|addr| NIC::new(program, addr)).collect(),
            nat: None
        }
    }

    fn step(self: &mut Network) {
        for i in 0..self.computers.len() {
            let mut output = None;
            self.computers[i].step(
                |addr, x, y| output = Some((addr, x, y))
            );

            if let Some((addr, x, y)) = output {
                if addr == 255 {
                    self.nat = Some((x,y))
                } else {
                    self.computers[addr].in_queue.push_back(x);
                    self.computers[addr].in_queue.push_back(y);
                }
            }
        }
    }

    fn is_idle(self: &Network) -> bool {
        self.computers.iter().all(|nic| nic.in_queue.is_empty())
    }
}


fn part_one(program: &[isize]) -> isize {
    let mut network = Network::new(program, 50);
    loop {
        network.step();
        if let Some((_x,y)) = network.nat {
            return y;
        }
    }
}


fn part_two(program: &[isize]) -> isize {
    let mut network = Network::new(program, 50);
    let mut natted_packages = HashSet::new();

    loop {
        network.step();
        if network.is_idle() {
            if let Some((x,y)) = network.nat {
                if natted_packages.contains(&(x,y)) {
                    return y;
                }
                natted_packages.insert((x,y));
                network.computers[0].in_queue.push_back(x);
                network.computers[0].in_queue.push_back(y);
                network.nat = None;
            }
        }
    }
}


fn main() {
    let input = std::fs::read_to_string("input.txt")
        .expect("Failed to read input.txt");

    let program = intcode::parse(input.trim_end())
        .expect("Failed to parse input file");

    println!("Part one: {}", part_one(&program));
    println!("Part two: {}", part_two(&program));
}
