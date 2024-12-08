use std::cmp::PartialEq;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn main() {
    let reports = parse("input.txt").unwrap();
    let safeties: Vec<bool> = reports.iter().map(|r| is_safe(r, None, false)).collect();
    let safe_count = safeties
        .iter()
        .fold(0, |acc, safe| if *safe { acc + 1 } else { acc });
    println!("Part 1: {}", safe_count);

    let safeties: Vec<bool> = reports.iter().map(|r| is_safe(r, None, true)).collect();
    let safe_count = safeties
        .iter()
        .fold(0, |acc, safe| if *safe { acc + 1 } else { acc });
    println!("Part 2: {}", safe_count);
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

fn is_safe(
    report: &Vec<i32>,
    current_removed_index: Option<usize>,
    problem_dampener: bool,
) -> bool {
    #[derive(Debug, Eq, PartialEq)]
    enum Direction {
        Increasing,
        Decreasing,
    }

    let r: &Vec<i32>;
    let mut r2: Vec<i32>;
    if problem_dampener && current_removed_index.is_some() {
        let i = current_removed_index.unwrap();
        if i == report.len() {
            return false;
        }
        r2 = report.clone();
        r2.remove(i);
        r = &r2
    } else {
        r = report;
    }

    let mut safe = true;
    let mut current_direction: Option<Direction> = None;
    for (i, _v) in r.iter().enumerate() {
        if i == 0 {
            continue;
        }
        let diff = r[i - 1] - r[i];
        let distance = diff.abs();
        if distance == 0 || distance > 3 {
            safe = false;
            if !problem_dampener {
                return safe;
            } else {
                break;
            }
        }

        if safe {
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
                        safe = false;
                        if !problem_dampener {
                            return safe;
                        } else {
                            break;
                        }
                    }
                }
            }
        }
    }

    if problem_dampener && !safe {
        let i = if let Some(index) = current_removed_index {
            index as isize
        } else {
            -1
        };
        return is_safe(report, Some((i + 1) as usize), true);
    }

    safe
}

#[test]
fn test_day2() {
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
    let safeties: Vec<bool> = reports.iter().map(|r| is_safe(r, None, false)).collect();
    assert_eq!(expected_safeties, safeties);

    // part 2
    let expected_safeties = vec![true, false, false, true, true, true];
    let safeties: Vec<bool> = reports.iter().map(|r| is_safe(r, None, true)).collect();
    assert_eq!(expected_safeties, safeties);
}
