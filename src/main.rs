use std::fs::File;
use std::io::{BufRead, BufReader};

fn input_file_to_vec(filename: &str) -> Vec<i32> {
    let reader = BufReader::new(File::open(filename).expect("Cannot open file"));

    reader
        .lines()
        .map(|x| x.unwrap().trim().parse().unwrap())
        .collect()
}

fn window_sums(measurements: &Vec<i32>, window_size: usize) -> Vec<i32> {
    measurements
        .windows(window_size)
        .map(|nums| nums.iter().sum())
        .collect()
}

fn count_window_increases(window_sums: &Vec<i32>) -> usize {
    window_sums
        .iter()
        .zip(window_sums.iter().skip(1))
        .filter(|(a, b)| b > a)
        .count()
}

fn main() {
    println!("Hello, world!");
    let filename = "input/day1.txt";

    let measurements = input_file_to_vec(filename);

    let part1_sums = window_sums(&measurements, 1);
    println!("Part 1 count: {:?}", count_window_increases(&part1_sums));
    let part2_sums = window_sums(&measurements, 3);
    println!("Part 2 count: {:?}", count_window_increases(&part2_sums));
}
