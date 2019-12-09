mod intcode;


fn part_one(program: &[isize]) -> isize {
    let mut output: isize = 0;
    let mut process = intcode::Process::new(program);
    process.run(
        |val| output = val,
        || 1
    );
    output
}


fn part_two(program: &[isize]) -> isize {
    let mut output: isize = 0;
    let mut process = intcode::Process::new(program);
    process.run(
        |val| output = val,
        || 2
    );
    output
}


fn main() {
    let input = std::fs::read_to_string("input.txt")
        .expect("Failed to read input.txt");

    let program = intcode::parse(input.trim_end())
        .expect("Failed to parse input file");

    println!("Part one: {:?}", part_one(&program));
    println!("Part two: {:?}", part_two(&program));
}
