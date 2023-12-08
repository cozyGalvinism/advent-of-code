use std::ops::Deref;

advent_of_code::solution!(7);

static CARDS_P1: &str = "23456789TJQKA";
static CARDS_P2: &str = "J23456789TQKA";

#[derive(Debug, PartialEq, PartialOrd)]
enum HandType {
    HighCard = 0,
    OnePair = 1,
    TwoPairs = 2,
    ThreeOfAKind = 3,
    FullHouse = 4,
    FourOfAKind = 5,
    FiveOfAKind = 6,
}

impl HandType {
    fn for_hand_p1(hand: &str) -> Self {
        let mut counts = [0; 13];
        for card in hand.chars() {
            let index = CARDS_P1.find(card).unwrap();
            counts[index] += 1;
        }
        counts.sort_unstable();
        let last_5 = &counts[8..];
        match last_5 {
            [1, 1, 1, 1, 1] => HandType::HighCard,
            [0, 1, 1, 1, 2] => HandType::OnePair,
            [0, 0, 1, 2, 2] => HandType::TwoPairs,
            [0, 0, 1, 1, 3] => HandType::ThreeOfAKind,
            [0, 0, 0, 2, 3] => HandType::FullHouse,
            [0, 0, 0, 1, 4] => HandType::FourOfAKind,
            [0, 0, 0, 0, 5] => HandType::FiveOfAKind,
            _ => unreachable!(),
        }
    }

    fn for_hand_p2(hand: &str) -> Self {
        let jokers = hand.matches('J').count();
        let mut counts = [0; 13];
        for card in hand.chars() {
            let index = CARDS_P2.find(card).unwrap();
            counts[index] += 1;
        }
        counts.sort_unstable();
        let last_5 = &counts[8..];
        match jokers {
            0 => match last_5 {
                [1, 1, 1, 1, 1] => HandType::HighCard,
                [0, 1, 1, 1, 2] => HandType::OnePair,
                [0, 0, 1, 2, 2] => HandType::TwoPairs,
                [0, 0, 1, 1, 3] => HandType::ThreeOfAKind,
                [0, 0, 0, 2, 3] => HandType::FullHouse,
                [0, 0, 0, 1, 4] => HandType::FourOfAKind,
                [0, 0, 0, 0, 5] => HandType::FiveOfAKind,
                _ => unreachable!(),
            },
            1 => match last_5 {
                [1, 1, 1, 1, 1] => HandType::OnePair,
                [0, 1, 1, 1, 2] => HandType::ThreeOfAKind,
                [0, 0, 1, 2, 2] => HandType::FullHouse,
                [0, 0, 1, 1, 3] => HandType::FourOfAKind,
                [0, 0, 0, 1, 4] => HandType::FiveOfAKind,
                _ => unreachable!(),
            },
            2 => match last_5 {
                [0, 1, 1, 1, 2] => HandType::ThreeOfAKind,
                [0, 0, 1, 2, 2] => HandType::FourOfAKind,
                [0, 0, 0, 2, 3] => HandType::FiveOfAKind,
                _ => unreachable!(),
            },
            3 => match last_5 {
                [0, 0, 1, 1, 3] => HandType::FourOfAKind,
                [0, 0, 0, 2, 3] => HandType::FiveOfAKind,
                _ => unreachable!(),
            },
            4 => match last_5 {
                [0, 0, 0, 1, 4] => HandType::FiveOfAKind,
                _ => unreachable!(),
            },
            5 => HandType::FiveOfAKind,
            _ => unreachable!(),
        }
    }
}

#[derive(PartialEq, Eq)]
struct Hand<'a> {
    cards: &'a str,
    bid: u32,
}

impl<'a> Hand<'a> {
    fn winnings(&self, rank: u32) -> u32 {
        self.bid * rank
    }
}

#[derive(PartialEq, Eq)]
struct Part1Hand<'a>(Hand<'a>);

impl<'a> Deref for Part1Hand<'a> {
    type Target = Hand<'a>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(PartialEq, Eq)]
struct Part2Hand<'a>(Hand<'a>);

impl<'a> Deref for Part2Hand<'a> {
    type Target = Hand<'a>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> Part1Hand<'a> {
    fn new(cards: &'a str, bid: u32) -> Self {
        Part1Hand(Hand { cards, bid })
    }

    fn hand_type(&self) -> HandType {
        HandType::for_hand_p1(self.cards)
    }

    fn card_value_at(&self, index: usize) -> usize {
        CARDS_P1
            .find(self.cards.chars().nth(index).unwrap())
            .unwrap()
    }
}

impl<'a> Part2Hand<'a> {
    fn new(cards: &'a str, bid: u32) -> Self {
        Part2Hand(Hand { cards, bid })
    }

    fn hand_type(&self) -> HandType {
        HandType::for_hand_p2(self.cards)
    }

    fn card_value_at(&self, index: usize) -> usize {
        CARDS_P2
            .find(self.cards.chars().nth(index).unwrap())
            .unwrap()
    }
}

impl<'a> PartialOrd for Part1Hand<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> Ord for Part1Hand<'a> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let self_type = self.hand_type();
        let other_type = other.hand_type();
        if self_type > other_type {
            return std::cmp::Ordering::Greater;
        } else if self_type < other_type {
            return std::cmp::Ordering::Less;
        }

        for i in 0_usize..5_usize {
            let self_card = self.card_value_at(i);
            let other_card = other.card_value_at(i);
            match self_card.cmp(&other_card) {
                std::cmp::Ordering::Greater => return std::cmp::Ordering::Greater,
                std::cmp::Ordering::Less => return std::cmp::Ordering::Less,
                _ => (),
            }
        }

        std::cmp::Ordering::Equal
    }
}

impl<'a> PartialOrd for Part2Hand<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> Ord for Part2Hand<'a> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let self_type = self.hand_type();
        let other_type = other.hand_type();
        if self_type > other_type {
            return std::cmp::Ordering::Greater;
        } else if self_type < other_type {
            return std::cmp::Ordering::Less;
        }

        for i in 0_usize..5_usize {
            let self_card = self.card_value_at(i);
            let other_card = other.card_value_at(i);
            match self_card.cmp(&other_card) {
                std::cmp::Ordering::Greater => return std::cmp::Ordering::Greater,
                std::cmp::Ordering::Less => return std::cmp::Ordering::Less,
                _ => (),
            }
        }

        std::cmp::Ordering::Equal
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut hand_bids = input
        .lines()
        .map(|line| {
            let mut cards = line.split_whitespace();
            let hand = cards.next().unwrap();
            let bid = cards.next().unwrap().parse::<u32>().unwrap();

            Part1Hand::new(hand, bid)
        })
        .collect::<Vec<_>>();
    hand_bids.sort_unstable();

    hand_bids
        .iter()
        .enumerate()
        .map(|(i, hand)| hand.winnings((i + 1) as u32))
        .sum::<u32>()
        .into()
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut hand_bids = input
        .lines()
        .map(|line| {
            let mut cards = line.split_whitespace();
            let hand = cards.next().unwrap();
            let bid = cards.next().unwrap().parse::<u32>().unwrap();

            Part2Hand::new(hand, bid)
        })
        .collect::<Vec<_>>();
    hand_bids.sort_unstable();

    hand_bids
        .iter()
        .enumerate()
        .map(|(i, hand)| hand.winnings((i + 1) as u32))
        .sum::<u32>()
        .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5905));
    }
}
