use std::{cmp::Ordering, collections::HashMap};

use color_eyre::eyre::Result;

use crate::solver::Answer;

#[derive(Debug)]
enum HandStrength {
    FiveOfKind,
    FourOfKind,
    FullHouse,
    ThreeOfKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl HandStrength {
    fn get_rank(&self) -> u32 {
        match self {
            // bigger is better
            HandStrength::FiveOfKind => 7,
            HandStrength::FourOfKind => 6,
            HandStrength::FullHouse => 5,
            HandStrength::ThreeOfKind => 4,
            HandStrength::TwoPair => 3,
            HandStrength::OnePair => 2,
            HandStrength::HighCard => 1,
        }
    }
}

#[derive(Eq, Debug, Clone)]
struct Card {
    symbol: char,
    count: u32,
}

impl Card {
    fn new(symbol: char, count: u32) -> Self {
        Self { symbol, count }
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        self.count.cmp(&other.count).reverse() // sort descending
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.count == other.count
    }
}

trait ToVecCardTrait {
    fn to_card_vec(&self) -> Vec<Card>;
}

trait VecHandTrait {
    fn sort_hands(&mut self);
    fn calculate(&self) -> u32;
}

impl ToVecCardTrait for HashMap<char, u32> {
    fn to_card_vec(&self) -> Vec<Card> {
        let mut vec = vec![];

        for (symbol, count) in self {
            vec.push(Card::new(*symbol, *count));
        }

        vec
    }
}

impl VecHandTrait for Vec<Hand> {
    fn sort_hands(&mut self) {
        self.sort_by(|a, b| {
            let mut ord = a.strength.cmp(&b.strength); // sort ascending

            let return_value = match ord {
                Ordering::Equal => {
                    for i in 0..a.raw_cards.len() {
                        ord = a.raw_cards[i].cmp(&b.raw_cards[i]);

                        if ord != Ordering::Equal {
                            break;
                        }
                    }
                    return ord;
                }
                _ => ord,
            };

            assert_ne!(return_value, Ordering::Equal);
            return_value
        })
    }

    fn calculate(&self) -> u32 {
        self.iter()
            .enumerate()
            .map(|(rank, hand)| (rank as u32 + 1) * hand.bid)
            .sum()
    }
}

#[derive(Debug)]
struct Hand {
    raw_cards: Vec<u32>,
    strength: u32,
    bid: u32,
}

impl Hand {
    fn new(input: &str, with_joker: bool) -> Self {
        let vec = input.split_whitespace().collect::<Vec<&str>>();

        assert_eq!(vec.len(), 2);

        let bid = vec.last().unwrap().parse::<u32>().unwrap();
        let (cards, raw_cards) = Self::parse_card(vec.first().unwrap(), with_joker);
        let strength = Self::get_strength(cards.clone(), with_joker);

        Self {
            raw_cards,
            strength,
            bid,
        }
    }

    fn get_strength(cards: Vec<Card>, with_joker: bool) -> u32 {
        assert!(cards.len() <= 5);

        // if with_joker, remove J from current cards
        let filtered = cards
            .clone()
            .into_iter()
            .filter(|f| if with_joker { f.symbol != 'J' } else { true })
            .collect::<Vec<Card>>();

        if filtered.is_empty() {
            // this can only happens if with_joker and hands is JJJJJ
            return HandStrength::FiveOfKind.get_rank();
        }

        let mut first_count = filtered.first().unwrap().count;

        if with_joker && first_count < 5 {
            if let Some(j) = cards.iter().find(|f| f.symbol == 'J') {
                first_count += j.count;
            }
        }

        // possibilities:
        // 5
        // 4 + 1
        // 3 + 2
        // 3 + 1 + 1
        // 2 + 2 + 1
        // 2 + 1 + 1 + 1
        // 1 + 1 + 1 + 1 + 1

        // in case of Joker, remove the Joker from card stacks
        // and then add the number of the Joker to the most cards in the stack

        let strength = match filtered.len() {
            1 => HandStrength::FiveOfKind,
            2 => {
                if first_count == 4 {
                    HandStrength::FourOfKind
                } else {
                    HandStrength::FullHouse
                }
            }
            3 => {
                if first_count == 3 {
                    HandStrength::ThreeOfKind
                } else {
                    HandStrength::TwoPair
                }
            }
            4 => HandStrength::OnePair,
            5 => HandStrength::HighCard,
            _ => unreachable!(),
        };

        strength.get_rank()
    }

    fn parse_card(input: &str, with_joker: bool) -> (Vec<Card>, Vec<u32>) {
        let mut map: HashMap<char, u32> = HashMap::new();
        let mut raw_cards = vec![];

        assert_eq!(input.len(), 5);

        for c in input.chars() {
            let kind: u32 = match c {
                'A' => 14,
                'K' => 13,
                'Q' => 12,
                'J' if with_joker => 1,
                'J' => 11,
                'T' => 10,
                _ => c.to_string().parse::<u32>().unwrap(),
            };

            raw_cards.push(kind);

            *map.entry(c).or_insert(0) += 1;
        }

        let mut cards = map.to_card_vec();
        cards.sort();

        (cards, raw_cards)
    }
}

pub fn solve_day07(input: &str) -> Result<Answer> {
    let mut answer = Answer::default();
    let mut hands = vec![];

    // part 1
    for line in input.lines() {
        let card = Hand::new(line, false);
        hands.push(card);
    }

    hands.sort_hands();
    let part1: u32 = hands.calculate();

    answer.part1 = Some(part1.to_string());

    // part 2
    hands.clear();

    for line in input.lines() {
        let hand = Hand::new(line, true);
        hands.push(hand);
    }

    hands.sort_hands();
    let part2: u32 = hands.calculate();

    answer.part2 = Some(part2.to_string());

    Ok(answer)
}

#[cfg(test)]
mod tests {
    use color_eyre::eyre::Result;

    use crate::day07::solve_day07;

    const TEST_INPUT: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    #[test]
    fn test() -> Result<()> {
        let answer = solve_day07(TEST_INPUT)?;

        assert_eq!(answer.part1, Some("6440".to_string()));
        assert_eq!(answer.part2, Some("5905".to_string()));

        Ok(())
    }
}
