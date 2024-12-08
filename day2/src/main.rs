use std::cmp::PartialEq;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn main() {
    let reports = parse("input.txt").unwrap();
    let safeties: Vec<bool> = reports.iter().map(is_safe).collect();
    let safe_count = safeties
        .iter()
        .fold(0, |acc, safe| if *safe { acc + 1 } else { acc });
    println!("Part 1: {}", safe_count);
}

fn parse(filename: &str) -> Result<Vec<Vec<i32>>, Box<dyn std::error::Error>> {
    let path = Path::new(filename);
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut reports = Vec::<Vec<i32>>::new();
    for line in reader.lines() {
        let line = line?;
        let report: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect();
        reports.push(report);
    }

    Ok(reports)
}

fn is_safe(report: &Vec<i32>) -> bool {
    #[derive(Debug, Eq, PartialEq)]
    enum Direction {
        Increasing,
        Decreasing,
    }

    let mut current_direction: Option<Direction> = None;
    for (i, v) in report.iter().enumerate() {
        if i == 0 {
            continue;
        }
        let diff = report[i - 1] - report[i];
        let distance = diff.abs();
        if distance == 0 || distance > 3 {
            return false;
        }

        match &current_direction {
            None => {
                if diff.is_positive() {
                    current_direction = Option::from(Direction::Increasing);
                } else {
                    current_direction = Option::from(Direction::Decreasing);
                }
            }
            Some(direction) => {
                if diff.is_positive() && *direction != Direction::Increasing
                    || diff.is_negative() && *direction != Direction::Decreasing
                {
                    return false;
                }
            }
        }
    }

    true
}

#[test]
fn test() {
    let expected_reports = vec![
        vec![7, 6, 4, 2, 1],
        vec![1, 2, 7, 8, 9],
        vec![9, 7, 6, 2, 1],
        vec![1, 3, 2, 4, 5],
        vec![8, 6, 4, 4, 1],
        vec![1, 3, 6, 7, 9],
    ];
    let reports = parse("test-input.txt").unwrap();
    assert_eq!(expected_reports, reports);

    let expected_safeties = vec![true, false, false, false, false, true];
    let safeties: Vec<bool> = reports.iter().map(is_safe).collect();

    assert_eq!(expected_safeties, safeties);
}
