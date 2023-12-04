use std::collections::HashMap;

advent_of_code::solution!(4);

#[derive(Clone, Debug)]
struct Card {
    id: u32,
    numbers: Vec<u32>,
    winning_numbers: Vec<u32>,
}

impl Card {
    fn parse(input: &str) -> Card {
        let mut card_name_game = input.split(':');
        let card_name = card_name_game.next().unwrap();
        let card_id = card_name
            .split_whitespace()
            .nth(1)
            .unwrap()
            .parse()
            .unwrap();
        let card = card_name_game.next().unwrap();
        let mut card_split = card.split('|');

        let card_seq = card_split.next().unwrap();
        let winning_seq = card_split.next().unwrap();

        let card_numbers = card_seq
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect::<Vec<u32>>();
        let winning_numbers = winning_seq
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect::<Vec<u32>>();

        Card {
            id: card_id,
            numbers: card_numbers,
            winning_numbers,
        }
    }

    fn matching(&self) -> u32 {
        let mut matching = 0;
        for n in self.numbers.iter() {
            if self.winning_numbers.contains(n) {
                matching += 1;
            }
        }

        matching
    }

    fn points(&self) -> u32 {
        let mut points = 0;
        for n in self.numbers.iter() {
            if self.winning_numbers.contains(n) {
                if points == 0 {
                    points += 1;
                } else {
                    points *= 2;
                }
            }
        }

        points
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    input
        .lines()
        .map(|l| Card::parse(l).points())
        .sum::<u32>()
        .into()
}

pub fn part_two(input: &str) -> Option<u32> {
    let cards = input.lines().map(Card::parse).collect::<Vec<_>>();
    let mut card_register = HashMap::new();

    for card in cards.iter() {
        card_register.entry(card.id).or_insert(1);
        let matching = card.matching();

        for i in 1..=matching {
            for _ in 0..card_register[&card.id] {
                let new_id = card.id + i;
                if new_id >= cards.len() as u32 {
                    break;
                }
                card_register
                    .entry(card.id + i)
                    .and_modify(|e| *e += 1)
                    .or_insert(2);
            }
        }
    }

    card_register.values().sum::<u32>().into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
