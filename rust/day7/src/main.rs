extern crate permutohedron;

mod intcode;

fn run_amplifiers(ints: &[isize], phase_signals: &[isize]) -> isize {
    let mut input: isize = 0;
    for phase in phase_signals {
        let mut mem = ints.to_vec();
        let inputs = vec![*phase, input];
        intcode::run(&mut mem, |val| input = val, inputs.into_iter());
    }
    input
}

fn part_one(ints: &[isize]) -> isize {
    let mut phase_signals = vec![0,1,2,3,4];
    permutohedron::Heap::new(&mut phase_signals)
        .into_iter()
        .map(|phase_permutation| run_amplifiers(&ints, &phase_permutation))
        .max()
        .unwrap()
}

fn main() {
    let input = std::fs::read_to_string("input.txt")
        .expect("Failed to read input.txt");

    let ints = intcode::parse(&input)
        .expect("Failed to parse input file");

    println!("Part one: {:?}", part_one(&ints));
}
