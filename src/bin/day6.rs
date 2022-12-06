use std::time::Instant;

fn unique(chrs: &[u8], window_size: usize) -> bool {
    for x in 0..window_size {
        for y in 0..window_size {
            if x != y && chrs[x] == chrs[y] {
                return false;
            }
        }
    }

    true
}

fn solve(chrs: &[u8], window_size: usize) -> usize {
    for (idx, window) in chrs.windows(window_size).enumerate() {
        let is_unique = unique(window, window_size);
        if is_unique {
            return idx + window_size;
        }
    }

    unreachable!();
}

fn main() {
    let input = include_str!("input6.txt");
    let input: Vec<u8> = input.bytes().collect();

    let it = Instant::now();
    let part1 = solve(&input, 4);
    println!("{part1} {:?}", it.elapsed());

    let it = Instant::now();
    let part2 = solve(&input, 14);
    println!("{part2} {:?}", it.elapsed());
}
