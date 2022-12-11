use std::fs;

fn main() {
    part_1_solution();
    part_2_solution();
}

fn part_1_solution() {
    let grid: Vec<Vec<u32>> = fs::read_to_string("day8/input/part1.txt")
        .expect("Something went wrong reading the file")
        .lines()
        .map(|line| line.chars()
            .map(|x| x.to_digit(10).unwrap())
            .collect())
        .collect();
    
    let width = grid[0].len();
    let height = grid.len();
    let mut visible = 0;
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            let left = (0..x).any(|i| grid[y][i] >= grid[y][x]);
            let right = (x+1..width).any(|i| grid[y][i] >= grid[y][x]);
            let top = (0..y).any(|i| grid[i][x] >= grid[y][x]);
            let down = (y+1..height).any(|i| grid[i][x] >= grid[y][x]);

            if !left || !right || !top || !down {
                visible += 1;
            }
        }
    }
    println!("Number of visible trees {}", visible);
}

fn part_2_solution() {
    let grid: Vec<Vec<u32>> = fs::read_to_string("day8/input/part1.txt")
        .expect("Something went wrong reading the file")
        .lines()
        .map(|line| line.chars()
            .map(|x| x.to_digit(10).unwrap())
            .collect())
        .collect();
    
    let width = grid[0].len();
    let height = grid.len();
    let mut score = 0;
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            let left = (0..x).rev().find(|&i| grid[y][i] >= grid[y][x]).unwrap_or(0);
            let right = (x+1..width).find(|&i| grid[y][i] >= grid[y][x]).unwrap_or(width - 1);
            let top = (0..y).rev().find(|&i| grid[i][x] >= grid[y][x]).unwrap_or(0);
            let down = (y+1..height).find(|&i| grid[i][x] >= grid[y][x]).unwrap_or(height - 1);
            score = score.max((x - left) * (right - x) * (y - top) * (down - y));
        }
    }
    println!("Max scenic score {}", score);
}