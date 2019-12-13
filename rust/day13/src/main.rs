use std::cell::Cell;


mod intcode;


fn run_game(program: &[isize], mut update_score: impl FnMut(usize), mut print_tile: impl FnMut(u32, u32, char), poll_joystick: impl Fn() -> isize) {
    let mut output_queue = Vec::new();
    let mut process = intcode::Process::new(program);

    process.run(
        |val| {
            if output_queue.len() < 2 {
                output_queue.push(val);
                return;
            }

            if output_queue[0] == -1 && output_queue[1] == 0 {
                update_score(val as usize);
            } else {
                let ch =  match val {
                    0 => ' ',
                    1 => '#',
                    2 => 'X',
                    3 => '=',
                    4 => '*',
                    _ => ' '
                };
                print_tile(output_queue[0] as u32, output_queue[1] as u32, ch);
            }
            output_queue.clear();
        },
        poll_joystick
    );
}


fn part_one(program: &[isize]) -> usize {
    let mut num_blocks = 0;
    run_game(
        program,
        |_| { },
        |_, _, tile| if tile == 'X' { num_blocks += 1; },
        || 0
    );
    num_blocks
}


fn part_two(program: &[isize]) -> usize {
    let mut mem = program.to_vec();
    mem[0] = 2;

    let mut score = 0;
    let ball_x: Cell<u32> = Cell::new(0);
    let paddle_x: Cell<u32> = Cell::new(0);

    run_game(
        &mem,
        |n| score = n,
        |x, _, tile| match tile {
            '=' => paddle_x.set(x),
            '*' => ball_x.set(x),
            _   => {}
        },
        || (ball_x.get() as isize - paddle_x.get() as isize).signum()
    );

    score
}


fn main() {
    let input = std::fs::read_to_string("input.txt")
        .expect("Failed to read input.txt");

    let program = intcode::parse(input.trim_end())
        .expect("Failed to parse input file");

    println!("Part one: {}", part_one(&program));
    println!("Part two: {}", part_two(&program));
}
