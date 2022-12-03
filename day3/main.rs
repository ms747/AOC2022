use std::collections::HashSet;
use std::iter::FromIterator;

const fn create_char_array() -> [char; 52] {
    let mut chars: [char; 52] = ['0'; 52];
    let mut i = 0;

    while i < 26 {
        chars[i] = (b'a' + i as u8) as char;
        i += 1;
    }

    i = 0;
    while i < 26 {
        chars[i + 26] = (b'A' + i as u8) as char;
        i += 1;
    }
    chars
}

const INPUT: &str = include_str!("./input.txt");
const CHARS: [char; 52] = create_char_array();

fn main() {
    let inputs: Vec<&str> = INPUT.lines().collect();

    let it = std::time::Instant::now();
    let part1: usize = inputs
        .iter()
        .map(|line| {
            let pivot = line.len() / 2;
            let part1: HashSet<u8> = HashSet::from_iter(&mut line[..pivot].bytes());
            let part2: HashSet<u8> = HashSet::from_iter(&mut line[pivot..].bytes());
            let common = *part1.intersection(&part2).next().unwrap();
            CHARS.iter().position(|c| c == &(common as char)).unwrap() + 1
        })
        .sum();
    println!("{part1:} {:?}", it.elapsed());

    let it = std::time::Instant::now();
    let part2: usize = inputs
        .chunks(3)
        .map(|chunk| {
            let line1: HashSet<u8> = HashSet::from_iter(&mut chunk[0].bytes());
            let line2: HashSet<u8> = HashSet::from_iter(&mut chunk[1].bytes());
            let line3: HashSet<u8> = HashSet::from_iter(&mut chunk[2].bytes());

            for ch in line1.intersection(&line2) {
                if line3.contains(ch) {
                    let position = CHARS.iter().position(|c| c == &(*ch as char)).unwrap() + 1;
                    return position;
                }
            }

            unreachable!();
        })
        .sum();

    println!("{part2:} {:?}", it.elapsed());
}
