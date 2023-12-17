use std::vec;

use color_eyre::eyre::Result;

use crate::solver::Answer;

struct Game {
    id: i32,
    sets: Vec<Set>,
}

#[derive(Debug, Eq, PartialEq)]
struct Set {
    red: i32,
    green: i32,
    blue: i32,
}

impl Set {
    fn power(&self) -> i32 {
        self.red * self.green * self.blue
    }
}

impl Game {
    fn new(input: &str) -> Self {
        let v: Vec<&str> = input.split(':').collect();
        assert_eq!(v.len(), 2);

        let id = Game::get_game_id(v.first().unwrap());
        let sets = Game::get_sets(v.last().unwrap());
        Self { id, sets }
    }

    fn possible_with_bag(&self, bag: &Set) -> bool {
        for set in self.sets.iter() {
            if set.red > bag.red || set.green > bag.green || set.blue > bag.blue {
                return false;
            }
        }

        true
    }

    fn minimum_bag(&self) -> Set {
        let mut bag = Set {
            red: 0,
            green: 0,
            blue: 0,
        };

        for set in self.sets.iter() {
            bag.red = std::cmp::max(bag.red, set.red);
            bag.green = std::cmp::max(bag.green, set.green);
            bag.blue = std::cmp::max(bag.blue, set.blue);
        }

        bag
    }

    fn get_sets(input: &str) -> Vec<Set> {
        let mut result = vec![];
        for set_str in input.split(';').map(|s| s.trim()) {
            assert!(!set_str.is_empty());

            result.push(Self::get_set(set_str));
        }

        result
    }

    fn get_set(input: &str) -> Set {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;
        for v in input.split(',').map(|f| f.trim()) {
            let t: Vec<&str> = v.split_whitespace().collect();
            assert_eq!(t.len(), 2);

            let value = t.first().unwrap().parse::<i32>().unwrap();

            if v.contains("red") {
                red += value;
            } else if v.contains("blue") {
                blue += value;
            } else if v.contains("green") {
                green += value;
            }
        }

        Set { red, green, blue }
    }

    fn get_game_id(input: &str) -> i32 {
        // convert "Game 20" into 20

        let v: Vec<&str> = input.split_whitespace().collect();

        assert_eq!(v.len(), 2);

        let id = v.last().unwrap().parse::<i32>().unwrap();

        id
    }
}

pub fn solve(input: &str) -> Result<Answer> {
    let bag = Set {
        red: 12,
        green: 13,
        blue: 14,
    };
    let mut part1 = 0;
    let mut part2 = 0;

    for line in input.lines() {
        let game = Game::new(line);

        if game.possible_with_bag(&bag) {
            part1 += game.id;
        }

        part2 += game.minimum_bag().power();
    }

    Ok(Answer {
        part1: Some(part1.to_string()),
        part2: Some(part2.to_string()),
    })
}

#[cfg(test)]
mod tests {
    use color_eyre::eyre::Result;
    use tracing_test::traced_test;

    use super::{Game, Set};

    const TEST_INPUT: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[traced_test]
    #[test]
    fn test_game_get_id() {
        let vec = vec![("Game 20", 20), ("Game 100", 100), ("Game 1", 1)];

        for v in vec {
            let id = Game::get_game_id(v.0);
            assert_eq!(id, v.1);
        }
    }

    #[traced_test]
    #[test]
    fn test_game_get_set() {
        let vec = vec![
            (
                "1 red, 10 green, 4 blue",
                Set {
                    red: 1,
                    green: 10,
                    blue: 4,
                },
            ),
            (
                "3 blue, 4 red",
                Set {
                    red: 4,
                    green: 0,
                    blue: 3,
                },
            ),
            (
                "1 blue",
                Set {
                    red: 0,
                    green: 0,
                    blue: 1,
                },
            ),
        ];

        for v in vec {
            let id = Game::get_set(v.0);
            assert_eq!(id, v.1);
        }
    }

    #[traced_test]
    #[test]
    fn test_game_get_sets() {
        let vec = vec![(
            "3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green ",
            vec![
                Set {
                    red: 4,
                    green: 0,
                    blue: 3,
                },
                Set {
                    red: 1,
                    green: 2,
                    blue: 6,
                },
                Set {
                    red: 0,
                    green: 2,
                    blue: 0,
                },
            ],
        )];

        for v in vec {
            let id = Game::get_sets(v.0);
            assert_eq!(id, v.1);
        }
    }

    #[traced_test]
    #[test]
    fn test_part1() -> Result<()> {
        let answer = super::solve(TEST_INPUT)?;

        assert_eq!(answer.part1, Some("8".to_string()));

        Ok(())
    }

    #[traced_test]
    #[test]
    fn test_part2() -> Result<()> {
        let answer = super::solve(TEST_INPUT)?;

        assert_eq!(answer.part2, Some("2286".to_string()));

        Ok(())
    }
}
