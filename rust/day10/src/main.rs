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

fn visible_from(map: &Map, pos: (usize, usize)) -> HashSet<(usize, usize)> {
    let mut seen_asteroids =  HashSet::new();
    for asteroid in &map.asteroids {
        let los = line_of_sight(pos, *asteroid);
        for (x, y) in los {
            if x.fract() == 0.0 && y.fract() == 0.0 {
                let ix = x.floor() as usize;
                let iy = y.floor() as usize;
                if map.asteroids.contains(&(ix, iy)) {
                    seen_asteroids.insert((ix, iy));
                    break;
                }
            }
        }
    }
    seen_asteroids
}


fn part_one(map: &Map)  -> usize {
    map.asteroids.iter().map(|pos| visible_from(&map, *pos).len()).max().unwrap_or(0)
}


fn main() {
    let input = std::fs::read_to_string("input.txt")
        .expect("Failed to read input file");

    let map = parse(&input.trim_end());
    println!("Part one: {}", part_one(&map));

}
