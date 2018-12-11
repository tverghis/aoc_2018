use std::collections::HashSet;
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    println!("{}", part1(&input));
    println!("{}", part2(&input));
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
            seen_set.insert(c);
        }

        if has_double {
            doubles += 1;
        }

        if has_triple {
            triples += 1;
        }
    }

    doubles * triples
}

fn part2(input: &str) -> String {
    let mut ids = ("", "");

    for (n, id) in input.lines().enumerate() {
        for cmp_id in input.lines().skip(n + 1) {
            // Assume that input lines all have the same length
            assert_eq!(id.len(), cmp_id.len());

            let mut differences = 0;

            for (n1, c1) in id.chars().enumerate() {
                if cmp_id.chars().nth(n1).unwrap() != c1 {
                    differences += 1;
                }

                if differences > 1 {
                    break;
                }
            }

            if differences == 1 {
                ids = (id, cmp_id);
            }
        }
    }

    ids.0
        .chars()
        .zip(ids.1.chars())
        .filter(|(c1, c2)| c1 == c2)
        .map(|(c1, _)| c1)
        .collect()
}
