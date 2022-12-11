use std::{fs, collections::{HashMap, HashSet}};

fn main() {
    part_1_solution();
    part_2_solution();
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
                e.or_default().insert(curr_dir.to_string() + "/" + cmd.split(" ").nth(1).unwrap());
            } else {
                let size = cmd.split(" ").nth(0).unwrap().parse::<i32>().unwrap();
                let e = wieghts.entry(curr_dir.to_string());
                e.and_modify(|x| *x += size).or_insert(size);
            }
        }
    }

    let mut out: HashMap<String, i32> = HashMap::new();
    dfs("/".to_string(), &wieghts, &edges, &mut out);
    let sum: i32 = out.iter()
        .map(|(_, &size)| size)
        .filter(|&size| size <= 100000)
        .sum();

    println!("sum of dirs below size limit {}", sum);
}

fn part_2_solution() {
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
                e.or_default().insert(curr_dir.to_string() + "/" + cmd.split(" ").nth(1).unwrap());
            } else {
                let size = cmd.split(" ").nth(0).unwrap().parse::<i32>().unwrap();
                let e = wieghts.entry(curr_dir.to_string());
                e.and_modify(|x| *x += size).or_insert(size);
            }
        }
    }

    let mut out: HashMap<String, i32> = HashMap::new();
    dfs("/".to_string(), &wieghts, &edges, &mut out);
    let space_needed = 30000000 - (70000000 - out.get("/").unwrap());
    let size: i32 = out.iter()
        .map(|(_, &size)| size)
        .filter(|&size| size >= space_needed)
        .min().unwrap();

    println!("deleted dir size {}", size);
}

fn dfs(curr: String,
    weights: &HashMap<String, i32>, 
    edges: &HashMap<String, HashSet<String>>, 
    out: &mut HashMap<String, i32>) -> i32 {
    let mut total_size = weights.get(&curr).unwrap().to_owned();
    edges.get(&curr).unwrap().iter().for_each(|x| {
        total_size += dfs(x.to_string(), &weights, &edges, out);
    });
    out.insert(curr, total_size);
    return total_size;
}