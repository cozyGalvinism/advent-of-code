use std::collections::{HashSet, VecDeque};

advent_of_code::solution!(10);

type Pos = (usize, usize);

fn parse_grid(input_str: &str) -> Vec<Vec<char>> {
    let mut grid: Vec<Vec<char>> = input_str
        .lines()
        .collect::<Vec<_>>()
        .iter()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| ".".to_owned() + line + ".")
        .map(|line| line.chars().collect())
        .collect();

    grid.insert(0, std::iter::repeat('.').take(grid[0].len()).collect());
    grid.push(std::iter::repeat('.').take(grid[0].len()).collect());

    grid
}

fn find_s(grid: &[Vec<char>]) -> Option<Pos> {
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] == 'S' {
                return Some((x, y));
            }
        }
    }

    None
}

fn connects(grid: &[Vec<char>], (x, y): Pos) -> Option<(Pos, Pos)> {
    if y >= grid.len() || x >= grid[0].len() {
        return None;
    }

    let item = grid[y][x];
    match item {
        '|' => Some(((x, y.wrapping_sub(1)), (x, y + 1))),
        '-' => Some(((x.wrapping_sub(1), y), (x + 1, y))),
        'L' => Some(((x, y.wrapping_sub(1)), (x + 1, y))),
        'J' => Some(((x.wrapping_sub(1), y), (x, y.wrapping_sub(1)))),
        '7' => Some(((x.wrapping_sub(1), y), (x, y + 1))),
        'F' => Some(((x, y + 1), (x + 1, y))),
        _ => None,
    }
}

fn find_connecting_pipes(grid: &[Vec<char>]) -> Vec<Pos> {
    let s = find_s(grid).unwrap();
    let mut curr = s;
    let neighbors = vec![
        (curr.0.wrapping_sub(1), curr.1),
        (curr.0 + 1, curr.1),
        (curr.0, curr.1.wrapping_sub(1)),
        (curr.0, curr.1 + 1),
    ];

    for n in neighbors {
        if n == s {
            continue;
        }

        let connect = connects(grid, n);
        if let Some((c1, c2)) = connect {
            if c1 == curr || c2 == curr {
                curr = n;
                break;
            }
        };
    }

    let mut pipes: Vec<Pos> = vec![s];
    while grid[curr.1][curr.0] != 'S' {
        let (c1, c2) = connects(grid, curr).unwrap();
        let next = if c1 == *pipes.last().unwrap() { c2 } else { c1 };
        pipes.push(curr);
        curr = next;
    }

    pipes
}

fn mark_grid(
    grid: Vec<Vec<char>>,
    curr: (usize, usize),
    pipes: &HashSet<(usize, usize)>,
) -> Vec<Vec<char>> {
    if curr.1 >= grid.len() || curr.0 >= grid[0].len() {
        return grid;
    }

    if grid[curr.1][curr.0] == 'X' {
        return grid;
    }

    if pipes.contains(&curr) {
        return grid;
    }

    let neighbors = vec![
        (curr.0.wrapping_sub(1), curr.1),
        (curr.0 + 1, curr.1),
        (curr.0, curr.1.wrapping_sub(1)),
        (curr.0, curr.1 + 1),
    ];

    let mut g = grid;
    g[curr.1][curr.0] = 'X';
    
    for n in neighbors {
        g = mark_grid(g, n, pipes);
    }

    g
}

fn count_chars(grid: &[Vec<char>], c: char) -> usize {
    grid.iter()
        .map(|line| line.iter().filter(|&&x| x == c).count())
        .sum()
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = parse_grid(input);
    let pipes = find_connecting_pipes(&grid);

    Some(pipes.len() as u32 / 2_u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = parse_grid(input);
    let s = find_s(&grid)?;
    let pipes = find_connecting_pipes(&grid);
    let pipe_set = pipes.iter().cloned().collect::<HashSet<_>>();

    let mut marked_grid = grid;
    let mut prev = (s.0 as i64, s.1 as i64);
    let mut points = Vec::new();

    for segment in pipes {
        let curr = (segment.0 as i64, segment.1 as i64);
        match (curr.0 - prev.0, curr.1 - prev.1) {
            (0, 1) => {
                points.push((segment.0.wrapping_sub(1), segment.1.wrapping_sub(1)));
                points.push((segment.0.wrapping_sub(1), segment.1));
            }
            (1, 0) => {
                points.push((segment.0, segment.1 + 1));
                points.push((segment.0.wrapping_sub(1), segment.1 + 1));
            }
            (0, -1) => {
                points.push((segment.0 + 1, segment.1));
                points.push((segment.0 + 1, segment.1 + 1));
            }
            (-1, 0) => {
                points.push((segment.0, segment.1.wrapping_sub(1)));
                points.push((segment.0 + 1, segment.1.wrapping_sub(1)));
            }
            _ => (),
        }
        prev = curr;
    }

    for p in points {
        marked_grid = mark_grid(marked_grid, p, &pipe_set);
    }

    let x_count = count_chars(&marked_grid, 'X');
    if marked_grid[0][0] == 'X' {
        let total = marked_grid.len() * marked_grid[0].len();
        Some((total - x_count - pipe_set.len()) as u32)
    } else {
        Some(x_count as u32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(10));
    }
}
