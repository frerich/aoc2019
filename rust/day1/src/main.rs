fn fuel_required(mass: u128) -> u128 {
    let quot = mass / 3;
    if quot > 2 {
        quot - 2
    } else {
        0
    }
}

fn absolute_fuel_required(mass: u128) -> u128 {
    if mass == 0 {
        0
    } else {
        let fuel = fuel_required(mass);
        fuel + absolute_fuel_required(fuel)
    }
}


fn main() {
    let input = std::fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");

    let masses = input.lines().map(|line| line.parse::<u128>().unwrap());

    let part_one: u128 = masses.clone().map(fuel_required).sum();
    let part_two: u128 = masses.map(absolute_fuel_required).sum();

    println!("Part One: {:?}", part_one);
    println!("Part Two: {:?}", part_two);
}


#[cfg(test)]
mod tests {
    mod fuel_required {
        use super::super::fuel_required;

        #[test]
        fn example_1() {
            assert_eq!(fuel_required(12), 2);
        }

        #[test]
        fn example_2() {
            assert_eq!(fuel_required(14), 2);
        }

        #[test]
        fn example_3() {
            assert_eq!(fuel_required(1969), 654);
        }

        #[test]
        fn example_4() {
            assert_eq!(fuel_required(100756), 33583);
        }

        #[test]
        fn small_mass() {
            assert_eq!(fuel_required(2), 0);
        }
    }

    mod absolute_fuel_required {
        use super::super::absolute_fuel_required;

        #[test]
        fn example_1() {
            assert_eq!(absolute_fuel_required(14), 2);
        }

        #[test]
        fn example_2() {
            assert_eq!(absolute_fuel_required(1969), 966);
        }

        #[test]
        fn example_3() {
            assert_eq!(absolute_fuel_required(100756), 50346);
        }
    }
}
