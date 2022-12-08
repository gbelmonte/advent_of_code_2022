use std::{fs, collections::HashSet};

fn main() {
    part_1_solution();
    part_2_solution();
}

fn part_1_solution() {
    let input: String = fs::read_to_string("day6/input/part1.txt")
        .expect("Something went wrong reading the file");

    let chars: Vec<char> = input.chars().filter(|x| *x != '\n').collect();
    let size = 4;
    let distincts: Vec<(usize, HashSet<&char>)> = chars.windows(size)
        .enumerate()
        .map(|(i, w)| {
            let set = w.iter()
                .fold(HashSet::new(), |mut acc, c| {
                    acc.insert(c);
                    acc
                });
            (i, set)
        })
        .filter(|(_, set)| set.len() == size)
        .collect();
    println!("start of packet {:?}", distincts.first().unwrap().0 + size);
}

fn part_2_solution() {
    let input: String = fs::read_to_string("day6/input/part1.txt")
        .expect("Something went wrong reading the file");

    let chars: Vec<char> = input.chars().filter(|x| *x != '\n').collect();
    let size = 14;
    let distincts: Vec<(usize, HashSet<&char>)> = chars.windows(size)
        .enumerate()
        .map(|(i, w)| {
            let set = w.iter()
                .fold(HashSet::new(), |mut acc, c| {
                    acc.insert(c);
                    acc
                });
            (i, set)
        })
        .filter(|(_, set)| set.len() == size)
        .collect();
    println!("start of message {:?}", distincts.first().unwrap().0 + size);
}