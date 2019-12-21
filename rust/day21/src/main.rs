use intcode;

fn run_springdroid(program: &[isize], commands: &[&str]) -> Option<usize> {
    let input = commands.join("\n") + "\n";
    let mut output = None;

    let mut input_it = input.chars();
    let mut process = intcode::Process::new(&program);
    process.run(
        |val| {
            if val < 256 {
                print!("{}", (val as u8) as char);
            } else {
                output = Some(val as usize);
            }
        },
        || input_it.next().unwrap() as isize
    );
    output
}


fn part_one(program: &[isize]) {
    println!("Part one\n--------");
    let result = run_springdroid(
        program,
        &["OR A J", "AND B J", "AND C J", "NOT J J", "AND D J", "WALK"]
    );

    if let Some(hull_dmg) = result {
        println!("Hull damage: {}", hull_dmg);
    }
}


fn main() {
    let input = std::fs::read_to_string("input.txt")
        .expect("Failed to read input.txt");

    let program = intcode::parse(input.trim_end())
        .expect("Failed to parse input file");

    part_one(&program);
}
