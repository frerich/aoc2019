pub fn parse(input: &str) -> Option<Vec<isize>> {
    input.trim().split(",").map(|s| s.parse().ok()).collect()
}


fn decode_arg(mem: &[isize], ip: &usize, idx: u32) -> isize {
    let opcode = mem[*ip];
    let arg = mem[*ip + 1 + idx as usize];
    if (opcode % 10_isize.pow(idx + 3)) / 10_isize.pow(idx + 2) == 1 {
        arg
    } else {
        mem[arg as usize]
    }
}


pub fn step(mem: &mut [isize], ip: &mut usize, mut put: impl FnMut(isize), mut get: impl FnMut() -> isize) -> u8 {
    let opcode = (mem[*ip] % 100) as u8;
    match opcode {
        1 => {
            let arg_0 = decode_arg(&mem, ip, 0);
            let arg_1 = decode_arg(&mem, ip, 1);
            let arg_2 = mem[*ip + 3];
            mem[arg_2 as usize] = arg_0 + arg_1;
            *ip += 4;
        }
        2 => {
            let arg_0 = decode_arg(&mem, ip, 0);
            let arg_1 = decode_arg(&mem, ip, 1);
            let arg_2 = mem[*ip + 3];
            mem[arg_2 as usize] = arg_0 * arg_1;
            *ip += 4;
        } 
        3 => {
            let arg_0 = mem[*ip + 1];
            mem[arg_0 as usize] = get();
            *ip += 2;
        }
        4 => {
            let arg_0 = decode_arg(&mem, ip, 0);
            put(arg_0);
            *ip += 2;
        }
        5 => {
            let arg_0 = decode_arg(&mem, ip, 0);
            if arg_0 != 0 {
                *ip = decode_arg(&mem, ip, 1) as usize;
            } else {
                *ip += 3;
            }
        }
        6 => {
            let arg_0 = decode_arg(&mem, ip, 0);
            if arg_0 == 0 {
                *ip = decode_arg(&mem, ip, 1) as usize;
            } else {
                *ip += 3;
            }
        }
        7 => {
            let arg_0 = decode_arg(&mem, ip, 0);
            let arg_1 = decode_arg(&mem, ip, 1);
            let arg_2 = mem[*ip + 3];
            if arg_0 < arg_1 {
                mem[arg_2 as usize] = 1;
            } else {
                mem[arg_2 as usize] = 0;
            }
            *ip += 4
        }
        8 => {
            let arg_0 = decode_arg(&mem, ip, 0);
            let arg_1 = decode_arg(&mem, ip, 1);
            let arg_2 = mem[*ip + 3];
            if arg_0 == arg_1 {
                mem[arg_2 as usize] = 1;
            } else {
                mem[arg_2 as usize] = 0;
            }
            *ip += 4
        }
        99 => {
        }
        n => {
            panic!("Unexpected opcode: {:?}", n);
        }
    }
    opcode
}
