use std::collections::HashSet;

advent_of_code::solution!(13);

type Pos = (i32, i32);

// create a trait for parsing a &str into a section iterator
trait SectionExt {
    fn sections(&self) -> SectionIter;
}

impl SectionExt for str {
    fn sections(&self) -> SectionIter {
        SectionIter {
            s: self,
            current_index: 0,
        }
    }
}

struct SectionIter<'a> {
    s: &'a str,
    current_index: usize,
}

impl<'a> Iterator for SectionIter<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_index >= self.s.len() {
            return None;
        }

        let next_section_start = self.s[self.current_index..]
            .find("\n\n")
            .map(|index| self.current_index + index + 2)
            .unwrap_or(self.s.len());

        let section = &self.s[self.current_index..next_section_start];

        // Skip all consecutive newline characters
        let skip_newlines = section
            .chars()
            .take_while(|&c| c == '\n')
            .count();

        self.current_index = next_section_start + skip_newlines;

        if section.trim().is_empty() {
            self.next()
        } else {
            Some(section.trim_end_matches('\n'))
        }
    }
}

fn parse(input: &str) -> Vec<HashSet<Pos>> {
    input
        .sections()
        .map(|s| {
            s
            .lines()
            .enumerate()
            .flat_map(|(y, row)| {
                row
                    .chars()
                    .enumerate()
                    .filter_map(move |(x, c)| {
                        if c == '#' {
                            Some((x as i32 + 1, y as i32 + 1))
                        } else {
                            None
                        }
                    })
            })
            .collect::<HashSet<_>>()
        })
        .collect()
}

fn has_reflection(
    (x, y): &Pos,
    pattern: &HashSet<Pos>,
    max_x: i32,
    max_y: i32,
    axis: &i32,
    axis_is_x: bool,
) -> bool {
    if axis_is_x {
        x <= axis && (2 * axis - x + 1 > max_x || pattern.contains(&(2 * axis - x + 1, *y)))
            || x > axis && (2 * axis - x + 1 < 1 || pattern.contains(&(2 * axis - x + 1, *y)))
    } else {
        y <= axis && (2 * axis - y + 1 > max_y || pattern.contains(&(*x, 2 * axis - y + 1)))
            || y > axis && (2 * axis - y + 1 < 1 || pattern.contains(&(*x, 2 * axis - y + 1)))
    }
}

fn summary_p1(pattern: &HashSet<Pos>) -> u32 {
    let max_x = pattern.iter().max_by(|a, b| a.0.cmp(&b.0)).unwrap().0;
    let max_y = pattern.iter().max_by(|a, b| a.1.cmp(&b.1)).unwrap().1;

    ((1..max_x)
        .find(|column| {
            pattern
                .iter()
                .all(|coords| has_reflection(coords, pattern, max_x, max_y, column, true))
        })
        .unwrap_or(0)
        + 100 * (1..max_y)
            .find(|row| {
                pattern
                    .iter()
                    .all(|coords| has_reflection(coords, pattern, max_x, max_y, row, false))
            })
            .unwrap_or(0)) as u32
}

fn summary_p2(pattern: &HashSet<Pos>) -> u32 {
    let max_x = pattern.iter().max_by(|a, b| a.0.cmp(&b.0)).unwrap().0;
    let max_y = pattern.iter().max_by(|a, b| a.1.cmp(&b.1)).unwrap().1;

    ((1..max_x)
        .find(|column| {
            pattern
                .iter()
                .filter(|coords| {
                    has_reflection(coords, pattern, max_x, max_y, column, true)
                })
                .count()
                == pattern.len() - 1
        })
        .unwrap_or(0)
        + 100 * (1..max_y)
            .find(|row| {
                pattern
                    .iter()
                    .filter(|coords| {
                        has_reflection(coords, pattern, max_x, max_y, row, false)
                    })
                    .count()
                    == pattern.len() - 1
            })
            .unwrap_or(0)) as u32
}

pub fn part_one(input: &str) -> Option<u32> {
    let tiles = parse(input);
    tiles.iter().map(summary_p1).sum::<u32>().into()
}

pub fn part_two(input: &str) -> Option<u32> {
    let tiles = parse(input);
    tiles.iter().map(summary_p2).sum::<u32>().into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(405));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(400));
    }
}
