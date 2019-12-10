use std::collections::HashSet;

struct Map {
    asteroids: HashSet<(usize, usize)>
}


fn parse(input: &str) -> Map {
    let map: Vec<Vec<char>> = input.split("\n").map(|line| line.chars().collect()).collect();

    let width = map[0].len();
    let height = map.len();
    let mut asteroids = HashSet::new();
    for y in 0..height {
        for x in 0..width {
            if map[y][x] == '#' {
                asteroids.insert((x,y));
            }
        }
    }

    Map { asteroids }
}


fn v2_dot(v0: (isize, isize), v1: (isize, isize)) -> isize  {
    v0.0 * v1.0 + v0.1 * v1.1
}


fn v2_len(v: (isize,  isize)) -> f64 {
    ((v.0.pow(2) + v.1.pow(2)) as f64).sqrt()
}


fn v2_angle(v0: (isize, isize),  v1: (isize, isize)) -> f64 {
    let angle = (v2_dot(v0, v1) as f64 / (v2_len(v0) * v2_len(v1))).acos();
    if v1.0 < v0.0 {
        2.0 * std::f64::consts::PI - angle
    } else {
        angle
    }
}


fn line_of_sight(from: (usize, usize), to: (usize, usize)) -> impl Iterator<Item = (f64,f64)> {
    let (ax, ay) = from;
    let (bx, by) = to;

    let dx = bx as isize - ax as isize;
    let dy = by as isize - ay as isize;

    let steps = std::cmp::max(dx.abs(), dy.abs()) as usize;

    let step_x = dx as f64 / steps as f64;
    let step_y = dy as f64 / steps as f64;

    (1..=steps).map(move |step| (ax as f64 + step_x * step as f64, ay as f64 + step_y * step as f64))
}


fn visible_from(map: &Map, pos: (usize, usize)) -> Vec<(usize, usize)> {
    let mut visible_asteroids = Vec::new();
    for asteroid in &map.asteroids {
        let first_hit = line_of_sight(pos, *asteroid)
            .filter(|(x,y)| x.fract() == 0.0 && y.fract()  == 0.0)
            .map(|(x,y)| (x as usize, y as usize))
            .find(|pos| map.asteroids.contains(&pos));

        if let Some(pos) = first_hit {
            if !visible_asteroids.contains(&pos) {
                visible_asteroids.push(pos);
            }
        }
    }
    visible_asteroids
}


fn part_one(map: &Map)  -> ((usize, usize), usize) {
    map.asteroids.iter()
        .map(|&pos| (pos, visible_from(&map, pos).len()))
        .max_by_key(|(_, num_visible)| *num_visible)
        .unwrap()
}


fn part_two(map: &mut Map, pos: (usize, usize))  -> usize {
    let v0: (isize, isize) = (0, 0 - pos.1 as isize);

    let mut lasered_asteroids = Vec::new();
    while lasered_asteroids.len() < 200 {
        let mut asteroids = visible_from(&map, pos);
        asteroids.sort_by(|a,b| {
            let va = (a.0 as isize - pos.0 as isize, a.1 as isize - pos.1 as isize);
            let vb = (b.0 as isize - pos.0 as isize, b.1 as isize - pos.1 as isize);
            return v2_angle(v0, va).partial_cmp(&v2_angle(v0, vb)).unwrap();
        });

        for asteroid in asteroids.iter() {
            map.asteroids.remove(&asteroid);
        }

        lasered_asteroids.append(&mut asteroids);
    }

    lasered_asteroids[199].0 * 100 + lasered_asteroids[199].1
}


fn main() {
    let input = std::fs::read_to_string("input.txt")
        .expect("Failed to read input file");

    let mut map = parse(&input.trim_end());
    let (max_pos, max_visible) = part_one(&map);
    println!("Part one: {}", max_visible);
    println!("Part two: {}", part_two(&mut map, max_pos));
}
