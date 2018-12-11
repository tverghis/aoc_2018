use std::collections::HashSet;
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    println!("{:?}", part1(&input));
    println!("{:?}", part2(&input));
}

fn part1(input: &str) -> i32 {
    let mut sum: i32 = 0;

    for line in input.lines() {
        sum += line.parse::<i32>().unwrap();
    }

    sum
}

fn part2(input: &str) -> i32 {
    let mut current_freq = 0;
    let mut seen_set = HashSet::new();

    seen_set.insert(current_freq);

    loop {
        for line in input.lines() {
            current_freq += line.parse::<i32>().unwrap();
            if seen_set.contains(&current_freq) {
                return current_freq;
            }
            seen_set.insert(current_freq);
        }
    }
}
