use std::collections::{HashMap, HashSet};

use crate::solver::Answer;
use color_eyre::eyre::Result;

struct Schematic {
    symbols: HashMap<(i32, i32), String>,
    numbers: HashMap<(i32, i32), (i32, i32)>,
}

impl Schematic {
    fn new(input: &str) -> Self {
        let mut y_stack = vec![];
        let mut symbols = HashMap::new();
        let mut numbers = HashMap::new();

        let mut number_id = 0; // to prevent calculating the same number multiple times

        for line in input.lines() {
            if line.is_empty() {
                continue;
            }

            let mut x_stack = vec![];
            // make peekable to see 1 char ahead
            let mut x_iterator = line.chars().peekable();

            let mut number_stacks = vec![];
            let mut number_location = vec![];

            while let Some(value) = x_iterator.next() {
                let value_string = value.to_string();
                let coordinate = (x_stack.len() as i32, y_stack.len() as i32);

                if value.is_numeric() {
                    number_stacks.push(value);
                    number_location.push(coordinate);
                } else if value != '.' {
                    symbols.insert(coordinate, value_string.clone());
                    number_stacks.clear();
                    number_location.clear();
                } else {
                    number_stacks.clear();
                    number_location.clear();
                }

                // peek, if next is none or not a number, that means the number sequence is done
                if x_iterator.peek().is_none() || !x_iterator.peek().unwrap().is_numeric() {
                    let n = number_stacks.iter().collect::<String>();

                    for location in number_location.iter() {
                        numbers.insert(*location, (number_id, n.parse::<i32>().unwrap()));
                    }

                    number_stacks.clear();
                    number_location.clear();
                    number_id += 1;
                }

                x_stack.push(value_string);
            }
            y_stack.push(x_stack);
        }

        Self { symbols, numbers }
    }

    fn get_all_number_around_symbols(&self) -> Vec<i32> {
        let mut results = vec![];
        let mut seen = HashSet::new();

        for ((base_x, base_y), _) in self.symbols.iter() {
            for y in [-1, 0, 1] {
                for x in [-1, 0, 1] {
                    if x == 0 && y == 0 {
                        continue;
                    }

                    if let Some((id, value)) = self.numbers.get(&(base_x + x, base_y + y)) {
                        if !seen.contains(id) {
                            results.push(*value);
                        }
                        seen.insert(*id);
                    }
                }
            }
        }

        results
    }

    fn get_gear_ratio(&self) -> Vec<i32> {
        let mut results = vec![];

        for ((base_x, base_y), symbol) in self.symbols.iter() {
            if symbol != "*" {
                continue;
            }

            let mut current = vec![];
            let mut seen = HashSet::new();

            for y in [-1, 0, 1] {
                for x in [-1, 0, 1] {
                    if x == 0 && y == 0 {
                        continue;
                    }

                    if let Some((id, value)) = self.numbers.get(&(base_x + x, base_y + y)) {
                        if !seen.contains(id) {
                            current.push(*value);
                        }
                        seen.insert(*id);
                    }
                }
            }

            if seen.len() == 2 {
                assert_eq!(current.len(), seen.len());
                results.push(current.iter().product());
            };
        }

        results
    }
}

pub fn solve_day03(input: &str) -> Result<Answer> {
    let schematic = Schematic::new(input);
    let part1: i32 = schematic.get_all_number_around_symbols().iter().sum();
    let part2: i32 = schematic.get_gear_ratio().iter().sum();

    Ok(Answer {
        part1: Some(part1.to_string()),
        part2: Some(part2.to_string()),
    })
}

#[cfg(test)]
mod tests {
    use super::Schematic;

    #[test]
    fn test_part1() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

        let schematic = Schematic::new(input);
        let v: i32 = schematic.get_all_number_around_symbols().iter().sum();

        assert_eq!(v, 4361)
    }

    #[test]
    fn test_part2() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

        let schematic = Schematic::new(input);
        let gear_ratio = &schematic.get_gear_ratio();

        assert_eq!(gear_ratio, &Vec::from([451490, 16345]));

        let value: i32 = gear_ratio.iter().sum();

        assert_eq!(value, 467835)
    }
}
