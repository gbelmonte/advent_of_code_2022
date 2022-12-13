use std::{fs, collections::{HashMap, VecDeque, BinaryHeap}};

fn main() {
    part_1_solution();
    part_2_solution();
}

fn get_function(function: String) -> Box<dyn Fn(i64) -> i64> {
    let mut s = function.trim().split(" ");
    let op = s.nth(4).unwrap().trim();
    let param = s.nth(0).unwrap().trim(); 
    if op == "*" {
        if param == "old" {
            Box::new(|x| x * x)
        } else {
            let y = param.parse::<i64>().unwrap();
            Box::new(move |x| x * y)
        }
    } else {
        let y = param.parse::<i64>().unwrap();
        Box::new(move |x| x + y)
    }
}

fn part_1_solution() {
    let input: Vec<String> = fs::read_to_string("day11/input/part1.txt")
        .expect("Something went wrong reading the file")
        .lines()
        .map(String::from)
        .collect();

    let mut op: HashMap<usize, Box<dyn Fn(i64) -> i64>> = HashMap::new();
    let mut items: HashMap<usize, VecDeque<i64>> = HashMap::new();
    let mut test: HashMap<usize, (i64, usize, usize)> = HashMap::new();
    for (i, lines) in input.chunks(7).enumerate() {
        items.insert(i, lines[1]
            .split(":").nth(1).unwrap().split(",")
            .map(|x| x.trim().parse::<i64>().unwrap())
            .collect()
        );
        op.insert(i, get_function(lines[2].to_string()));
        test.insert(i, 
            (lines[3].split(" ").last().unwrap().parse::<i64>().unwrap(),
                lines[4].split(" ").last().unwrap().parse::<usize>().unwrap(),
                lines[5].split(" ").last().unwrap().parse::<usize>().unwrap())
        );
    }

    let mut inspects: HashMap<usize, i64> = HashMap::new();
    for _ in 0..20 {
        for m in 0..items.len() {
            while let Some(i) = items.get_mut(&m).unwrap().pop_front() {
                inspects.entry(m).and_modify(|x| *x += 1).or_insert(1);

                let mut worry = op.get(&m).unwrap()(i);
                worry /= 3;
                let (div, m1, m2) = test.get(&m).unwrap();
                let to = if worry % div == 0 {m1} else {m2};
                items.get_mut(to).unwrap().push_back(worry);
            }
        }
    }
    let mut heap = inspects.into_values().collect::<BinaryHeap<i64>>();
    println!("{:?}", heap.pop().unwrap() * heap.pop().unwrap());
}

fn part_2_solution() {
    let input: Vec<String> = fs::read_to_string("day11/input/part1.txt")
        .expect("Something went wrong reading the file")
        .lines()
        .map(String::from)
        .collect();

    let mut op: HashMap<usize, Box<dyn Fn(i64) -> i64>> = HashMap::new();
    let mut items: HashMap<usize, VecDeque<i64>> = HashMap::new();
    let mut test: HashMap<usize, (i64, usize, usize)> = HashMap::new();
    for (i, lines) in input.chunks(7).enumerate() {
        items.insert(i, lines[1]
            .split(":").nth(1).unwrap().split(",")
            .map(|x| x.trim().parse::<i64>().unwrap())
            .collect()
        );
        op.insert(i, get_function(lines[2].to_string()));
        test.insert(i, 
            (lines[3].split(" ").last().unwrap().parse::<i64>().unwrap(),
                lines[4].split(" ").last().unwrap().parse::<usize>().unwrap(),
                lines[5].split(" ").last().unwrap().parse::<usize>().unwrap())
        );
    }

    let lcm: i64 = test.values().map(|(x, _, _)| x).product();
    let mut inspects: HashMap<usize, i64> = HashMap::new();
    for _ in 0..10000 {
        for m in 0..items.len() {
            while let Some(i) = items.get_mut(&m).unwrap().pop_front() {
                inspects.entry(m).and_modify(|x| *x += 1).or_insert(1);
                let (div, m1, m2) = test.get(&m).unwrap();

                let mut worry = op.get(&m).unwrap()(i);
                worry = worry % lcm;

                let to = if worry % div == 0 {m1} else {m2};
                items.get_mut(to).unwrap().push_back(worry);
            }
        }
    }
    let mut heap = inspects.into_values().collect::<BinaryHeap<i64>>();
    println!("{:?}", heap.pop().unwrap() * heap.pop().unwrap());
}