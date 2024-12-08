use regex::Regex;
use std::fs;

fn main() {
    let operands = parse("input.txt").unwrap();
    let sum = operands.iter().map(|(x, y)| x * y).sum::<isize>();
    println!("Part 1: {}", sum);
}

fn parse(filename: &str) -> Result<Vec<(isize, isize)>, Box<dyn std::error::Error>> {
    let contents = fs::read_to_string(filename)?;
    let re = Regex::new(r"mul\((\d+),(\d+)\)")?;

    let mut operands: Vec<(isize, isize)> = Vec::new();
    for (_, [x, y]) in re.captures_iter(&contents).map(|c| c.extract()) {
        operands.push((x.parse()?, y.parse()?));
    }

    Ok(operands)
}

#[test]
fn test_day3() {
    let expected_operands = vec![(2, 4), (5, 5), (11, 8), (8, 5)];
    let operands = parse("test-input.txt").unwrap();
    assert_eq!(expected_operands, operands);
    let expected_sum: isize = 161;
    let sum = operands.iter().map(|(x, y)| x * y).sum::<isize>();
    assert_eq!(expected_sum, sum);
}
