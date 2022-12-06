use std::time::Instant;

fn unique(chrs: &[u8]) -> bool {
    let mut seen: u32 = 0;

    for c in chrs {
        let prev = seen;
        seen |= 1 << (c - b'a');

        if seen == prev {
            return false;
        }
    }

    true
}

fn solve(chrs: &[u8], window_size: usize) -> usize {
    for (idx, window) in chrs.windows(window_size).enumerate() {
        let is_unique = unique(window);
        if is_unique {
            return idx + window_size;
        }
    }

    unreachable!();
}

fn main() {
    let input = include_str!("input6.txt").as_bytes();

    let it = Instant::now();
    let part1 = solve(input, 4);
    println!("{part1} {:?}", it.elapsed());

    let it = Instant::now();
    let part2 = solve(input, 14);
    println!("{part2} {:?}", it.elapsed());
}
