use std::fs;

fn main() {
    part_1_solution();
    part_2_solution();
}

fn part_1_solution() {
    let overlaps = fs::read_to_string("day4/input/part1.txt")
        .expect("Something went wrong reading the file")
        .lines()
        .map(|line| {
            line.split(',')
                .map(|range| { 
                    range.split('-')
                        .map(|x| x.parse::<i32>().unwrap())
                        .collect::<Vec<i32>>()
                })
                .collect::<Vec<Vec<i32>>>()  
        })
        .filter(|partner| {
            let min = i32::min(partner[0][0], partner[1][0]);
            let max = i32::max(partner[0][1], partner[1][1]);
            (min == partner[0][0] && max == partner[0][1]) || (min == partner[1][0] && max == partner[1][1]) 
        })
        .count();

    println!("Number of complete overlaps {}", overlaps)
}

fn part_2_solution() {
    let overlaps = fs::read_to_string("day4/input/part1.txt")
        .expect("Something went wrong reading the file")
        .lines()
        .map(|line| {
            line.split(',')
                .map(|range| { 
                    range.split('-')
                        .map(|x| x.parse::<i32>().unwrap())
                        .collect::<Vec<i32>>()
                })
                .collect::<Vec<Vec<i32>>>()  
        })
        .filter(|partner| {
            (partner[1][0] >= partner[0][0] && partner[1][0] <= partner[0][1]) 
            || (partner[1][1] >= partner[0][0] && partner[1][1] <= partner[0][1]) 
            || (partner[0][0] >= partner[1][0] && partner[0][0] <= partner[1][1]) 
            || (partner[0][1] >= partner[1][0] && partner[0][1] <= partner[1][1]) 
        })
        .count();

    println!("Number of overlaps {}", overlaps)
}