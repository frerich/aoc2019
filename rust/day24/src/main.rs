use std::collections::HashSet;
use std::iter::successors;


fn adjacent(area: &[(i32,i32)], x: i32, y: i32) -> Vec<(i32,i32)> {
    [(0,-1),(0,1),(-1,0),(1,0)].iter()
        .map(|(dx,dy)| (x + dx, y + dy))
        .filter(|pos| area.contains(pos))
        .collect()
}


fn biodiversity(area: &[(i32,i32)]) -> usize {
    area.iter()
        .map(|(x,y)| 2_usize.pow((y * 5 + x) as u32))
        .sum()
}


fn evolve(area: &[(i32,i32)]) -> Vec<(i32,i32)> {
    let mut result = Vec::new();
    for y in 0..5 {
        for x in 0..5 {
            let num_adjacent = adjacent(area, x, y).len();
            if area.contains(&(x,y)) {
                if num_adjacent == 1 {
                    result.push((x,y));
                }
            } else {
                if num_adjacent >= 1 && num_adjacent <= 2 {
                    result.push((x,y));
                }
            }
        }
    }
    result
}


fn parse(input: &str) -> Vec<(i32,i32)> {
    let mut result = Vec::new();
    for (y, line) in input.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            if ch == '#' {
                result.push((x as i32, y as i32));
            }
        }
    }
    result
}


fn part_one(input: &str) -> usize {
    let evolutions = successors(
        Some(parse(&input)),
        |area| Some(evolve(&area))
    );

    let mut seen = HashSet::new();
    for evolution in evolutions {
        let diversity = biodiversity(&evolution);
        if seen.contains(&diversity) {
            return diversity;
        }
        seen.insert(diversity);
    }
    0
}


fn main() {
    let input = std::fs::read_to_string("input.txt")
        .expect("Failed to read input.txt");

    println!("Part one: {}", part_one(&input));
}
