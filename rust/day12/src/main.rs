extern crate num;

use num::integer::lcm;


type Vector3 = [isize; 3];


#[derive(Clone)]
struct Moon {
    pos: Vector3,
    vel: Vector3
}


impl Moon {
    fn new(pos: Vector3) -> Moon {
        Moon { pos: pos, vel: [0, 0, 0] }
    }

    fn energy(self: &Moon) -> usize {
        v3_len(&self.vel) * v3_len(&self.pos)
    }
}


fn v3_len(v: &Vector3) -> usize {
    v[0].abs() as usize + v[1].abs() as usize + v[2].abs() as usize
}


fn apply_gravity(moons: &mut [Moon]) {
    for i in 0..moons.len() {
        for j in 0..moons.len() {
            for dim in 0..3 {
                moons[i].vel[dim] += (moons[j].pos[dim] - moons[i].pos[dim]).signum();
            }
        }
    }
}


fn apply_velocity(moons: &mut [Moon]) {
    for moon in moons {
        for dim in 0..3 {
            moon.pos[dim] += moon.vel[dim];
        }
    }
}


fn part_one(input: &[Moon]) -> usize {
    let mut moons = input.to_vec();
    for _step in 0..1000 {
        apply_gravity(&mut moons);
        apply_velocity(&mut moons);
    }
    moons.iter().map(|moon| moon.energy()).sum()
}


fn dimension_state(moons: &[Moon], dim: usize) -> [isize; 8] {
    [
        moons[0].pos[dim], moons[0].vel[dim],
        moons[1].pos[dim], moons[1].vel[dim],
        moons[2].pos[dim], moons[2].vel[dim],
        moons[3].pos[dim], moons[3].vel[dim]
    ]
}


fn part_two(input: &[Moon]) -> usize {
    let mut moons = input.to_vec();

    let initial_state = [
        dimension_state(&moons, 0),
        dimension_state(&moons, 1),
        dimension_state(&moons, 2)
    ];

    let mut cycle_len = [0; 3];

    let mut step: usize = 0;
    while cycle_len[0] == 0 || cycle_len[1] == 0 || cycle_len[2] == 0 {
        apply_gravity(&mut moons);
        apply_velocity(&mut moons);
        step += 1;

        for dim in 0..3 {
            if cycle_len[dim] == 0 && dimension_state(&moons, dim) == initial_state[dim] {
                cycle_len[dim] = step;
            }
        }
    }

    lcm(cycle_len[0], lcm(cycle_len[1], cycle_len[2]))
}


fn main() {
    let input = [
        Moon::new([-1, -4, 0]),
        Moon::new([4, 7, -1]),
        Moon::new([-14, -10, 9]),
        Moon::new([1, 2, 17])
    ];

    println!("Part one: {}", part_one(&input));
    println!("Part two: {}", part_two(&input));
}
