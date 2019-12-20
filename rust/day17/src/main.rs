use intcode;
use std::collections::HashSet;


fn part_one(program: &[isize]) -> usize {
    let mut scaffolds = HashSet::new();
    let mut x = 0;
    let mut y = 0;
    let mut process = intcode::Process::new(program);
    process.run(
        |ascii| match (ascii as u8) as char {
            '#'  => { scaffolds.insert((x,y)); x += 1; },
            '\n' => { x = 0; y += 1; },
            _    => x += 1
        },
        || 0
    );

    scaffolds.iter()
        .filter(|&(x,y)|
            scaffolds.contains(&(x - 1, *y)) &&
            scaffolds.contains(&(x + 1, *y)) &&
            scaffolds.contains(&(*x, y - 1)) &&
            scaffolds.contains(&(*x, y + 1))
        )
        .map(|(x, y)| (x * y) as usize)
        .sum()
}


fn main() {
    let input = std::fs::read_to_string("input.txt")
        .expect("Failed to read input.txt");

    let program = intcode::parse(input.trim_end())
        .expect("Failed to parse input file");

    println!("Part one: {}", part_one(&program));
}
