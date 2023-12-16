use std::{collections::{VecDeque, HashSet}, str::FromStr, fmt::{Display, Debug}};

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

advent_of_code::solution!(16);

#[derive(Clone, Copy)]
enum Tile {
    Empty,
    HorizontalMirror,
    VerticalMirror,
    ForwardSlashMirror,
    BackwardSlashMirror
}

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::Up => write!(f, "Up"),
            Direction::Down => write!(f, "Down"),
            Direction::Left => write!(f, "Left"),
            Direction::Right => write!(f, "Right"),
        }
    }
}

impl Debug for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::Up => write!(f, "Up"),
            Direction::Down => write!(f, "Down"),
            Direction::Left => write!(f, "Left"),
            Direction::Right => write!(f, "Right"),
        }
    }
}

impl TryFrom<char> for Tile {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Tile::Empty),
            '-' => Ok(Tile::HorizontalMirror),
            '|' => Ok(Tile::VerticalMirror),
            '/' => Ok(Tile::ForwardSlashMirror),
            '\\' => Ok(Tile::BackwardSlashMirror),
            _ => Err(()),
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Empty => write!(f, "."),
            Tile::HorizontalMirror => write!(f, "-"),
            Tile::VerticalMirror => write!(f, "|"),
            Tile::ForwardSlashMirror => write!(f, "/"),
            Tile::BackwardSlashMirror => write!(f, "\\"),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Pos(usize, usize);

impl Display for Pos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}, {}]", self.1, self.0)
    }
}

impl Debug for Pos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}, {}]", self.1, self.0)
    }
}

