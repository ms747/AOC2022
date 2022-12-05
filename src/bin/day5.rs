use std::time::Instant;

const INPUT: &str = include_str!("input5.txt");

fn parse(line: &str) -> (usize, usize, usize) {
    let (Some(count), Some(source), Some(destination)) = (line.split(' ').nth(1), line.split(' ').nth(3), line.split(' ').nth(5)) else {
        unreachable!();
    };

    let (Ok(count), Ok(source), Ok(destination)) = (count.parse::<usize>(), source.parse::<usize>(), destination.parse::<usize>()) else {
        unreachable!();
    };

    (count, source - 1, destination - 1)
}

fn main() {
    let mut crates: Vec<_> = vec![
        vec!['R', 'N', 'F', 'V', 'L', 'J', 'S', 'M'],
        vec!['P', 'N', 'D', 'Z', 'F', 'J', 'W', 'H'],
        vec!['W', 'R', 'C', 'D', 'G'],
        vec!['N', 'B', 'S'],
        vec!['M', 'Z', 'W', 'P', 'C', 'B', 'F', 'N'],
        vec!['P', 'R', 'M', 'W'],
        vec!['R', 'T', 'N', 'G', 'L', 'S', 'W'],
        vec!['Q', 'T', 'H', 'F', 'N', 'B', 'V'],
        vec!['L', 'M', 'H', 'Z', 'N', 'F'],
    ];

    let mut crates_2 = crates.clone();

    let input: Vec<_> = INPUT.trim().lines().map(parse).collect();

    let it = Instant::now();
    for (count, source, destination) in input.iter() {
        for _ in 0..*count {
            let ch = crates[*source].pop().unwrap();
            crates[*destination].push(ch);
        }
    }

    let mut part1 = String::new();

    for crat in crates {
        part1.push(*crat.last().unwrap());
    }

    println!("{part1} {:?}", it.elapsed());

    let it = Instant::now();
    for (count, source, destination) in input {
        let index = crates_2[destination].len();
        for _ in 0..count {
            let item = crates_2[source].pop().unwrap();
            crates_2[destination].insert(index, item);
        }
    }

    let mut part2 = String::new();

    for crat in crates_2 {
        part2.push(*crat.last().unwrap());
    }

    println!("{part2} {:?}", it.elapsed());
}
