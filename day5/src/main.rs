use regex::Regex;
use std::fs;

fn main() {
    let pq = parse("input.txt").unwrap();
    println!("Part 1: {}", pq.part1())
}

#[derive(Debug)]
struct PrintQueue {
    rules: Vec<(usize, usize)>,
    updates: Vec<Vec<usize>>,
}

impl PrintQueue {
    fn is_update_correct(&self, index: usize) -> bool {
        let update = &self.updates[index];
        for rule in &self.rules {
            let a = update.iter().position(|n| *n == rule.0);
            let b = update.iter().position(|n| *n == rule.1);
            if let (Some(a), Some(b)) = (a, b) {
                if b < a {
                    return false;
                }
            }
        }

        true
    }

    fn correct_updates(&self) -> Vec<&Vec<usize>> {
        self.updates
            .iter()
            .enumerate()
            .filter(|(i, _)| self.is_update_correct(*i))
            .map(|(_, u)| u)
            .collect()
    }

    fn part1(&self) -> usize {
        let mut sum = 0;
        for update in self.correct_updates() {
            if update.len() % 2 == 0 {
                panic!("uneven update")
            }
            sum += update[update.len() / 2]
        }
        sum
    }
}

fn parse(filename: &str) -> Result<PrintQueue, Box<dyn std::error::Error>> {
    let contents = fs::read_to_string(filename)?;
    let s: Vec<_> = contents.split("\n\n").collect();
    let rules_source = s[0];
    let updates_source = s[1];

    let rules_re = Regex::new(r"(\d+)\|(\d+)")?;

    let mut rules: Vec<(usize, usize)> = vec![];
    for (_, [x, y]) in rules_re.captures_iter(rules_source).map(|c| c.extract()) {
        rules.push((x.parse()?, y.parse()?));
    }

    let mut updates: Vec<Vec<usize>> = vec![];
    updates_source.split("\n").for_each(|line| {
        if line.is_empty() {
            return;
        }
        let numbers = line.split(",");
        updates.push(numbers.map(|n| n.parse().unwrap()).collect());
    });

    Ok(PrintQueue { rules, updates })
}

#[test]
fn test_day5() {
    let pq = parse("test-input.txt").unwrap();
    let updates: Vec<Vec<usize>> = pq.correct_updates().iter().map(|u| u.to_vec()).collect();
    let expected_correct_updates: Vec<Vec<usize>> = vec![
        vec![75, 47, 61, 53, 29],
        vec![97, 61, 53, 29, 13],
        vec![75, 29, 13],
    ];
    assert_eq!(updates, expected_correct_updates);
    assert_eq!(pq.part1(), 143);
}
