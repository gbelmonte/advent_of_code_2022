use std::{fs, collections::HashSet};

fn main() {
    part_1_solution();
}

fn part_1_solution() {
    let total: i32 = fs::read_to_string("day3/input/part1.txt")
        .expect("Something went wrong reading the file")
        .lines()
        .map(|line| {
            let items: Vec<char> = line.chars().collect();
            let (left, right) = items.split_at(items.len() / 2);
            let distinct = left.iter()
                .fold(HashSet::new(), |mut acc, c| {
                    acc.insert(c);
                    acc
                });
            right.iter()
                .filter(|&x| distinct.contains(x))
                .map(|&x| if x.is_uppercase() {x as i32 - 64 + 26} else {x as i32 - 96})
                .last()
                .unwrap()
        })
        .sum();

    println!("Total errored priority {}", total)
}
