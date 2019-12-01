fn refills(mass: &usize) -> impl Iterator<Item = usize> {
    std::iter::successors(Some(*mass), |mass| {
        (mass / 3).checked_sub(2)
    }).skip(1)
}


fn fuel_required(mass: &usize) -> usize {
    refills(mass).next().unwrap()
}


fn absolute_fuel_required(mass: &usize) -> usize {
    refills(mass).sum()
}


fn parse(input: &str) -> Option<Vec<usize>> {
    input.lines().map(|s| s.parse().ok()).collect()
}


fn main() {
    let input = std::fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");

    let masses = parse(&input)
        .expect("Failed to parse masses from input file");

    let part_one: usize = masses.iter().map(fuel_required).sum();
    let part_two: usize = masses.iter().map(absolute_fuel_required).sum();

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
