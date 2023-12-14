advent_of_code::solution!(11);

fn give_me_the_galaxies(input: &str, mut expansion: u32) -> Vec<Pos> {
    expansion -= 1;

    let lines = input
        .lines()
        .collect::<Vec<_>>();
    let mut grid: Vec<Vec<char>> = vec![vec!['.'; lines[0].len()]; lines.len()];
    let mut galaxies = Vec::new();
    lines.iter().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, char)| {
            if char == '#' {
                grid[y][x] = '#';
                galaxies.push(Pos(x, y));
            }
        });
    });

    let empty_rows = grid
        .clone()
        .into_iter()
        .enumerate()
        .filter_map(|(y, line)| {
            if line.iter().any(|c| c == &'#') {
                None
            } else {
                Some(y)
            }
        })
        .collect::<Vec<_>>();
    
    let mut empty_cols = Vec::new();
    for x in 0..grid[0].len() {
        let mut empty_col = true;
        for g in &grid {
            if g[x] == '#' {
                empty_col = false;
                break;
            }
        }

        if empty_col {
            empty_cols.push(x);
        }
    }

    galaxies
        .iter_mut()
        .for_each(|g| {
            let Pos(x, y) = g;
            let rows_added = empty_rows.iter().filter(|r| **r < *y).count() * expansion as usize;
            let cols_added = empty_cols.iter().filter(|c| **c < *x).count() * expansion as usize;

            *g = Pos(*x + cols_added, *y + rows_added);
        });

    galaxies
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Pos(usize, usize);

impl Pos {
    fn manhattan(&self, other: &Pos) -> u64 {
        self.0.abs_diff(other.0) as u64 + self.1.abs_diff(other.1) as u64
    }
}

fn sums_of_distances_with_expansion(input: &str, expansion: u32) -> u64 {
    let galaxies = give_me_the_galaxies(input, expansion);

    galaxies
        .iter()
        .enumerate()
        .fold(0, |mut acc, (i, galaxy)| {
            for next in &galaxies[i + 1..] {
                acc += galaxy.manhattan(next);
            }

            acc
        })
}

pub fn part_one(input: &str) -> Option<u64> {
    sums_of_distances_with_expansion(input, 2).into()
}

pub fn part_two(input: &str) -> Option<u64> {
    sums_of_distances_with_expansion(input, 1_000_000).into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(374));
    }

    #[test]
    fn test_part_two_one() {
        let result = sums_of_distances_with_expansion(&advent_of_code::template::read_file("examples", DAY), 10);
        assert_eq!(result, 1030);
    }

    #[test]
    fn test_part_two_two() {
        let result = sums_of_distances_with_expansion(&advent_of_code::template::read_file("examples", DAY), 100);
        assert_eq!(result, 8410);
    }
}