struct Grid {
    tiles: Vec<Vec<Tile>>,
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.tiles {
            for tile in row {
                write!(f, "{}", tile)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl FromStr for Grid {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tiles = Vec::new();
        for line in s.lines() {
            let mut row = Vec::new();
            for c in line.chars() {
                row.push(Tile::try_from(c)?);
            }
            tiles.push(row);
        }
        Ok(Grid { tiles })
    }
}

impl Grid {
    fn edges_with_directions(&self) -> Vec<(Pos, Direction)> {
        let mut edges = Vec::new();
        for (i, row) in self.tiles.iter().enumerate() {
            for (j, _) in row.iter().enumerate() {
                if i == 0 && j == 0 {
                    edges.push((Pos(i, j), Direction::Right));
                    edges.push((Pos(i, j), Direction::Down));
                } else if i == 0 && j == row.len() - 1 {
                    edges.push((Pos(i, j), Direction::Left));
                    edges.push((Pos(i, j), Direction::Down));
                } else if i == self.tiles.len() - 1 && j == 0 {
                    edges.push((Pos(i, j), Direction::Right));
                    edges.push((Pos(i, j), Direction::Up));
                } else if i == self.tiles.len() - 1 && j == row.len() - 1 {
                    edges.push((Pos(i, j), Direction::Left));
                    edges.push((Pos(i, j), Direction::Up));
                } else if i == 0 {
                    edges.push((Pos(i, j), Direction::Left));
                    edges.push((Pos(i, j), Direction::Right));
                    edges.push((Pos(i, j), Direction::Down));
                } else if i == self.tiles.len() - 1 {
                    edges.push((Pos(i, j), Direction::Left));
                    edges.push((Pos(i, j), Direction::Right));
                    edges.push((Pos(i, j), Direction::Up));
                } else if j == 0 {
                    edges.push((Pos(i, j), Direction::Right));
                    edges.push((Pos(i, j), Direction::Up));
                    edges.push((Pos(i, j), Direction::Down));
                } else if j == row.len() - 1 {
                    edges.push((Pos(i, j), Direction::Left));
                    edges.push((Pos(i, j), Direction::Up));
                    edges.push((Pos(i, j), Direction::Down));
                }
            }
        }

        edges
    }

    fn print_energized(&self, energized: &[Pos], current_pos: Pos, current_direction: Direction, queue: &VecDeque<(Pos, Direction)>, debug: bool) {
        print!("\x1B[2J");
        for (i, row) in self.tiles.iter().enumerate() {
            if debug {
                for (j, tile) in row.iter().enumerate() {
                    if current_pos == Pos(i, j) {
                        print!("\x1B[41m{}\x1B[0m", tile);
                        continue;
                    } else {
                        print!("{}", tile);
                    }
                }

                print!("   ");
            }

            for (j, tile) in row.iter().enumerate() {
                if energized.contains(&Pos(i, j)) {
                    if current_pos == Pos(i, j) {
                        print!("\x1B[41m{}\x1B[0m", tile);
                        continue;
                    } else {
                        match tile {
                            Tile::Empty => print!("\x1B[1;33m#\x1B[0m"),
                            _ => print!("\x1B[33m{}\x1B[0m", tile),
                        }
                    }
                } else {
                    print!("{}", tile);
                }
            }

            if debug {
                if i == 0 {
                    print!("   ");
                    print!("Position: {}", current_pos);
                }

                if i == 1 {
                    print!("   ");
                    print!("Current tile: {}", self.tiles[current_pos.0][current_pos.1]);
                }

                if i == 2 {
                    print!("   ");
                    print!("Direction: {}", current_direction);
                }

                if i == 3 {
                    print!("   ");
                    if !queue.is_empty() {
                        print!("Next in queue: ");
                        for (i, (pos, direction)) in queue.iter().enumerate() {
                            if i > 0 {
                                print!(", ");
                            }
                            print!("{} ({}) to {}", pos, self.tiles[pos.0][pos.1], direction);
                        }
                    }
                }
            }

            println!();
        }
    }

    fn energized(&self, start: (Pos, Direction), step: bool, show_debug_information: bool) -> Vec<Pos> {
        let mut already_visited = HashSet::new();
        let mut energized = Vec::new();
        let mut queue = VecDeque::new();
        queue.push_back(start);

        while let Some((p, direction)) = queue.pop_front() {
            if !energized.contains(&p) {
                energized.push(p);
            }

            if step {
                self.print_energized(&energized, p, direction, &queue, show_debug_information);

                let mut input = String::new();
                std::io::stdin().read_line(&mut input).unwrap();
            }

            match (self.tiles[p.0][p.1], direction) {
                (Tile::Empty, Direction::Up)
                | (Tile::VerticalMirror, Direction::Up)
                | (Tile::ForwardSlashMirror, Direction::Right)
                | (Tile::BackwardSlashMirror, Direction::Left) => {
                    if p.0 > 0 {
                        queue.push_back((Pos(p.0 - 1, p.1), Direction::Up));
                        continue;
                    }
                }
                (Tile::Empty, Direction::Down)
                | (Tile::VerticalMirror, Direction::Down)
                | (Tile::ForwardSlashMirror, Direction::Left)
                | (Tile::BackwardSlashMirror, Direction::Right) => {
                    if p.0 < self.tiles.len() - 1 {
                        queue.push_back((Pos(p.0 + 1, p.1), Direction::Down));
                        continue;
                    }
                }
                (Tile::Empty, Direction::Left)
                | (Tile::HorizontalMirror, Direction::Left)
                | (Tile::ForwardSlashMirror, Direction::Down)
                | (Tile::BackwardSlashMirror, Direction::Up) => {
                    if p.1 > 0 {
                        queue.push_back((Pos(p.0, p.1 - 1), Direction::Left));
                        continue;
                    }
                }
                (Tile::Empty, Direction::Right)
                | (Tile::HorizontalMirror, Direction::Right)
                | (Tile::ForwardSlashMirror, Direction::Up)
                | (Tile::BackwardSlashMirror, Direction::Down) => {
                    if p.1 < self.tiles[p.0].len() - 1 {
                        queue.push_back((Pos(p.0, p.1 + 1), Direction::Right));
                        continue;
                    }
                }
                (Tile::HorizontalMirror, Direction::Up)
                | (Tile::HorizontalMirror, Direction::Down) => {
                    if p.1 > 0 && already_visited.insert((p, Pos(p.0, p.1 - 1))) {
                        queue.push_back((Pos(p.0, p.1 - 1), Direction::Left));
                    }
                    if p.1 < self.tiles[p.0].len() - 1 && already_visited.insert((p, Pos(p.0, p.1 + 1))) {
                        queue.push_back((Pos(p.0, p.1 + 1), Direction::Right));
                    }
                    continue;
                }
                (Tile::VerticalMirror, Direction::Left)
                | (Tile::VerticalMirror, Direction::Right) => {
                    if p.0 > 0 && already_visited.insert((p, Pos(p.0 - 1, p.1))) {
                        queue.push_back((Pos(p.0 - 1, p.1), Direction::Up));
                    }
                    if p.0 < self.tiles.len() - 1 && already_visited.insert((p, Pos(p.0 + 1, p.1))) {
                        queue.push_back((Pos(p.0 + 1, p.1), Direction::Down));
                    }
                    continue;
                }
            }
        }

        energized
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = Grid::from_str(input).ok()?;
    // okay so the visualization and debug info was too tedious to remove
    // so it stays in
    // feel free to turn it on!
    let energized = grid.energized((Pos(0, 0), Direction::Right), false, false);

    Some(energized.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = Grid::from_str(input).ok()?;

    grid
        .edges_with_directions()
        .par_iter()
        .map(|(pos, direction)| {
            let energized = grid.energized((*pos, *direction), false, false);
            energized.len() as u32
        })
        .max()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(51));
    }
}
