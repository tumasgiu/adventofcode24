use crate::Instruction::{Do, Dont, Mul};
use regex::Regex;
use std::fs;

fn main() {
    let program = parse("input.txt").unwrap();
    let sum = run_program(&program, false);
    println!("Part 1: {}", sum);
    let sum = run_program(&program, true);
    println!("Part 2: {}", sum);
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Instruction {
    Do,
    Dont,
    Mul(isize, isize),
}

fn parse(filename: &str) -> Result<Vec<Instruction>, Box<dyn std::error::Error>> {
    let contents = fs::read_to_string(filename)?;
    let re = Regex::new(r"(don't)(\(\))|(do)(\(\))|mul\((\d+),(\d+)\)")?;

    let mut program: Vec<Instruction> = Vec::new();
    for (_, [x, y]) in re.captures_iter(&contents).map(|c| c.extract()) {
        match x {
            "do" => {
                program.push(Do);
            }
            "don't" => {
                program.push(Dont);
            }
            &_ => {
                program.push(Mul(x.parse()?, y.parse()?));
            }
        }
    }

    Ok(program)
}

fn run_program(program: &Vec<Instruction>, enable_conditionals: bool) -> isize {
    let mut sum = 0;
    let mut mul_enabled = true;
    for instruction in program {
        match instruction {
            Do => {
                if enable_conditionals {
                    mul_enabled = true
                }
            }
            Dont => {
                if enable_conditionals {
                    mul_enabled = false
                }
            }
            Mul(x, y) => {
                if mul_enabled {
                    sum += x * y;
                }
            }
        }
    }

    sum
}

#[test]
fn test_day3() {
    let expected_program = vec![Mul(2, 4), Dont, Mul(5, 5), Mul(11, 8), Do, Mul(8, 5)];
    let programs = parse("test-input.txt").unwrap();
    assert_eq!(expected_program, programs);
    let expected_sum: isize = 161;
    let sum = run_program(&programs, false);
    assert_eq!(expected_sum, sum);
    let expected_sum: isize = 48;
    let sum = run_program(&programs, true);
    assert_eq!(expected_sum, sum);
}
