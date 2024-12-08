use std::error::Error;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

fn main() {
    let pairs = parse("input.txt").unwrap();

    let total_distance: i32 = pairs.iter().map(|p| p.distance()).sum();

    println!("Part 1 Answer: {}", total_distance);
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

fn parse(filename: &str) -> Result<Vec<Pair>, Box<dyn std::error::Error>> {
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

        left.sort();
        right.sort();
    }

    if left.len() != right.len() {
        return Err(Box::new(Err::MalformedInput));
    }

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

#[test]
fn test() {
    let pairs = parse("test-input.txt").unwrap();

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
