use std::{str::FromStr, fmt::Display, collections::HashMap};

advent_of_code::solution!(14);

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum Tile {
    Empty,
    Round,
    Cube,
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Empty => write!(f, "."),
            Tile::Round => write!(f, "O"),
            Tile::Cube => write!(f, "#"),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct Map {
    tiles: Vec<Vec<Tile>>,
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.tiles.iter() {
            for tile in row.iter() {
                write!(f, "{}", tile)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl FromStr for Map {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.lines()
            .map(|line| {
                line.chars()
                    .map(|c| match c {
                        '.' => Ok(Tile::Empty),
                        'O' => Ok(Tile::Round),
                        '#' => Ok(Tile::Cube),
                        _ => Err(()),
                    })
                    .collect()
            })
            .collect::<Result<Vec<_>, _>>()
            .map(|tiles| Self { tiles })
    }
}

impl Map {
    fn rotate_right(&mut self) {
        let mut new_tiles = vec![vec![Tile::Empty; self.tiles.len()]; self.tiles.len()];

        for (y, rows) in self.tiles.iter().enumerate() {
            for (x, tile) in rows.iter().enumerate() {
                new_tiles[x][self.tiles.len() - 1 - y] = *tile;
            }
        }

        self.tiles = new_tiles;
    }

    fn tilt_north(&mut self) {
        for y in 0..self.tiles.len() {
            for x in 0..self.tiles[y].len() {
                if let Tile::Round = self.tiles[y][x] {
                    let mut y = y;
                    while y > 0 {
                        let old_y = y;
                        y -= 1;
                        if let Tile::Cube = self.tiles[y][x] {
                            break;
                        }
                        if let Tile::Round = self.tiles[y][x] {
                            break;
                        }
                        self.tiles[y][x] = Tile::Round;
                        self.tiles[old_y][x] = Tile::Empty;
                    }
                }
            }
        }
    }

    fn total_load(&self) -> usize {
        let rows = self.tiles.len();

        self.tiles
            .iter()
            .enumerate()
            .map(|(y, row)| {
                row.iter().filter(|t| **t == Tile::Round).count() * (rows - y)
            })
            .sum()
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut map = Map::from_str(input).expect("invalid input");
    map.tilt_north();

    Some(map.total_load() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut map = Map::from_str(input).expect("invalid input");

    let mut seen = HashMap::new();
    for i in 1..1000000000 {
        for _ in 0..4 {
            map.tilt_north();
            map.rotate_right();
        }

        if let Some(seen_at) = seen.insert(map.clone(), i) {
            if (1000000000 - i) % (i - seen_at) == 0 {
                break;
            }
        }
    }

    Some(map.total_load() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(136));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(64));
    }
}
