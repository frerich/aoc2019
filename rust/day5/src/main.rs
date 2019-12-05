use std::io::{self, Read, Write};


fn parse(input: &str) -> Option<Vec<isize>> {
    input.trim().split(",").map(|s| s.parse().ok()).collect()
}


fn decode_arg(mem: &[isize], ip: usize, idx: u32) -> isize {
    let opcode = mem[ip];
    let arg = mem[ip + 1 + idx as usize];
    if (opcode % 10_isize.pow(idx + 3)) / 10_isize.pow(idx + 2) == 1 {
        arg
    } else {
        mem[arg as usize]
    }
}


fn run(opcodes: &[isize]) {
    let mut mem = opcodes.to_vec();
    let mut ip = 0;
    loop {
        match mem[ip] % 100 {
            1 => {
                let arg_0 = decode_arg(&mem, ip, 0);
                let arg_1 = decode_arg(&mem, ip, 1);
                let arg_2 = mem[ip + 3];
                mem[arg_2 as usize] = arg_0 + arg_1;
                ip += 4;
            }
            2 => {
                let arg_0 = decode_arg(&mem, ip, 0);
                let arg_1 = decode_arg(&mem, ip, 1);
                let arg_2 = mem[ip + 3];
                mem[arg_2 as usize] = arg_0 * arg_1;
                ip += 4;
            } 
            3 => {
                let arg_0 = mem[ip + 1];
                let mut buf = [0; 1];
                io::stdin().read(&mut buf);
                mem[arg_0 as usize] = buf[0] as isize - 48;
                ip += 2;
            }
            4 => {
                let arg_0 = decode_arg(&mem, ip, 0);
                let chars: Vec<u8> = arg_0.to_string().bytes().collect();
                io::stdout().write(&chars);
                ip += 2;
            }
            5 => {
                let arg_0 = decode_arg(&mem, ip, 0);
                if arg_0 != 0 {
                    ip = decode_arg(&mem, ip, 1) as usize;
                } else {
                    ip += 3;
                }
            }
            6 => {
                let arg_0 = decode_arg(&mem, ip, 0);
                if arg_0 == 0 {
                    ip = decode_arg(&mem, ip, 1) as usize;
                } else {
                    ip += 3;
                }
            }
            7 => {
                let arg_0 = decode_arg(&mem, ip, 0);
                let arg_1 = decode_arg(&mem, ip, 1);
                let arg_2 = mem[ip + 3];
                if arg_0 < arg_1 {
                    mem[arg_2 as usize] = 1;
                } else {
                    mem[arg_2 as usize] = 0;
                }
                ip += 4
            }
            8 => {
                let arg_0 = decode_arg(&mem, ip, 0);
                let arg_1 = decode_arg(&mem, ip, 1);
                let arg_2 = mem[ip + 3];
                if arg_0 == arg_1 {
                    mem[arg_2 as usize] = 1;
                } else {
                    mem[arg_2 as usize] = 0;
                }
                ip += 4
            }
            99 => {
                break
            }
            n => {
                panic!("Unexpected opcode: {:?}", n);
            }
        }
    }
}


fn main() {
    let input = std::fs::read_to_string("input.txt")
        .expect("Failed to read input.txt");

    let opcodes = parse(&input)
        .expect("Failed to parse input file");

    run(&opcodes);
}
