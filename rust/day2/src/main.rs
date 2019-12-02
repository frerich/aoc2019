fn parse(input: &str) -> Option<Vec<usize>> {
    input.trim().split(",").map(|s| s.parse().ok()).collect()
}


fn run(opcodes: &[usize], noun: usize, verb: usize) -> usize {
    let mut mem = opcodes.to_vec();
    mem[1] = noun;
    mem[2] = verb;

    let mut ip = 0;;
    loop {
        match mem[ip] {
            1 => {
                let addr_0 = mem[ip + 1];
                let addr_1 = mem[ip + 2];
                let addr_2 = mem[ip + 3];
                mem[addr_2] = mem[addr_0] + mem[addr_1];
                ip += 4;
            }
            2 => {
                let addr_0 = mem[ip + 1];
                let addr_1 = mem[ip + 2];
                let addr_2 = mem[ip + 3];
                mem[addr_2] = mem[addr_0] * mem[addr_1];
                ip += 4;
            }
            99 => {
                break
            }
            _ => {
                panic!("Unexpected opcode");
            }
        }
    }
    mem[0]
}


fn part_one(opcodes: &[usize]) -> usize {
    run(opcodes, 12, 2)
}


fn part_two(opcodes: &[usize]) -> usize {
    for noun in 0..100 {
        for verb in 0..100 {
            if run(opcodes, noun, verb) == 19690720 {
                return 100 * noun + verb;
            }
        }
    }
    panic!("Unreachable")
}


fn main() {
    let input = std::fs::read_to_string("input.txt")
        .expect("Failed to read input.txt");

    let opcodes = parse(&input)
        .expect("Failed to parse input file");

    println!("Part One: {:?}", part_one(&opcodes));
    println!("Part Two: {:?}", part_two(&opcodes));
}
