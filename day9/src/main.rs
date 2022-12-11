use std::{fs, collections::HashSet};

fn main() {
    part_1_solution();
    part_2_solution();
}

fn part_1_solution() {
    let cmds: Vec<(String, i32)> = fs::read_to_string("day9/input/part1.txt")
        .expect("Something went wrong reading the file")
        .lines()
        .map(|line| {
            let mut vals = line.split(" ");
            (vals.next().unwrap().to_string(), vals.next().unwrap().parse::<i32>().unwrap())
        })
        .collect();

    let mut head: (i32, i32) = (0, 0);
    let mut tail: (i32, i32) = (0, 0);
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    visited.insert((0, 0));
    for cmd in cmds {
        let offset = match cmd.0.as_str() {
            "R" => (1, 0),
            "L" => (-1, 0),
            "U" => (0, 1),
            "D" => (0, -1),
            _ => (0, 0),
        };
        for _ in 0..cmd.1 {
            head = (head.0 + offset.0, head.1 + offset.1);
            let toff = (head.0 - tail.0, head.1 - tail.1);
            if toff.0.abs() == 2 || toff.1.abs() == 2 {
                tail = (tail.0 + toff.0.clamp(-1, 1), tail.1 + toff.1.clamp(-1, 1));
                visited.insert(tail);
            }
        }
    }
    println!("visited {} spaces", visited.len());
}

fn part_2_solution() {
    let cmds: Vec<(String, i32)> = fs::read_to_string("day9/input/part1.txt")
        .expect("Something went wrong reading the file")
        .lines()
        .map(|line| {
            let mut vals = line.split(" ");
            (vals.next().unwrap().to_string(), vals.next().unwrap().parse::<i32>().unwrap())
        })
        .collect();

    let mut knots: Vec<(i32, i32)> = vec![(0,0); 10];
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    visited.insert((0, 0));
    for cmd in cmds {
        let offset = match cmd.0.as_str() {
            "R" => (1, 0),
            "L" => (-1, 0),
            "U" => (0, 1),
            "D" => (0, -1),
            _ => (0, 0),
        };
        for _ in 0..cmd.1 {
            knots[0] = (knots[0].0 + offset.0, knots[0].1 + offset.1);
            for i in 1..knots.len() {
                let toff = (knots[i-1].0 - knots[i].0, knots[i-1].1 - knots[i].1);
                if toff.0.abs() == 2 || toff.1.abs() == 2 {
                    knots[i] = (knots[i].0 + toff.0.clamp(-1, 1), knots[i].1 + toff.1.clamp(-1, 1));
                }
            }
            visited.insert(*knots.last().unwrap());
        }
    }
    println!("visited {} spaces", visited.len());
}