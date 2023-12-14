use std::{collections::HashMap, str::FromStr};

advent_of_code::solution!(12);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Spring {
    Operational,
    Damaged,
    Unknown,
}

#[derive(Debug, PartialEq)]
struct ConditionRecord {
    springs: Vec<Spring>,
    groups: Vec<usize>,
}

impl ConditionRecord {
    fn possible_arrangements(&self) -> u64 {
        let mut cache = HashMap::new();
        self.possible_arrangements_for_section(&mut cache, 0, 0)
    }

    fn possible_arrangements_for_section(&self, cache: &mut HashMap<(usize, usize), u64>, spring_ix: usize, group_ix: usize,) -> u64 {
        if let Some(cached_value) = cache.get(&(spring_ix, group_ix)) {
            return *cached_value;
        }

        let consume_group = self.groups.get(group_ix).map_or(0, |group_len| {
            if (spring_ix + group_len) > self.springs.len() {
                return 0;
            }

            if (0..*group_len).any(|pos| self.springs.get(spring_ix + pos) == Some(&Spring::Operational)) {
                return 0;
            }

            if self.springs.get(spring_ix + group_len) == Some(&Spring::Damaged) {
                return 0;
            }

            self.possible_arrangements_for_section(cache, spring_ix + group_len + 1, group_ix + 1)
        });

        let skip = match self.springs.get(spring_ix) {
            None => u64::from(group_ix >= self.groups.len()),
            Some(Spring::Damaged) => 0,
            Some(_) => self.possible_arrangements_for_section(cache, spring_ix + 1, group_ix),
        };

        let arrangements = consume_group + skip;
        cache.insert((spring_ix, group_ix), arrangements);
        arrangements
    }

    fn unfold(&self) -> Self {
        let mut springs = Vec::new();
        let mut groups = Vec::new();

        for repeat in 1..=5 {
            springs.extend(&self.springs);
            if repeat != 5 {
                springs.push(Spring::Unknown);
            }

            groups.extend(&self.groups);
        }

        Self { springs, groups }
    }
}

#[derive(Debug, PartialEq)]
struct ParseConditionRecordError;

impl TryFrom<char> for Spring {
    type Error = ParseConditionRecordError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Self::Operational),
            '#' => Ok(Self::Damaged),
            '?' => Ok(Self::Unknown),
            _ => Err(ParseConditionRecordError),
        }
    }
}

impl FromStr for ConditionRecord {
    type Err = ParseConditionRecordError;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        if let Some((springs_txt, groups_txt)) = line.split_once(' ') {
            let mut springs = Vec::new();
            for spring in springs_txt.chars() {
                let spring = spring.try_into()?;
                springs.push(spring);
            }

            let mut groups = Vec::new();
            for group in groups_txt.split(',') {
                let group = group.parse().map_err(|_| ParseConditionRecordError)?;
                groups.push(group);
            }

            Ok(Self { springs, groups })
        } else {
            Err(ParseConditionRecordError)
        }
    }
}

fn total_possible_arrangements(input: &str, unfold: bool) -> u64 {
    input
        .lines()
        .map(|line| match line.parse::<ConditionRecord>() {
            Ok(record) => {
                if unfold {
                    record.unfold().possible_arrangements()
                } else {
                    record.possible_arrangements()
                }
            },
            Err(_) => 0,
        })
        .sum()
}

pub fn part_one(input: &str) -> Option<u64> {
    total_possible_arrangements(input, false).into()
}

pub fn part_two(input: &str) -> Option<u64> {
    total_possible_arrangements(input, true).into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(525_152));
    }
}
