use crate::Direction::{East, North, NorthEast, NorthWest, South, SouthEast, SouthWest, West};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn main() {
    let mut ws = parse("input.txt").unwrap();
    let matches = ws.part1();
    println!("Part 1 : {}", matches);
    println!("Part 2 : {}", ws.part2());
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
            North => (0, -1),
            NorthEast => (1, -1),
            East => (1, 0),
            SouthEast => (1, 1),
            South => (0, 1),
            SouthWest => (-1, 1),
            West => (-1, 0),
            NorthWest => (-1, -1),
        }
    }

    fn iter() -> impl Iterator<Item = Self> {
        [
            North, South, East, West, NorthEast, SouthEast, NorthWest, SouthWest,
        ]
        .into_iter()
    }
}

impl WordSearch {
    fn character_at(&self, x: usize, y: usize) -> char {
        self.matrix[y][x]
    }

    fn position_from_direction(
        &self,
        pos: (usize, usize),
        direction: &Direction,
    ) -> Option<(usize, usize)> {
        let (dx, dy) = direction.vector();
        let nx = pos.0 as isize + dx;
        let ny = pos.1 as isize + dy;
        if nx.is_negative()
            || ny.is_negative()
            || ny > self.matrix.len() as isize - 1
            || nx > self.matrix[ny as usize].len() as isize - 1
        {
            return None;
        }
        Some((nx as usize, ny as usize))
    }

    fn part1(&mut self) -> usize {
        let mut matches: usize = 0;
        let start_positions = self.chars_map.get(&'X').unwrap();
        for start_position in start_positions {
            for direction in Direction::iter() {
                let mut current_char = 'X';
                let mut current_position = *start_position;
                for _ in 1..=4 {
                    let new_pos = self.position_from_direction(current_position, &direction);
                    if new_pos.is_none() {
                        break;
                    }
                    let new_pos = new_pos.unwrap();
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

    fn part2(self) -> usize {
        let mut matches: usize = 0;
        let start_positions: &Vec<(usize, usize)> = self.chars_map.get(&'A').unwrap();
        for start_position in start_positions {
            let ne_pos = self.position_from_direction(*start_position, &NorthEast);
            if ne_pos.is_none() {
                continue;
            }
            let ne_pos = ne_pos.unwrap();

            let sw_pos = self.position_from_direction(*start_position, &SouthWest);
            if sw_pos.is_none() {
                continue;
            }
            let sw_pos = sw_pos.unwrap();

            let nw_pos = self.position_from_direction(*start_position, &NorthWest);
            if nw_pos.is_none() {
                continue;
            }
            let nw_pos = nw_pos.unwrap();

            let se_pos = self.position_from_direction(*start_position, &SouthEast);
            if se_pos.is_none() {
                continue;
            }
            let se_pos = se_pos.unwrap();

            if !((self.character_at(ne_pos.0, ne_pos.1) == 'M'
                && self.character_at(sw_pos.0, sw_pos.1) == 'S')
                || (self.character_at(ne_pos.0, ne_pos.1) == 'S'
                    && self.character_at(sw_pos.0, sw_pos.1) == 'M'))
            {
                continue;
            }

            if !((self.character_at(nw_pos.0, nw_pos.1) == 'M'
                && self.character_at(se_pos.0, se_pos.1) == 'S')
                || (self.character_at(nw_pos.0, nw_pos.1) == 'S'
                    && self.character_at(se_pos.0, se_pos.1) == 'M'))
            {
                continue;
            }

            matches += 1;
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
    assert_eq!(ws.part1(), 18);
    assert_eq!(ws.part2(), 9)
}
