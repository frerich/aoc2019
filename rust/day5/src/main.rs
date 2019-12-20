use intcode;


fn output(program: &[isize], input: isize) -> isize {
    let mut output  = 0;

    let mut process = intcode::Process::new(program);
    process.run(
        |val| output = val,
        || input
    );

    output
}


fn main() {
    let input = std::fs::read_to_string("input.txt")
        .expect("Failed to read input.txt");

    let program = intcode::parse(input.trim_end())
        .expect("Failed to parse input file");

    println!("Part one: {}", output(&program, 1));
    println!("Part two: {}", output(&program, 5));
}
