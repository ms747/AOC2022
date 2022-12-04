use std::{cmp::Ordering::*, time::Instant};

const INPUT: &str = include_str!("input4.txt");

fn parse(line: &str) -> (i32, i32, i32, i32) {
    let Some((range1, range2)) = line.split_once(',') else {
        unreachable!();
    };

    let (Some((a, b)), Some((x, y))) = (range1.split_once('-'), range2.split_once('-')) else {
        unreachable!();
    };

    let (Ok(a), Ok(b), Ok(x), Ok(y)) = (a.parse(), b.parse(), x.parse(), y.parse()) else {
        unreachable!();
    };

    (a, b, x, y)
}

fn main() {
    let it = Instant::now();
    let input: Vec<_> = INPUT.trim().lines().map(parse).collect();
    println!("Parsing took: {:?}", it.elapsed());

    let it = Instant::now();
    let part1 = input
        .iter()
        .filter(|(a, b, x, y)| match (b - a).cmp(&(y - x)) {
            Greater => (x - a) >= 0 && (b - y) >= 0,
            Less => (a - x) >= 0 && (y - b) >= 0,
            Equal => a == x && b == y,
        })
        .count();
    println!("Part 1: {part1} took {:?}", it.elapsed());

    let it = Instant::now();
    let part2 = input
        .iter()
        .filter(|(a, b, x, y)| {
            let range_xy_contains_a = a >= x && a <= y;
            let range_xy_contains_b = b >= x && b <= y;
            let range_ab_contains_x = x >= a && y <= b;
            let range_ab_contains_y = y >= a && y <= b;
            range_xy_contains_a || range_xy_contains_b || range_ab_contains_x || range_ab_contains_y
        })
        .count();
    println!("Part 2: {part2} took {:?}", it.elapsed());
}
