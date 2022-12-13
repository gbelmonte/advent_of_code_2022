use std::{fs, collections::{VecDeque, HashSet}};

fn main() {
    part_1_solution();
    part_2_solution();
}

fn part_1_solution() {
    let input: String = fs::read_to_string("day12/input/part1.txt")
        .expect("Something went wrong reading the file");

    let grid: Vec<Vec<i32>> = input.lines()
        .map(|line| line.chars()
            .map(|c| {
                if c == 'S' {
                    'a' as i32 - 97
                } else if c == 'E' {
                    'z' as i32 - 97
                } else {
                    c as i32 - 97
                }
            })
            .collect()
        )
        .collect();
    
    let start = input.chars().filter(|x| !x.is_whitespace()).position(|x| x == 'S').unwrap() as i32;
    let end = input.chars().filter(|x| !x.is_whitespace()).position(|x| x == 'E').unwrap() as i32;
    let start_cords = (start % grid[0].len() as i32, start / grid[0].len() as i32);
    let end_cords = (end % grid[0].len() as i32, end / grid[0].len() as i32);

    let mut q: VecDeque<((i32, i32), i32)> = VecDeque::new();
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    q.push_back((start_cords, 0));
    while !q.is_empty() {
        let (curr, distance) = q.pop_front().unwrap();
        if visited.contains(&curr) {
            continue;
        } else if curr.0 == end_cords.0 && curr.1 == end_cords.1 {
            println!("it took {} steps", distance);
            break;
        }
        visited.insert(curr);
        for offset in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let coord = (curr.0 + offset.0, curr.1 + offset.1);
            if coord.0 < 0 || coord.0 >= grid[0].len() as i32 || coord.1 < 0 || coord.1 >= grid.len() as i32 {
                continue;
            }
            let val = grid[coord.1 as usize][coord.0 as usize];
            if val <= grid[curr.1 as usize][curr.0 as usize] || val == grid[curr.1 as usize][curr.0 as usize] + 1 {
                q.push_back(((coord.0, coord.1), distance + 1));
            }
        }
    }
}

fn part_2_solution() {
    let input: String = fs::read_to_string("day12/input/part1.txt")
        .expect("Something went wrong reading the file");

    let grid: Vec<Vec<i32>> = input.lines()
        .map(|line| line.chars()
            .map(|c| {
                if c == 'S' {
                    'a' as i32 - 97
                } else if c == 'E' {
                    'z' as i32 - 97
                } else {
                    c as i32 - 97
                }
            })
            .collect()
        )
        .collect();
    
    let starts: Vec<(i32, i32)> = input.chars()
        .filter(|x| !x.is_whitespace())
        .enumerate()
        .filter(|(_, x)| *x == 'a')
        .map(|(i, _)| {
            (i as i32 % grid[0].len() as i32, i as i32 / grid[0].len() as i32)
        })
        .collect();
    let end = input.chars().filter(|x| !x.is_whitespace()).position(|x| x == 'E').unwrap() as i32;
    let end_cords = (end % grid[0].len() as i32, end / grid[0].len() as i32);

    let mut q: VecDeque<((i32, i32), i32)> = VecDeque::new();
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    for start in starts {
        q.push_back((start, 0));
    }
    while !q.is_empty() {
        let (curr, distance) = q.pop_front().unwrap();
        if visited.contains(&curr) {
            continue;
        } else if curr.0 == end_cords.0 && curr.1 == end_cords.1 {
            println!("it took {} steps", distance);
            break;
        }
        visited.insert(curr);
        for offset in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let coord = (curr.0 + offset.0, curr.1 + offset.1);
            if coord.0 < 0 || coord.0 >= grid[0].len() as i32 || coord.1 < 0 || coord.1 >= grid.len() as i32 {
                continue;
            }
            let val = grid[coord.1 as usize][coord.0 as usize];
            if val <= grid[curr.1 as usize][curr.0 as usize] || val == grid[curr.1 as usize][curr.0 as usize] + 1 {
                q.push_back(((coord.0, coord.1), distance + 1));
            }
        }
    }
}