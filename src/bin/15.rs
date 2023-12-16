use std::num::Wrapping;

advent_of_code::solution!(15);

fn hash(s: &str) -> u32 {
    let mut hash = Wrapping::<u16>(0);
    for c in s.chars() {
        let ascii_value = c as u16;
        hash += ascii_value;
        hash *= 17;
    }
    hash %= 256;
    hash.0 as u32
}

pub fn part_one(input: &str) -> Option<u32> {
    input
        .split(',')
        .map(|s| hash(s.trim()))
        .sum::<u32>()
        .into()
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut boxes: Vec<Vec<(String, u32)>> = Vec::with_capacity(256);
    for _ in 0..256 {
        boxes.push(Vec::new());
    }

    for operation in input.split(',') {
        let operation = operation.trim();

        if operation.contains('=') {
            let mut operation = operation.split('=');
            let label = operation.next().unwrap();
            let hash = hash(label) as usize;
            let focal_strength = operation.next().unwrap().parse::<u32>().unwrap();
            if let Some(box_) = boxes.get_mut(hash) {
                // check if box already contains lens with label
                if let Some(pos) = box_.iter().position(|(l, _)| l == label) {
                    box_[pos] = (label.to_string(), focal_strength);
                } else {
                    box_.push((label.to_string(), focal_strength));
                }
            }
        } else if operation.contains('-') {
            let mut operation = operation.split('-');
            let label = operation.next().unwrap();
            let hash = hash(label) as usize;

            if let Some(box_) = boxes.get_mut(hash) {
                if let Some(pos) = box_.iter().position(|(l, _)| l == label) {
                    box_.remove(pos);
                }
            }
        }
    }

    boxes
        .iter()
        .enumerate()
        .map(|(bi, box_)| {
            let bi = bi + 1;
            box_
                .iter()
                .enumerate()
                .map(|(li, (_, focal))| {
                    let li = li + 1;
                    focal * (bi as u32) * (li as u32)
                })
                .sum::<u32>()
        })
        .sum::<u32>()
        .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hashing() {
        assert_eq!(hash("HASH"), 52);
    }

    #[test]
    fn test_hashing_alt() {
        assert_eq!(hash("HASH"), 52);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1320));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(145));
    }
}
