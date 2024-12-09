use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn main() {
    let mut ws = parse("input.txt").unwrap();
    let matches = ws.part1();
    println!("Part 1 : {}", matches);
}

#[derive(Debug, Eq, PartialEq)]
struct WordSearch {
    matrix: Vec<Vec<char>>,
    // relevant characters positions
    chars_map: HashMap<char, Vec<(usize, usize)>>,
}

#[derive(Debug)]
enum Direction {
    North,
    East,
    South,
    West,
    NorthEast,
    SouthEast,
    NorthWest,
    SouthWest,
}

impl Direction {
    fn vector(&self) -> (isize, isize) {
        match self {
            Direction::North => (0, -1),
            Direction::NorthEast => (1, -1),
            Direction::East => (1, 0),
            Direction::SouthEast => (1, 1),
            Direction::South => (0, 1),
            Direction::SouthWest => (-1, 1),
            Direction::West => (-1, 0),
            Direction::NorthWest => (-1, -1),
        }
    }

    fn iter() -> impl Iterator<Item = Self> {
        [
            Direction::North,
            Direction::South,
            Direction::East,
            Direction::West,
            Direction::NorthEast,
            Direction::SouthEast,
            Direction::NorthWest,
            Direction::SouthWest,
        ]
        .into_iter()
    }
}

impl WordSearch {
    fn character_at(&self, x: usize, y: usize) -> char {
        self.matrix[y][x]
    }

    fn part1(&mut self) -> usize {
        let mut matches: usize = 0;
        let start_positions = self.chars_map.get(&'X').unwrap();
        for start_position in start_positions {
            for direction in Direction::iter() {
                let mut current_char = 'X';
                let mut current_position = *start_position;
                let (dx, dy) = direction.vector();
                for _ in 1..=4 {
                    let nx = current_position.0 as isize + dx;
                    let ny = current_position.1 as isize + dy;
                    if nx.is_negative()
                        || ny.is_negative()
                        || ny > self.matrix.len() as isize - 1
                        || nx > self.matrix[ny as usize].len() as isize - 1
                    {
                        break;
                    }
                    let new_pos = (nx as usize, ny as usize);
                    let next = self.character_at(new_pos.0, new_pos.1);
                    match current_char {
                        'X' => {
                            if next != 'M' {
                                break;
                            } else {
                                current_char = next;
                                current_position = new_pos;
                            }
                        }
                        'M' => {
                            if next != 'A' {
                                break;
                            } else {
                                current_char = next;
                                current_position = new_pos;
                            }
                        }
                        'A' => {
                            if next != 'S' {
                                break;
                            } else {
                                matches += 1;
                                break;
                            }
                        }
                        _ => {
                            break;
                        }
                    }
                }
            }
        }
        matches
    }
}

fn parse(filename: &str) -> Result<WordSearch, Box<dyn std::error::Error>> {
    let path = Path::new(filename);
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut matrix = Vec::new();
    let mut chars_map = HashMap::new();
    for (line_index, line) in reader.lines().enumerate() {
        let mut line_vec = Vec::new();
        for (char_index, character) in line?.chars().enumerate() {
            match character {
                'X' | 'M' | 'A' | 'S' => {
                    let v: &mut Vec<(usize, usize)> =
                        chars_map.entry(character).or_insert_with(Vec::new);
                    v.push((char_index, line_index));
                }
                _ => {}
            }
            line_vec.push(character);
        }
        matrix.push(line_vec);
    }

    Ok(WordSearch { matrix, chars_map })
}

#[test]
fn test_day4() {
    let mut ws = parse("test-input.txt").unwrap();
    let matches = ws.part1();
    assert_eq!(matches, 18);
}
