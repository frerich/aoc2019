use std::collections::HashMap;
use std::collections::HashSet;


mod intcode;


#[derive(Clone, Copy)]
enum Direction  {
    N, S, W, E
}


#[derive(PartialEq, Clone, Copy)]
enum Status {
    Wall,
    Free,
    Oxygen
}


// Runs the droid until it generates a new status for the given input direction.
fn step(droid: &mut intcode::Process, dir: Direction) -> Status {
    let mut status = None;
    while status ==  None {
        droid.step(
            |x| {
                status = match x {
                    0 => Some(Status::Wall),
                    1 => Some(Status::Free),
                    2 => Some(Status::Oxygen),
                    _ => panic!("Unknown status code {} encountered!", x)
                };
            },
            || match dir {
                Direction::N => 1,
                Direction::S => 2,
                Direction::W => 3,
                Direction::E => 4,
            }
        );
    }
    status.unwrap()
}


fn visit(droid: &mut intcode::Process, map: &mut HashMap<(isize,isize), Status>, pos: (isize,isize)) {
    for dir in &[Direction::E, Direction::N, Direction::W, Direction::S] {
        let new_pos = match dir {
            Direction::N => (pos.0, pos.1 - 1),
            Direction::S => (pos.0, pos.1 + 1),
            Direction::W => (pos.0 - 1, pos.1),
            Direction::E => (pos.0 + 1, pos.1)
        };

        if !map.contains_key(&new_pos) {
            let status = step(droid, *dir);

            map.insert(new_pos, status);
            if status != Status::Wall {
                visit(droid, map, new_pos);

                step(droid, match *dir {
                    Direction::N => Direction::S,
                    Direction::S => Direction::N,
                    Direction::W => Direction::E,
                    Direction::E => Direction::W
                });
            }
        }
    }
}


fn explore(program: &[isize]) -> HashMap<(isize,isize), Status> {
    let mut droid = intcode::Process::new(program);
    let mut map: HashMap<(isize, isize), Status> = HashMap::new();
    map.insert((0,0), Status::Free);
    visit(&mut droid, &mut map, (0,0));
    map
}


fn adjacent(pos: &(isize, isize)) -> [(isize,isize); 4] {
    [
        (pos.0, pos.1 - 1),
        (pos.0, pos.1 + 1),
        (pos.0 - 1, pos.1),
        (pos.0 + 1, pos.1)
    ]
}


fn part_one(map: &HashMap<(isize, isize), Status>) -> usize {
    let mut visited = HashSet::new();

    std::iter::successors(Some(vec![(0,0)]), |positions| {
        let mut next_positions = Vec::new();
        for pos in positions {
            if visited.contains(pos) {
                continue;
            }
            visited.insert(pos.clone());

            for next_pos in &adjacent(&pos) {
                match map.get(&next_pos) {
                    Some(Status::Free)   => next_positions.push(*next_pos),
                    Some(Status::Oxygen) => return None,
                    _                    => {}
                }
            }
        }
        Some(next_positions)
    }).count()
}


fn part_two(mut map: HashMap<(isize,isize), Status>) -> usize {
    let oxygen_pos = map.iter()
        .find(|(_,&status)| status == Status::Oxygen)
        .unwrap().0;

    std::iter::successors(Some(vec![*oxygen_pos]), |positions| {
        let mut next_positions = Vec::new();
        for pos in positions {
            for next_pos in &adjacent(&pos) {
                if let Some(Status::Free) = map.get(next_pos) {
                    next_positions.push(*next_pos);
                    map.insert(*next_pos, Status::Oxygen);
                }
            }
        }

        if !next_positions.is_empty() {
            Some(next_positions)
        } else {
            None
        }
    }).count() - 1
}


fn main() {
    let input = std::fs::read_to_string("input.txt")
        .expect("Failed to read input.txt");

    let program = intcode::parse(input.trim_end())
        .expect("Failed to parse input file");

    let map = explore(&program);

    println!("Part one: {}", part_one(&map));
    println!("Part two: {}", part_two(map));
}
