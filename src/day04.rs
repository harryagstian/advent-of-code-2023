use color_eyre::eyre::Result;

use std::collections::{HashSet, VecDeque};

use crate::solver::Answer;

#[derive(Debug)]
struct Card {
    winning_numbers: HashSet<u32>,
    our_numbers: HashSet<u32>,
}

impl Card {
    fn new(input: &str) -> Self {
        // input: "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53"

        // text: ["Card 1", "41 48 83 86 17 | 83 86  6 31 17  9 48 53"]
        let text = input.split(":").map(|f| f.trim()).collect::<Vec<&str>>();
        assert!(text.len() == 2);

        // text: ["41 48 83 86 17", "83 86  6 31 17  9 48 53"]
        let text = text
            .last()
            .unwrap()
            .split("|")
            .map(|f| f.trim())
            .collect::<Vec<&str>>();
        assert!(text.len() == 2);

        let mut winning_numbers = HashSet::new();
        let mut our_numbers = HashSet::new();

        Self::insert_numbers(text.first().unwrap(), &mut winning_numbers);
        Self::insert_numbers(text.last().unwrap(), &mut our_numbers);

        Self {
            winning_numbers,
            our_numbers,
        }
    }

    fn get_score(&self, card_stacks: &mut VecDeque<u32>) -> (u32, u32) {
        let win_counter = self.our_numbers.intersection(&self.winning_numbers).count() as u32;

        let cards_processed = card_stacks.pop_front().unwrap_or(1_u32);

        for index in 0..win_counter as usize {
            if card_stacks.len() <= index {
                // number of current processed card + 1 original card
                card_stacks.push_back(cards_processed + 1);
            } else {
                card_stacks[index] += cards_processed;
            }
        }

        let score = if win_counter > 0 {
            2_u32.pow(win_counter - 1)
        } else {
            0
        };

        (score, cards_processed)
    }

    fn insert_numbers(text: &str, numbers: &mut HashSet<u32>) {
        for number in text.split_whitespace().map(|f| f.parse::<u32>().unwrap()) {
            numbers.insert(number);
        }
    }
}

pub fn solve_day04(input: &str) -> Result<Answer> {
    let mut part1 = 0;
    let mut part2 = 0;

    let mut card_stacks = VecDeque::new();

    for line in input.lines() {
        let card = Card::new(line);
        let (score, cards_processed) = card.get_score(&mut card_stacks);

        part1 += score;
        part2 += cards_processed;
    }

    Ok(Answer {
        part1: Some(part1.to_string()),
        part2: Some(part2.to_string()),
    })
}

#[cfg(test)]
mod tests {
    use std::collections::VecDeque;

    use crate::day04::Card;
    use color_eyre::eyre::Result;

    use super::solve_day04;

    const TEST_INPUT: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn test_part1() {
        let scores = vec![8, 2, 2, 1, 0, 0, 0];
        let total: u32 = scores.iter().sum();
        let mut copies = VecDeque::new();
        let mut current_score = 0;

        for (index, line) in TEST_INPUT.lines().enumerate() {
            let card = Card::new(line);
            let (score, _) = card.get_score(&mut copies);

            assert_eq!(score, scores[index]);
            current_score += score;
        }

        assert_eq!(current_score, total)
    }

    #[test]
    fn test_part2() -> Result<()> {
        let answer = solve_day04(TEST_INPUT)?;

        assert_eq!(answer.part2, Some("30".to_string()));

        Ok(())
    }
}
