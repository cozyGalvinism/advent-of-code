use regex::Regex;

advent_of_code::solution!(3);

// 467..114..
// ...*......
// ..35..633.
// ......#...
// 617*......
// .....+.58.
// ..592.....
// ......755.
// ...$.*....
// .664.598..
//
// add up all numbers that are adjacent to a symbol (467 would be adjacent because of the 7)
// . means empty, any other character is a symbol

#[derive(Debug)]
struct Part {
    x: i32,
    y: i32,
    symbol: String,
}

impl Part {
    fn next_to(&self, other: &Part) -> bool {
        (other.y - self.y).abs() <= 1
            && self.x <= other.x + other.symbol.len() as i32
            && other.x <= self.x + self.symbol.len() as i32
    }

    fn value(&self) -> u32 {
        self.symbol.parse().unwrap()
    }
}

fn parse(input: &str, r: Regex) -> Vec<Part> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            r.find_iter(line)
                .map(|m| Part {
                    x: m.start() as i32,
                    y: y as i32,
                    symbol: m.as_str().to_string(),
                })
                .collect::<Vec<_>>()
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let symbols = parse(input, Regex::new(r"[^.0-9]").unwrap());
    let numbers = parse(input, Regex::new(r"\d+").unwrap());

    numbers
        .iter()
        .filter_map(|n| {
            if symbols.iter().any(|s| s.next_to(n)) {
                Some(n.value())
            } else {
                None
            }
        })
        .sum::<u32>()
        .into()
}

pub fn part_two(input: &str) -> Option<u32> {
    let gears = parse(input, Regex::new(r"\*").unwrap());
    let numbers = parse(input, Regex::new(r"\d+").unwrap());

    gears
        .iter()
        .filter_map(|g| {
            let neighbours = numbers
                .iter()
                .filter(|n| g.next_to(n))
                .map(|n| n.value())
                .collect::<Vec<_>>();
            if neighbours.len() == 2 {
                Some(neighbours[0] * neighbours[1])
            } else {
                None
            }
        })
        .sum::<u32>()
        .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}
