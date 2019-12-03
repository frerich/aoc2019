use std::collections::HashSet;


fn parse_step(step: &str) -> impl Iterator<Item = (isize, isize)> {
    let mut chars = step.chars();

    let coord = match chars.next() {
        Some('U') => (0, -1),
        Some('D') => (0,  1),
        Some('L') => (-1, 0),
        Some('R') => (1, 0),
        _         => (0, 0)
    };

    let dist: usize  = chars.collect::<String>().parse().unwrap();

    std::iter::repeat(coord).take(dist)
}


fn parse_line(line: &str) -> Vec<(isize, isize)> {
    line.split(",").flat_map(parse_step).scan((0,0), |pos, step| {
        pos.0 += step.0;
        pos.1 += step.1;
        Some(pos.clone())
    }).collect()
}


fn manhattan_distance(pos: &(isize, isize)) -> usize {
    pos.0.abs() as usize + pos.1.abs() as usize
}


fn main() {
    let input = std::fs::read_to_string("input.txt")
        .expect("Failed to read input.txt");

    let mut lines = input.lines();
    let steps_0 = parse_line(lines.next().unwrap());
    let steps_1 = parse_line(lines.next().unwrap());

    let positions_0: HashSet<_> = steps_0.iter().cloned().collect();
    let positions_1: HashSet<_> = steps_1.iter().cloned().collect();
    let collisions = positions_0.intersection(&positions_1);

    let closest_collision = collisions.clone().map(manhattan_distance).min();
    println!("Part One: {:?}", closest_collision.unwrap());

    let quickest_collision = collisions.map(|pos| steps_0.iter().position(|x| x == pos).unwrap() + steps_1.iter().position(|x| x == pos).unwrap() + 2).min();
    println!("Part Two: {:?}", quickest_collision.unwrap());
}
