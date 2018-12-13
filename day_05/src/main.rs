use std::io::{self, Read};

#[derive(Debug)]
enum ExclusionUnit {
    Unit(u8),
    NoExclusion,
}

impl PartialEq<u8> for ExclusionUnit {
    fn eq(&self, rhs: &u8) -> bool {
        match *self {
            ExclusionUnit::Unit(x) => (x == *rhs) || will_react(x, *rhs),
            ExclusionUnit::NoExclusion => false,
        }
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    println!("{}", part1(&input.trim()));
    println!("{}", part2(&input.trim()));
}

fn part1(input: &str) -> usize {
    chain_react(&input, ExclusionUnit::NoExclusion)
}

fn part2(input: &str) -> usize {
    (b'A'..b'Z')
        .map(|b| chain_react(input, ExclusionUnit::Unit(b)))
        .min()
        .unwrap()
}

fn chain_react(polymer: &str, excl_unit: ExclusionUnit) -> usize {
    let mut stack = vec![];

    for b in polymer.as_bytes() {
        if excl_unit == *b {
            continue;
        }

        stack.push(b);

        let len = stack.len();

        if len >= 2 {
            if will_react(*stack[len - 1], *stack[len - 2]) {
                stack.pop();
                stack.pop();
            }
        }
    }

    stack.len()
}

fn will_react(b1: u8, b2: u8) -> bool {
    ((b1 as i8) - (b2 as i8)).abs() == 32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn excl_unit_matches_same_u8() {
        assert_eq!(ExclusionUnit::Unit(10), 10u8);
    }

    #[test]
    fn excl_unit_doesnt_match_diff_u8() {
        assert_ne!(ExclusionUnit::Unit(10), 11u8);
    }

    #[test]
    fn excl_unit_matches_complement_u8() {
        assert_eq!(ExclusionUnit::Unit(65), 97u8);
        assert_eq!(ExclusionUnit::Unit(97), 65u8);
    }

    #[test]
    fn no_excl_doesnt_match_u8() {
        assert_ne!(ExclusionUnit::NoExclusion, 42u8);
    }
}
