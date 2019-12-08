fn count<T: PartialEq>(v: &[T], x: T) -> usize {
    v.iter().filter(|&n| *n == x).count()
}


fn parse(input: &str) -> Option<Vec<u8>> {
    input.chars().map(|ch| {
        ch.to_digit(10).map(|n| n as u8)
    }).collect()
}


fn merge(layers: &Vec<&[u8]>) -> Vec<u8> {
    let mut iters: Vec<_> = layers.iter().map(|layer| layer.iter()).collect();

    let mut result = Vec::new();
    loop {
        let mut pixel = None;
        for it in iters.iter_mut() {
            match it.next() {
                Some(digit) => if *digit != 2 && pixel == None { pixel = Some(*digit) }
                None => return result
            }
        }
        result.push(pixel.unwrap());
    }
}


fn part_one(layers: &Vec<&[u8]>) -> usize {
    let least_zeros_layer = layers.iter().min_by_key(|layer| count(layer, 0)).unwrap();
    count(least_zeros_layer, 1) * count(least_zeros_layer, 2)
}


fn part_two(layers: &Vec<&[u8]>) -> String {
    let merged = merge(&layers);

    let mut lines: Vec<String> = Vec::new();
    for chunk in merged.chunks(25) {
        lines.push(chunk.iter().map(|n| if *n == 0 {"  "} else {"##"}).collect());
    }

    lines.join("\n")
}


fn main() {
    let width = 25;
    let height = 6;
    let input = std::fs::read_to_string("input.txt")
        .expect("Failed to read input file");

    let pixels = parse(input.trim_end())
        .expect("Failed to parse input file");

    let layers: Vec<_> = pixels.chunks(width * height).collect();

    println!("Part one: {:?}", part_one(&layers));
    println!("Part two:\n{}", part_two(&layers));
}
