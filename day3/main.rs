#![feature(iter_array_chunks)]

const INPUT: &str = include_str!("./input.txt");

fn map_priority(ch: u8) -> usize {
    if ch >= b'a' {
        ch as usize - 96
    } else {
        ch as usize - 38
    }
}

fn main() {
    let it = std::time::Instant::now();
    let part1: usize = INPUT
        .lines()
        .map(|line| {
            let (part1, part2) = line.split_at(line.len() / 2);
            let mut occurences = [false; 53];

            for ch1 in part1.bytes() {
                let priority = map_priority(ch1);
                occurences[priority] = true;
            }

            for ch2 in part2.bytes() {
                let priority = map_priority(ch2);
                if occurences[priority] {
                    return priority;
                }
            }

            unreachable!();
        })
        .sum();

    println!("Part 1: {part1} took {:?}", it.elapsed());

    let it = std::time::Instant::now();
    let part2: usize = INPUT
        .lines()
        .array_chunks::<3>()
        .map(|chunk| {
            let mut occurences: [bool; 53] = [false; 53];
            let mut occurences2: [bool; 53] = [false; 53];

            for ch1 in chunk[0].bytes() {
                let priority = map_priority(ch1);
                occurences[priority] = true;
            }

            for ch2 in chunk[1].bytes() {
                let priority = map_priority(ch2);
                if occurences[priority] {
                    occurences2[priority] = true;
                }
            }

            for ch3 in chunk[2].bytes() {
                let priority = map_priority(ch3);
                if occurences2[priority] {
                    return priority;
                }
            }

            unreachable!();
        })
        .sum();

    println!("Part 2: {part2} took {:?}", it.elapsed());
}
