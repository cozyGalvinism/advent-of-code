advent_of_code::solution!(1);

fn process_line(input: &str) -> String {
    let number_words = [
        ("zero", "0"),
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ];

    let mut first_digit = None;
    let mut last_digit = None;

    for (i, c) in input.chars().enumerate() {
        if c.is_ascii_digit() {
            match first_digit {
                None => first_digit = Some(c.to_string()),
                Some(_) => last_digit = Some(c.to_string()),
            }
        } else if c.is_alphabetic() {
            for (word, digit) in number_words.iter() {
                if input[i..].starts_with(word) {
                    match first_digit {
                        None => first_digit = Some(digit.to_string()),
                        Some(_) => last_digit = Some(digit.to_string()),
                    }
                }
            }
        }
    }

    match (first_digit, last_digit) {
        (Some(first), Some(last)) => format!("{}{}", first, last),
        (Some(digit), None) | (None, Some(digit)) => format!("{}{}", digit, digit),
        (None, None) => String::new(),
    }
}

fn parse_lines(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let digits = process_line(line);
            digits.parse::<u32>().unwrap()
        })
        .sum()
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(parse_lines(input))
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(parse_lines(input))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(281));
    }
}
