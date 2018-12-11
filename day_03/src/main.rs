use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::io::{self, Read};

struct Claim {
    id: u32,
    points: Vec<Point>,
}

impl Claim {
    fn new(input: &str) -> Claim {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^#(\d+) @ (\d+),(\d+): (\d+)x(\d+)$").unwrap();
        }

        let captures = RE.captures(input).unwrap();

        let id = captures[1].parse().unwrap();

        let start = Point {
            x: captures[2].parse().unwrap(),
            y: captures[3].parse().unwrap(),
        };

        let dimensions = Dimension {
            width: captures[4].parse().unwrap(),
            height: captures[5].parse().unwrap(),
        };

        let mut points = vec![];

        for i in 1..=dimensions.width {
            for j in 1..=dimensions.height {
                points.push(Point {
                    x: start.x + i,
                    y: start.y + j,
                });
            }
        }

        Claim { id, points }
    }
}

#[derive(Clone, Eq, PartialEq, Hash)]
struct Point {
    x: u32,
    y: u32,
}

struct Dimension {
    width: u32,
    height: u32,
}

type ClaimTable<'a> = HashMap<&'a Point, u32>;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let claims = parse_claims(&input);
    let claim_table = make_table(&claims);

    println!("{}", part1(&claim_table));
    println!("{}", part2(&claims, &claim_table).unwrap().id);
}

fn parse_claims(input: &str) -> Vec<Claim> {
    let mut claims = vec![];
    for line in input.lines() {
        let claim = Claim::new(&line);
        claims.push(claim);
    }

    claims
}

fn make_table(claims: &Vec<Claim>) -> ClaimTable {
    let mut claim_table: ClaimTable = HashMap::new();

    for claim in claims {
        for p in &claim.points {
            let ovlaps = claim_table.entry(p).or_default();
            *ovlaps += 1;
        }
    }

    claim_table
}

fn part1(claim_table: &ClaimTable) -> usize {
    claim_table.values().filter(|v| **v > 1).count()
}

fn part2<'b>(claims: &'b Vec<Claim>, claim_table: &ClaimTable) -> Option<&'b Claim> {
    for claim in claims {
        if claim.points.iter().all(|p| claim_table[p] == 1) {
            return Some(claim);
        }
    }

    None
}
