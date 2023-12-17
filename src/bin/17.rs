use std::{collections::{BinaryHeap, HashSet}, fmt::{Debug, Display}};

advent_of_code::solution!(17);

// trying out complex numbers for the first time
type Pos = num::Complex<isize>;
type Dir = num::Complex<isize>;
type Map = std::collections::HashMap<Pos, u32>;

#[derive(Clone, Copy)]
struct ShortDirections;
#[derive(Clone, Copy)]
struct LongDirections;

impl CustomFormat<ShortDirections> for Dir {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>, _: ShortDirections) -> std::fmt::Result {
        match self {
            Dir { re: 0, im: 1 } => write!(fmt, "S"),
            Dir { re: 0, im: -1 } => write!(fmt, "N"),
            Dir { re: 1, im: 0 } => write!(fmt, "E"),
            Dir { re: -1, im: 0 } => write!(fmt, "W"),
            _ => panic!("invalid direction"),
        }
    }
}

impl CustomFormat<LongDirections> for Dir {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>, _: LongDirections) -> std::fmt::Result {
        match self {
            Dir { re: 0, im: 1 } => write!(fmt, "South"),
            Dir { re: 0, im: -1 } => write!(fmt, "North"),
            Dir { re: 1, im: 0 } => write!(fmt, "East"),
            Dir { re: -1, im: 0 } => write!(fmt, "West"),
            _ => panic!("invalid direction"),
        }
    }
}

trait CustomFormat<F: Copy> {
    fn custom_format(&self, format_type: F) -> CustomFormatWrapper<'_, F, Self> {
        CustomFormatWrapper(format_type, self)
    }

    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>, format_type: F) -> std::fmt::Result;
}

struct CustomFormatWrapper<'a, F: Copy, T: CustomFormat<F> + ?Sized>(F, &'a T);

impl<'a, F: Copy, T: CustomFormat<F>> Debug for CustomFormatWrapper<'a, F, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <T as CustomFormat<F>>::fmt(self.1, f, self.0)
    }
}

impl<'a, F: Copy, T: CustomFormat<F>> Display for CustomFormatWrapper<'a, F, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <T as CustomFormat<F>>::fmt(self.1, f, self.0)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Crucible(Pos, Dir, u8);

impl Debug for Crucible {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Crucible")
            .field("pos", &self.0)
            .field("dir", &self.1.custom_format(ShortDirections))
            .field("straight", &self.2)
            .finish()
    }
}

type Rule = fn(&Crucible) -> bool;

fn parse(input: &str) -> Map {
    let mut map = Map::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            map.insert(Pos::new(x as isize, y as isize), c.to_digit(10).unwrap());
        }
    }
    map
}

impl Crucible {
    fn moves(&self, straight: Rule, turn: Rule) -> Vec<Crucible> {
        let mut moves = vec![];

        if (straight)(self) {
            moves.push(Crucible(
                self.0 + self.1,
                self.1,
                self.2 + 1
            ));
        }

        if (turn)(self) {
            let dir = self.1 * Dir::new(0, 1);
            moves.push(Crucible(self.0 + dir, dir, 1));
            moves.push(Crucible(self.0 - dir, -dir, 1));
        }

        moves
    }
}

struct PrioQItem(Crucible, u32);

impl PartialEq for PrioQItem {
    fn eq(&self, other: &Self) -> bool {
        self.1 == other.1
    }
}

impl Eq for PrioQItem {}

impl PartialOrd for PrioQItem {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PrioQItem {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.1.cmp(&self.1)
    }
}

fn heatloss(input: &str, straight: Rule, turn: Rule) -> u32 {
    let map = parse(input);
    let goal = map.keys().max_by(|a, b| {
        (a.im + a.re)
            .cmp(&(b.im + b.re))
    })
    .unwrap();

    let mut queue = BinaryHeap::new();
    queue.push(PrioQItem(Crucible(
        Pos::new(0, 0),
        Dir::new(1, 0),
        0,
    ), 0));
    queue.push(PrioQItem(Crucible(
        Pos::new(0, 0),
        Dir::new(0, 1),
        0,
    ), 0));
    let mut seen = HashSet::new();

    while let Some(PrioQItem(crucible, heatloss)) = queue.pop() {
        if crucible.0 == *goal && (turn)(&crucible) {
            return heatloss;
        }

        let moves = crucible.moves(straight, turn);
        for move_ in moves {
            if map.contains_key(&move_.0) && !seen.contains(&move_) {
                seen.insert(move_);
                let map_heatloss = map.get(&move_.0).unwrap();
                queue.push(PrioQItem(move_, heatloss + map_heatloss));
            }
        }
    }

    panic!("no path found");
}

pub fn part_one(input: &str) -> Option<u32> {
    heatloss(input, |c| c.2 < 3, |_| true)
    .into()
}

pub fn part_two(input: &str) -> Option<u32> {
    heatloss(input, |c| c.2 < 10, |c| c.2 >= 4)
    .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(102));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(94));
    }
}
