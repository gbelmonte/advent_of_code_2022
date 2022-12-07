use std::{fs, collections::{HashSet, HashMap}};

fn main() {
    part_1_solution();
    part_2_solution();
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

fn part_2_solution() {
    let distinct: Vec<HashSet<char>> = fs::read_to_string("day3/input/part1.txt")
        .expect("Something went wrong reading the file")
        .lines()
        .map(|line| {
            line.chars()
                .fold(HashSet::new(), |mut acc, c| {
                    acc.insert(c);
                    acc
                })
        })
        .collect();
    
    let total: i32 = distinct.chunks(3)
        .map(|group| {
            let c = group[0].iter()
                .chain(group[1].iter())
                .chain(group[2].iter())
                .fold(HashMap::new(), |mut acc, c| {
                    *acc.entry(*c).or_insert(0) += 1;
                    acc
                })
                .drain()
                .filter(|(_, count)| *count == 3)
                .map(|(c, _)| c)
                .last()
                .unwrap();
            if c.is_uppercase() {c as i32 - 64 + 26} else {c as i32 - 96}
        })
        .sum();
        
    println!("Total errored priority part 2 {:?}", total)
}