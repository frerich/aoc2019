extern crate permute;

use std::collections::VecDeque;
use permute::permutations_of;

mod intcode;


struct Amplifier {
    ip: usize,
    mem: Vec<isize>,
    input: VecDeque<isize>
}


impl Amplifier {
    fn new(phase_signal: isize, program: &[isize]) -> Amplifier {
        Amplifier {
            ip: 0,
            mem: program.to_vec(),
            input: vec![phase_signal].into_iter().collect()
        }
    }

    /// Run Amplifier until Some output is produced, then suspends. None is
    /// is returned in case the amplifier program halts without producing
    /// any output.
    fn get_output(&mut self) -> Option<isize> {
        let mut output = None;
        let input = &mut self.input;
        loop {
            let opcode = intcode::step(
                &mut self.mem,
                &mut self.ip,
                |val| output = Some(val),
                || input.pop_front().expect("Missing input")
            );

            if opcode == 4 || opcode == 99 {
                break;
            }
        }
        output
    }
}

struct AmplifierSequence {
    amps: Vec<Amplifier>
}

impl AmplifierSequence {
    fn new (program: &[isize], phase_signals: impl Iterator<Item = isize>) -> AmplifierSequence {
        AmplifierSequence {
            amps: phase_signals
                    .map(|phase_signal| Amplifier::new(phase_signal, program))
                    .collect()
        }
    }

    fn get_output(self: &mut AmplifierSequence, input: isize) -> Option<isize> {
        let mut next_input = input;
        for amp in &mut self.amps {
            amp.input.push_back(next_input);
            match amp.get_output() {
                Some(output) => next_input = output,
                None         => return None
            }
        }
        Some(next_input)
    }
}


fn part_one(program: &[isize]) -> isize {
    permutations_of(&[0 as isize, 1, 2, 3, 4])
        .map(|phase_signals| {
            let mut sequence = AmplifierSequence::new(program, phase_signals.cloned());
            sequence.get_output(0).unwrap_or(0)
        })
        .max().unwrap_or(0)
}


fn part_two(program: &[isize]) -> isize {
    permutations_of(&[5 as isize,6,7,8,9])
        .map(|phase_signals| {
            let mut sequence = AmplifierSequence::new(program, phase_signals.cloned());

            let mut input = 0;
            while let Some(thrust) = sequence.get_output(input) {
                input = thrust;
            }
            input
        })
        .max().unwrap_or(0)
}


fn main() {
    let input = std::fs::read_to_string("input.txt")
        .expect("Failed to read input.txt");

    let program = intcode::parse(&input)
        .expect("Failed to parse input file");

    println!("Part one: {:?}", part_one(&program));
    println!("Part two: {:?}", part_two(&program));
}
