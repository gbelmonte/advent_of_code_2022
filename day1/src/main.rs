use std::{fs, collections::BinaryHeap};

fn main() {
    part_1_solution();
    part_2_solution();
}

fn part_1_solution() {
    let input: Vec<String> = fs::read_to_string("day1/input/part1.txt")
        .expect("Something went wrong reading the file")
        .lines()
        .map(String::from)
        .collect();

    let max = input.split(|x| x.is_empty())
        .map(|inventory|
            inventory.into_iter()
                .map(|x| x.parse::<i32>().unwrap())
                .sum::<i32>()
        )
        .max().unwrap();

    println!("max elf inventory {:?}", max);
}

fn part_2_solution() {
    let input: Vec<String> = fs::read_to_string("day1/input/part1.txt")
        .expect("Something went wrong reading the file")
        .lines()
        .map(String::from)
        .collect();

    let mut max_heap = input.split(|x| x.is_empty())
        .fold(BinaryHeap::new(), |mut max_heap, inventory| {
            let size = inventory.into_iter()
                .map(|x| x.parse::<i32>().unwrap())
                .sum::<i32>();

            max_heap.push(size);
            max_heap
        });

    println!("max 3 elves inventory {}", max_heap.pop().unwrap() + max_heap.pop().unwrap() + max_heap.pop().unwrap());
}
