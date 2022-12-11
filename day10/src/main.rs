use std::fs;

fn main() {
    part_1_solution();
    part_2_solution();
}

fn part_1_solution() {
    let cmds: Vec<i32> = fs::read_to_string("day10/input/part1.txt")
        .expect("Something went wrong reading the file")
        .lines()
        .flat_map(|line| {
            if line.starts_with("noop") {
                vec![0]
            } else {
                vec![0, line.split(" ").nth(1).unwrap().parse::<i32>().unwrap()]
            }
        })
        .collect();

    let mut reg: i32 = 1;
    let mut signal = 0;
    for (i, x) in cmds.iter().enumerate().take(220) {
        let cycle = i + 1;
        if (cycle + 20) % 40 == 0 {
            signal += reg * cycle as i32;
        }
        reg += x;
    }
    println!("Sum of signal {}", signal);
}

fn part_2_solution() {
    let cmds: Vec<i32> = fs::read_to_string("day10/input/part1.txt")
        .expect("Something went wrong reading the file")
        .lines()
        .flat_map(|line| {
            if line.starts_with("noop") {
                vec![0]
            } else {
                vec![0, line.split(" ").nth(1).unwrap().parse::<i32>().unwrap()]
            }
        })
        .collect();

    let mut reg: i32 = 1;
    for (i, x) in cmds.iter().enumerate().take(240) {
        let pixel = i as i32 % 40;
        if pixel == 0 {
            println!();
        }

        if pixel >= (reg - 1) && pixel <= (reg + 1) {
            print!("#");
        } else {
            print!(".");
        }
        reg += x;
    }
    println!();
}