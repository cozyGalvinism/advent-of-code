advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u32> {
    let mut lines = input.lines();
    let time_values = lines
        .next()?
        .split(':')
        .nth(1)?
        .split_whitespace()
        .map(|x| x.parse::<u32>().ok())
        .collect::<Option<Vec<u32>>>()?;
    let distance_values = lines
        .next()?
        .split(':')
        .nth(1)?
        .split_whitespace()
        .map(|x| x.parse::<u32>().ok())
        .collect::<Option<Vec<u32>>>()?;

    let mut possibilities = 0;
    for race in 0..time_values.len() {
        let mut winning_possibilities = 0;
        let time = time_values[race];
        let record_distance = distance_values[race];
        for hold_time in 0..=time {
            let acceleration_time = time - hold_time;
            let distance = acceleration_time * hold_time;
            if distance > record_distance {
                winning_possibilities += 1;
            }
        }

        if possibilities == 0 {
            possibilities = winning_possibilities;
        } else {
            possibilities *= winning_possibilities;
        }
    }

    Some(possibilities)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut lines = input.lines();
    let time = lines
        .next()?
        .replace(' ', "")
        .split(':')
        .nth(1)?
        .parse::<u64>()
        .ok()?;
    let record_distance = lines
        .next()?
        .replace(' ', "")
        .split(':')
        .nth(1)?
        .parse::<u64>()
        .ok()?;
    let mut winning_possibilities = 0;

    for hold_time in 0..=time {
        let acceleration_time = time - hold_time;
        let distance = acceleration_time * hold_time;
        if distance > record_distance {
            winning_possibilities += 1;
        }
    }

    Some(winning_possibilities)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }
}
