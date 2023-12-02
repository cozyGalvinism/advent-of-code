advent_of_code::solution!(2);

struct Game {
    id: u32,
    draws: Vec<Draw>,
}

impl Game {
    fn is_possible(&self, max_red: u32, max_green: u32, max_blue: u32) -> bool {
        self.draws.iter().all(|draw| {
            draw.red <= max_red && draw.green <= max_green && draw.blue <= max_blue
        })
    }

    fn minimum_red(&self) -> u32 {
        self.draws.iter().map(|draw| draw.red).max().unwrap_or(0)
    }

    fn minimum_green(&self) -> u32 {
        self.draws.iter().map(|draw| draw.green).max().unwrap_or(0)
    }

    fn minimum_blue(&self) -> u32 {
        self.draws.iter().map(|draw| draw.blue).max().unwrap_or(0)
    }

    fn power(&self) -> u32 {
        self.minimum_red() * self.minimum_green() * self.minimum_blue()
    }
}

struct Draw {
    red: u32,
    green: u32,
    blue: u32,
}

fn process_line(input: &str) -> Option<Game> {
    let game_id = input
        .split_whitespace()
        .nth(1)?
        .trim_end_matches(':')
        .parse::<u32>()
        .ok()?;
    let line = input.split(':').nth(1)?;
    let mut draws = Vec::new();
    for draw in line.split(';') {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;
        for colours in draw.split(',') {
            let mut colours = colours.split_whitespace();
            let count = colours.next()?.parse::<u32>().ok()?;
            let colour = colours.next()?;
            match colour {
                "blue" => blue += count,
                "red" => red += count,
                "green" => green += count,
                _ => return None,
            }
        }
        draws.push(Draw { red, green, blue });
    }

    Some(Game { id: game_id, draws })
}

pub fn part_one(input: &str) -> Option<u32> {
    input
        .lines()
        .filter_map(process_line)
        .filter(|game| game.is_possible(12, 13, 14))
        .map(|game| Some(game.id))
        .sum()
}

pub fn part_two(input: &str) -> Option<u32> {
    input
        .lines()
        .filter_map(process_line)
        .map(|game| Some(game.power()))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
