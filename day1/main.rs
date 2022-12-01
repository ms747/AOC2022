const INPUT: &str = include_str!("./input.txt");

fn main() {
    let mut data: Vec<u32> = INPUT
        .split("\n\n")
        .map(|x| x.trim().split('\n'))
        .map(|x| x.fold(0, |acc, x| acc + x.parse::<u32>().unwrap()))
        .collect();

    // Sort
    data.sort_by(|a, b| b.cmp(a));

    // Part 1
    let part1: u32 = data.iter().take(1).sum();

    // Part 2
    let part2: u32 = data.iter().take(3).sum();

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}
