advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<i64> {
    input
        .lines()
        .map(|line| {
            let readings = line.split_whitespace().map(|reading| reading.parse::<i64>().unwrap())
                .collect::<Vec<i64>>();
            let mut differences = Vec::new();
            let difference_len = readings.len() - 1;
            differences.push(readings);
            for i in 0..difference_len {
                let previous = &differences[i];
                let differences_lower = previous.windows(2)
                    .map(|window| window[1] - window[0])
                    .collect::<Vec<i64>>();
                if differences_lower.iter().any(|d| *d != 0) {
                    differences.push(differences_lower);
                } else {
                    break;
                }
            }

            differences.iter().rev().fold(0, |acc, difference| acc + difference.last().unwrap())
        })
        .sum::<i64>()
        .into()
}

pub fn part_two(input: &str) -> Option<i64> {
    input
        .lines()
        .map(|line| {
            let readings = line.split_whitespace().map(|reading| reading.parse::<i64>().unwrap())
                .collect::<Vec<i64>>();
            let mut differences = Vec::new();
            let difference_len = readings.len() - 1;
            differences.push(readings);
            for i in 0..difference_len {
                let previous = &differences[i];
                let differences_lower = previous.windows(2)
                    .map(|window| window[0] - window[1])
                    .collect::<Vec<i64>>();
                if differences_lower.iter().any(|d| *d != 0) {
                    differences.push(differences_lower);
                } else {
                    break;
                }
            }

            differences.iter().rev().fold(0, |acc, difference| acc + difference.first().unwrap())
        })
        .sum::<i64>()
        .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() { 
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}
