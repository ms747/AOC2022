use std::time::Instant;

const INPUT: &str = include_str!("input4.txt");

fn parse(line: &str) -> (i32, i32, i32, i32) {
    let Some((range1, range2)) = line.split_once(',') else {
        unreachable!();
    };

    let (Some((x1, y1)), Some((x2, y2))) = (range1.split_once('-'), range2.split_once('-')) else {
        unreachable!();
    };

    let (Ok(x1), Ok(y1), Ok(x2), Ok(y2)) = (x1.parse(), y1.parse(), x2.parse(), y2.parse()) else {
        unreachable!();
    };

    (x1, y1, x2, y2)
}

fn main() {
    let it = Instant::now();
    let input: Vec<_> = INPUT.trim().lines().map(parse).collect();
    println!("Parsing took: {:?}", it.elapsed());

    let it = Instant::now();
    let part1 = input
        .iter()
        .filter(|(x1, y1, x2, y2)| {
            let range1 = x1 >= x2 && y1 <= y2;
            let range2 = x2 >= x1 && y2 <= y1;
            range1 || range2
        })
        .count();
    println!("Part 1: {part1} took {:?}", it.elapsed());

    let it = Instant::now();
    let part2 = input
        .iter()
        .filter(|(x1, y1, x2, y2)| !((y1 < x2) || (y2 < x1)))
        .count();
    println!("Part 2: {part2} took {:?}", it.elapsed());
}
