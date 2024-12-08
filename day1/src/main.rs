use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

fn main() {
    let (left, right) = parse_lists("input.txt").unwrap();

    let pairs = get_pairs(&left, &right).unwrap();
    let total_distance: i32 = pairs.iter().map(|p| p.distance()).sum();
    println!("Part 1 Answer: {}", total_distance);

    let similarity_score = compute_score(&left, &right);
    println!("Part 2 Answer: {}", similarity_score);
}

#[derive(Debug, Eq, PartialEq)]
pub enum Err {
    MalformedInput,
}

impl Display for Err {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Err::MalformedInput => {
                write!(f, "Malformed input")
            }
        }
    }
}

impl Error for Err {}

#[derive(Debug, Eq, PartialEq)]
struct Pair {
    left: i32,
    right: i32,
}

fn parse_lists(filename: &str) -> Result<(Vec<i32>, Vec<i32>), Box<dyn Error>> {
    let path = Path::new(filename);
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    let mut left = Vec::new();
    let mut right = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let split: Vec<&str> = line.split_whitespace().collect();

        if split.len() != 2 {
            return Err(Box::new(Err::MalformedInput));
        }

        left.push(split[0].parse::<i32>()?);
        right.push(split[1].parse::<i32>()?);
    }

    if left.len() != right.len() {
        return Err(Box::new(Err::MalformedInput));
    }

    Ok((left, right))
}

fn get_pairs(left: &Vec<i32>, right: &Vec<i32>) -> Result<Vec<Pair>, Err> {
    if left.len() != right.len() {
        return Err(Err::MalformedInput);
    }

    let mut l = left.clone();
    l.sort();
    let mut r = right.clone();
    r.sort();

    let mut pairs = Vec::new();

    for e in left.iter().enumerate() {
        pairs.push(Pair {
            left: left[e.0],
            right: right[e.0],
        });
    }

    Ok(pairs)
}

impl Pair {
    fn distance(&self) -> i32 {
        (self.left - self.right).abs()
    }
}

fn compute_score(left: &[i32], right: &[i32]) -> i32 {
    let mut m = HashMap::new();
    for i in right.iter() {
        match m.contains_key(i) {
            true => {
                m.insert(i, m.get(i).unwrap() + 1);
            }
            false => {
                m.insert(i, 1);
            }
        }
    }

    let mut score = 0;
    for i in left.iter() {
        score += i * m.get(i).unwrap_or(&0);
    }

    score
}

#[test]
fn test_part1() {
    let (left, right) = parse_lists("test-input.txt").unwrap();
    let pairs = get_pairs(&left, &right).unwrap();

    let expected_pairs = vec![
        Pair { left: 1, right: 3 },
        Pair { left: 2, right: 3 },
        Pair { left: 3, right: 3 },
        Pair { left: 3, right: 4 },
        Pair { left: 3, right: 5 },
        Pair { left: 4, right: 9 },
    ];

    assert_eq!(pairs, expected_pairs);

    let expected_distances: Vec<i32> = vec![2, 1, 0, 1, 2, 5];

    let distances: Vec<i32> = pairs.iter().map(|p| p.distance()).collect();

    assert_eq!(distances, expected_distances);

    let expected_total_distance = 11;
    let total_distance: i32 = pairs.iter().map(|p| p.distance()).sum();

    assert_eq!(total_distance, expected_total_distance);
}

#[test]
fn test_part2() {
    let (left, right) = parse_lists("test-input.txt").unwrap();

    let score = compute_score(&left, &right);
    let expected_score = 31;

    assert_eq!(score, expected_score);
}
