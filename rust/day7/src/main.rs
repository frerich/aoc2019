extern crate permutohedron;

use std::collections::VecDeque;
use crate::permutohedron::LexicalPermutation;
use std::iter::FromIterator;

mod intcode;

struct Amplifier {
    ip: usize,
    mem: Vec<isize>,
    input: VecDeque<isize>
}

fn run(amp: &mut Amplifier) -> Option<isize> {
    let mut output = None;
    let mut inputs = amp.input.clone();
    loop {
        let opcode = intcode::step(
            &mut amp.mem,
            &mut amp.ip,
            &mut |val| output = Some(val),
            &mut || inputs.pop_front().expect("Missing input")
            );

        if opcode == 4 || opcode == 99 {
            break;
        }
    }
    amp.input = inputs;
    output
}

fn sequence(amplifiers: &mut [Amplifier], first_input: isize) -> Option<isize> {
    let mut input = first_input;
    for mut amp in amplifiers {
        amp.input.push_back(input);
        match run(&mut amp) {
            Some(output) => input = output,
            None         => return None
        };
    }
    Some(input)
}

fn find_maximum_thrust_by(amplifier_program: &[isize], phase_signals: &[isize], go: impl Fn(&mut [Amplifier]) -> isize) -> isize {
    let mut max_thrust = 0;

    let mut phase_permutation = phase_signals.to_vec();

    loop {
        let mut amplifiers = phase_permutation.iter().map(|phase_signal| {
            Amplifier {
                ip: 0,
                mem: amplifier_program.to_vec(),
                input: VecDeque::from_iter(vec![*phase_signal]),
            }
        }).collect::<Vec<_>>();

        max_thrust = std::cmp::max(max_thrust, go(&mut amplifiers));

        if !phase_permutation.next_permutation() {
            break;
        }
    }

    max_thrust
}

fn part_one(ints: &[isize]) -> isize {
    let phase_signals: Vec<_> = (0..5).collect();
    find_maximum_thrust_by(ints, &phase_signals, |mut amplifiers| {
        sequence(&mut amplifiers, 0).unwrap_or(0)
    })
}

fn part_two(ints: &[isize]) -> isize {
    let phase_signals: Vec<_> = (5..10).collect();
    find_maximum_thrust_by(ints, &phase_signals, |mut amplifiers| {
        let mut input = 0;
        loop {
            match sequence(&mut amplifiers, input) {
                Some(thrust) => input = thrust,
                None         => return input
            };
        }
    })
}

fn main() {
    let input = std::fs::read_to_string("input.txt")
        .expect("Failed to read input.txt");

    let ints = intcode::parse(&input)
        .expect("Failed to parse input file");

    println!("Part one: {:?}", part_one(&ints));
    println!("Part two: {:?}", part_two(&ints));
}
