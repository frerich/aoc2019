fn refills(mass: &usize) -> impl Iterator<Item = usize> {
    std::iter::successors(Some(*mass), |mass| {
        (mass / 3).checked_sub(2)
    }).skip(1)
}


fn initial_fuel(mass: &usize) -> usize {
    refills(mass).next().unwrap()
}


fn total_fuel(mass: &usize) -> usize {
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

    let part_one: usize = masses.iter().map(inital_fuel).sum();
    let part_two: usize = masses.iter().map(total_fuel).sum();

    println!("Part One: {:?}", part_one);
    println!("Part Two: {:?}", part_two);
}


#[cfg(test)]
mod tests {
    mod inital_fuel {
        use super::super::inital_fuel;

        #[test]
        fn example_1() {
            assert_eq!(inital_fuel(12), 2);
        }

        #[test]
        fn example_2() {
            assert_eq!(inital_fuel(14), 2);
        }

        #[test]
        fn example_3() {
            assert_eq!(inital_fuel(1969), 654);
        }

        #[test]
        fn example_4() {
            assert_eq!(inital_fuel(100756), 33583);
        }

        #[test]
        fn small_mass() {
            assert_eq!(inital_fuel(2), 0);
        }
    }

    mod total_fuel {
        use super::super::total_fuel;

        #[test]
        fn example_1() {
            assert_eq!(total_fuel(14), 2);
        }

        #[test]
        fn example_2() {
            assert_eq!(total_fuel(1969), 966);
        }

        #[test]
        fn example_3() {
            assert_eq!(total_fuel(100756), 50346);
        }
    }
}
