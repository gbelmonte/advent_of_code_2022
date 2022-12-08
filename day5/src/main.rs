use std::fs;

fn main() {
    part_1_solution();
    part_2_solution();
}

fn part_1_solution() {
    let input: String = fs::read_to_string("day5/input/part1.txt")
        .expect("Something went wrong reading the file");

    let mut stacks: Vec<Vec<char>> = input.lines()
        .take(8)
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .fold(vec![Vec::new(); 9], |mut acc, line| {
            let items: Vec<char> = line.chars().collect();
            items.chunks(4)
                .enumerate()
                .filter(|(_, val)| val[0] != ' ')
                .for_each(|(i, val)| {
                    acc[i].push(val[1]);
                });
            acc
        });

    input.lines()
        .skip(10)
        .for_each(|line| {
            let split: Vec<&str> = line.split(" ").collect();
            let num = split[1].parse::<i32>().unwrap();
            let from = split[3].parse::<usize>().unwrap() - 1;
            let to = split[5].parse::<usize>().unwrap() - 1;

            for _ in 0..num {
                let val = stacks[from].pop().unwrap();
                stacks[to].push(val);
            }
        });
    
    print!("Crates at the top: ");
    for mut stack in stacks {
        if let Some(x) = stack.pop() {
            print!("{}", x)
        }        
    }
    println!();
}

fn part_2_solution() {
    let input: String = fs::read_to_string("day5/input/part1.txt")
        .expect("Something went wrong reading the file");

    let mut stacks: Vec<Vec<char>> = input.lines()
        .take(8)
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .fold(vec![Vec::new(); 9], |mut acc, line| {
            let items: Vec<char> = line.chars().collect();
            items.chunks(4)
                .enumerate()
                .filter(|(_, val)| val[0] != ' ')
                .for_each(|(i, val)| {
                    acc[i].push(val[1]);
                });
            acc
        });

    input.lines()
        .skip(10)
        .for_each(|line| {
            let split: Vec<&str> = line.split(" ").collect();
            let num = split[1].parse::<i32>().unwrap();
            let from = split[3].parse::<usize>().unwrap() - 1;
            let to = split[5].parse::<usize>().unwrap() - 1;

            let mut temp = Vec::new();
            for _ in 0..num {
                temp.push(stacks[from].pop().unwrap());
            }
            for _ in 0..num {
                stacks[to].push(temp.pop().unwrap())
            }
        });
    
    print!("Crates at the top: ");
    for mut stack in stacks {
        if let Some(x) = stack.pop() {
            print!("{}", x)
        }        
    }
    println!();
}