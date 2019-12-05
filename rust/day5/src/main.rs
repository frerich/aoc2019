mod intcode;

fn main() -> std::io::Result<()> {
    let input = std::fs::read_to_string("input.txt")
        .expect("Failed to read input.txt");

    let mut ints = intcode::parse(&input)
        .expect("Failed to parse input file");

    intcode::run(&mut ints).map(|_| ())
}
