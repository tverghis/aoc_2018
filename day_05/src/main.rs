use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    println!("{}", part1(&input.trim()));
}

fn part1(input: &str) -> usize {
    let mut stack = vec![];

    for b in input.as_bytes() {
        stack.push(b);

        let len = stack.len();

        if len >= 2 {
            if should_react(*stack[len - 1], *stack[len - 2]) {
                stack.pop();
                stack.pop();
            }
        }
    }

    stack.len()
}

fn should_react(b1: u8, b2: u8) -> bool {
    ((b1 as i8) - (b2 as i8)).abs() == 32
}
