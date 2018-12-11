use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    println!("{:?}", part1(&input));
}

fn part1(input: &str) -> i32 {
    let mut sum: i32 = 0;

    for line in input.lines() {
        sum += line.parse::<i32>().unwrap();
    }

    sum
}
