use std::fs;

fn main() {
    part_1_solution();
    part_2_solution();
}

enum RPS {
    ROCK = 1,
    PAPER = 2,
    SCISSOR = 3,
    LOSE,
    DRAW,
    WIN,
}

fn part_1_solution() {
    let total: i32 = fs::read_to_string("day2/input/part1.txt")
        .expect("Something went wrong reading the file")
        .lines()
        .map(|line| {
            line.split(' ')
                .map(|choice| {
                    match choice {
                        "A" => RPS::ROCK,
                        "B" => RPS::PAPER,
                        "C" => RPS::SCISSOR,
                        "X" => RPS::ROCK,
                        "Y" => RPS::PAPER,
                        "Z" => RPS::SCISSOR,
                        _ => RPS::PAPER,
                    }
                })
                .collect::<Vec<RPS>>()
        })
        .map(|round| {
            match round[0..2] {
                [RPS::ROCK, RPS::ROCK] => RPS::ROCK as i32 + 3,
                [RPS::ROCK, RPS::PAPER] => RPS::PAPER as i32 + 6,
                [RPS::ROCK, RPS::SCISSOR] => RPS::SCISSOR as i32 + 0,
                [RPS::PAPER, RPS::ROCK] => RPS::ROCK as i32 + 0,
                [RPS::PAPER, RPS::PAPER] => RPS::PAPER as i32 + 3,
                [RPS::PAPER, RPS::SCISSOR] => RPS::SCISSOR as i32 + 6,
                [RPS::SCISSOR, RPS::ROCK] => RPS::ROCK as i32 + 6,
                [RPS::SCISSOR, RPS::PAPER] => RPS::PAPER as i32 + 0,
                [RPS::SCISSOR, RPS::SCISSOR] => RPS::SCISSOR as i32 + 3,
                _ => 0
            }
        })
        .sum();

    println!("Total round score {:?}", total);
}

fn part_2_solution() {
    let total: i32 = fs::read_to_string("day2/input/part1.txt")
        .expect("Something went wrong reading the file")
        .lines()
        .map(|line| {
            line.split(' ')
                .map(|choice| {
                    match choice {
                        "A" => RPS::ROCK,
                        "B" => RPS::PAPER,
                        "C" => RPS::SCISSOR,
                        "X" => RPS::LOSE,
                        "Y" => RPS::DRAW,
                        "Z" => RPS::WIN,
                        _ => RPS::PAPER,
                    }
                })
                .collect::<Vec<RPS>>()
        })
        .map(|round| {
            match round[0..2] {
                [RPS::ROCK, RPS::LOSE] => RPS::SCISSOR as i32 + 0,
                [RPS::ROCK, RPS::DRAW] => RPS::ROCK as i32 + 3,
                [RPS::ROCK, RPS::WIN] => RPS::PAPER as i32 + 6,
                [RPS::PAPER, RPS::LOSE] => RPS::ROCK as i32 + 0,
                [RPS::PAPER, RPS::DRAW] => RPS::PAPER as i32 + 3,
                [RPS::PAPER, RPS::WIN] => RPS::SCISSOR as i32 + 6,
                [RPS::SCISSOR, RPS::LOSE] => RPS::PAPER as i32 + 0,
                [RPS::SCISSOR, RPS::DRAW] => RPS::SCISSOR as i32 + 3,
                [RPS::SCISSOR, RPS::WIN] => RPS::ROCK as i32 + 6,
                _ => 0
            }
        })
        .sum();

    println!("Total round score {:?}", total);
}
