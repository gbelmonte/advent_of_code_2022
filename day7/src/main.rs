use std::{fs, collections::{HashMap, HashSet}};

fn main() {
    part_1_solution();
}

fn part_1_solution() {
    let input = fs::read_to_string("day7/input/part1.txt")
        .expect("Something went wrong reading the file");

    let mut edges: HashMap<String, HashSet<String>> = HashMap::new();
    let mut wieghts: HashMap<String, i32> = HashMap::new();
    let mut stack: Vec<String> = Vec::new();
    let mut curr_dir = String::new();
    for cmd in input.lines() {
        if cmd.starts_with("$ cd") {
            let val = cmd.split(" ").nth(2).unwrap();
            if val == ".." { stack.pop(); } else { stack.push(val.to_owned()); }
        } else if cmd.starts_with("$ ls") {
            curr_dir = stack.join("/");
            edges.insert(curr_dir.to_string(), HashSet::new());
            wieghts.insert(curr_dir.to_string(), 0);
        } else {
            if cmd.starts_with("dir") {
                let e = edges.entry(curr_dir.to_string());
                e.or_default().insert(cmd.split(" ").nth(1).unwrap().to_string());
            } else {
                let size = cmd.split(" ").nth(0).unwrap().parse::<i32>().unwrap();
                let e = wieghts.entry(curr_dir.to_string());
                e.and_modify(|x| *x += size).or_insert(size);
            }
        }
    }

    println!("{:?}", wieghts);
}