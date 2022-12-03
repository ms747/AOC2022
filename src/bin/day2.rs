const INPUT: &str = include_str!("input2.txt");

#[derive(Debug, Eq, PartialEq, Clone)]
enum Shape {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

enum Outcome {
    Won = 6,
    Draw = 3,
    Lost = 0,
}

macro_rules! add_scores {
    ($x: ident, $y: ident) => {
        $x as usize + $y as usize
    };
}

fn calculate_score(elf: Shape, my_shape: Shape) -> usize {
    use Outcome::*;
    use Shape::*;

    match (elf, my_shape) {
        (Rock, Paper) => add_scores!(Won, Paper),
        (Rock, Scissors) => add_scores!(Lost, Scissors),
        (Paper, Scissors) => add_scores!(Won, Scissors),
        (Paper, Rock) => add_scores!(Lost, Rock),
        (Scissors, Rock) => add_scores!(Won, Rock),
        (Scissors, Paper) => add_scores!(Lost, Paper),
        (x, y) if x == y => add_scores!(Draw, y),
        (_, _) => panic!("Unknown combination"),
    }
}

fn calculate_score_based_on_outcome(elf: Shape, outcome: Outcome) -> usize {
    use Outcome::*;
    use Shape::*;

    match (elf, outcome) {
        (Rock, Won) => add_scores!(Paper, Won),
        (Rock, Lost) => add_scores!(Scissors, Lost),
        (Paper, Won) => add_scores!(Scissors, Won),
        (Paper, Lost) => add_scores!(Rock, Lost),
        (Scissors, Won) => add_scores!(Rock, Won),
        (Scissors, Lost) => add_scores!(Paper, Lost),
        (x, Draw) => add_scores!(x, Draw),
    }
}

fn main() {
    use Outcome::*;
    use Shape::*;

    let part1: usize = INPUT
        .lines()
        .map(|line| {
            let (elf_shape, my_shape) = line.split_once(' ').unwrap();

            let elf_shape = match elf_shape {
                "A" => Rock,
                "B" => Paper,
                "C" => Scissors,
                _ => panic!("Unknown shape"),
            };

            let my_shape = match my_shape {
                "X" => Rock,
                "Y" => Paper,
                "Z" => Scissors,
                _ => panic!("Unknown shape"),
            };

            calculate_score(elf_shape, my_shape)
        })
        .sum();

    println!("{part1}");

    let part2: usize = INPUT
        .lines()
        .map(|line| {
            let (elf_shape, outcome) = line.split_once(' ').unwrap();

            let elf_shape = match elf_shape {
                "A" => Rock,
                "B" => Paper,
                "C" => Scissors,
                _ => panic!("Unknown shape"),
            };

            let outcome = match outcome {
                "X" => Lost,
                "Y" => Draw,
                "Z" => Won,
                _ => panic!("Unknown outcome"),
            };

            calculate_score_based_on_outcome(elf_shape, outcome)
        })
        .sum();

    println!("{part2}");
}
