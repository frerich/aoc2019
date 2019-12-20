mod intcode;


fn attracted_at(program: &[isize], x: usize, y: usize) -> bool {
    let mut result = false;
    let mut input = vec![x as isize, y as isize].into_iter();
    let mut process = intcode::Process::new(program);
    process.run(
        |out| result = out == 1,
        || input.next().unwrap()
    );
    result

}


fn part_one(program: &[isize]) -> usize {
    let mut num_affected = 0;
    for y in 0..50 {
        for x in 0..50 {
            if attracted_at(program, x, y) {
                num_affected += 1;
            }
        }
    }
    num_affected
}


fn part_two(program: &[isize]) -> usize {
    let mut x = 0;
    let mut y = 99;
    loop {
        while !attracted_at(program, x, y) {
            x += 1;
        }

        if attracted_at(program, x + 99, y - 99) && attracted_at(program, x + 99, y) && attracted_at(program, x, y - 99) {
            return x * 10_000 + (y - 99);
        }

        y += 1;
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
