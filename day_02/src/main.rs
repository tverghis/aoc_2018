use std::collections::HashSet;
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    println!("{}", part1(&input));
}

fn part1(input: &str) -> u32 {
    let mut doubles = 0;
    let mut triples = 0;

    for line in input.lines() {
        let mut seen_set = HashSet::new();
        let mut has_double = false;
        let mut has_triple = false;

        for c in line.chars() {
            if seen_set.contains(&c) {
                continue;
            }

            let m: Vec<_> = line.matches(c).collect();
            has_double = has_double || (m.len() == 2);
            has_triple = has_triple || (m.len() == 3);
            seen_set.insert(c.clone());
        }

        if has_double {
            doubles += 1;
        }

        if has_triple {
            triples += 1;
        }
    }

    return doubles * triples;
}
