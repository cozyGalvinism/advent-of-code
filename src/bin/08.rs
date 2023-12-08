use std::fmt::Display;

use num::Integer;

advent_of_code::solution!(8);

#[derive(Clone, Debug)]
struct NodeVisitor {
    nodes: Vec<Node>,
    sequence: String,
}

struct NodeVisitorIterP1 {
    visitor: NodeVisitor,
    sequence_index: usize,
    vec_index: usize,
}

struct NodeVisitorIterP2<'a> {
    visitor: &'a mut NodeVisitor,
    sequence_index: usize,
    vec_indices: Vec<usize>,
}

impl NodeVisitor {
    fn from_input(input: &str) -> Self {
        let node_regex = regex::Regex::new(r"\(([A-Z0-9]{3}), ([A-Z0-9]{3})\)").unwrap();

        let mut lines = input.lines();
        let sequence = lines.next().unwrap().to_string();
        lines.next();
        let nodes = lines
            .map(|line| {
                let mut parts = line.split('=');
                let location = parts.next().unwrap().trim().to_string();
                let captures = node_regex.captures(parts.next().unwrap()).unwrap();
                let left = captures.get(1).unwrap().as_str().to_string();
                let right = captures.get(2).unwrap().as_str().to_string();

                Node {
                    location,
                    left,
                    right,
                }
            })
            .collect::<Vec<_>>();

        Self { nodes, sequence }
    }

    fn aaa_index(&self) -> usize {
        self.nodes
            .iter()
            .position(|node| node.location == "AAA")
            .unwrap()
    }

    fn a_indices(&self) -> Vec<usize> {
        // get all indices of nodes where location ends with "A"
        self.nodes
            .iter()
            .enumerate()
            .filter(|(_, node)| node.location.ends_with('A'))
            .map(|(i, _)| i)
            .collect::<Vec<_>>()
    }

    fn into_iter_p1(self) -> NodeVisitorIterP1 {
        NodeVisitorIterP1 {
            vec_index: self.aaa_index(),
            visitor: self,
            sequence_index: 0,
        }
    }

    fn into_iter_p2_lcm(self) -> Vec<NodeVisitorIterP1> {
        let mut iters = Vec::new();
        for i in self.a_indices().iter() {
            iters.push(NodeVisitorIterP1 {
                vec_index: *i,
                visitor: self.clone(),
                sequence_index: 0,
            });
        }

        iters
    }
}

impl Iterator for NodeVisitorIterP1 {
    type Item = Node;

    fn next(&mut self) -> Option<Self::Item> {
        let current_step = self
            .visitor
            .sequence
            .chars()
            .nth(self.sequence_index)
            .unwrap();
        if self.sequence_index == self.visitor.sequence.len() - 1 {
            self.sequence_index = 0;
        } else {
            self.sequence_index += 1;
        }

        let current_node = self.visitor.nodes.get(self.vec_index).unwrap();

        if current_node.location.ends_with('Z') {
            return None;
        }

        if current_step == 'L' {
            self.vec_index = self
                .visitor
                .nodes
                .iter()
                .position(|node| node.location == current_node.left)
                .unwrap();
        } else {
            self.vec_index = self
                .visitor
                .nodes
                .iter()
                .position(|node| node.location == current_node.right)
                .unwrap();
        }

        self.visitor.nodes.get(self.vec_index).cloned()
    }
}

impl<'a> Iterator for NodeVisitorIterP2<'a> {
    type Item = Vec<Node>;

    fn next(&mut self) -> Option<Self::Item> {
        let current_step = self
            .visitor
            .sequence
            .chars()
            .nth(self.sequence_index)
            .unwrap();
        if self.sequence_index == self.visitor.sequence.len() - 1 {
            self.sequence_index = 0;
        } else {
            self.sequence_index += 1;
        }

        if self
            .vec_indices
            .iter()
            .all(|&i| self.visitor.nodes.get(i).unwrap().location.ends_with('Z'))
        {
            return None;
        }

        let mut nodes = Vec::new();
        for i in 0..self.vec_indices.len() {
            let current_node = self.visitor.nodes.get(self.vec_indices[i]).unwrap();

            if current_step == 'L' {
                self.vec_indices[i] = self
                    .visitor
                    .nodes
                    .iter()
                    .position(|node| node.location == current_node.left)
                    .unwrap();
            } else {
                self.vec_indices[i] = self
                    .visitor
                    .nodes
                    .iter()
                    .position(|node| node.location == current_node.right)
                    .unwrap();
            }

            nodes.push(self.visitor.nodes.get(self.vec_indices[i]).unwrap().clone());
        }

        println!(
            "next seq: {}",
            self.visitor
                .sequence
                .chars()
                .nth(self.sequence_index)
                .unwrap()
        );

        Some(nodes)
    }
}

#[derive(Clone, Debug)]
struct Node {
    location: String,

    left: String,
    right: String,
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} <- {} -> {}", self.left, self.location, self.right)
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    NodeVisitor::from_input(input)
        .into_iter_p1()
        .enumerate()
        .map(|(i, _)| i as u32 + 1)
        .max()
}

pub fn part_two(input: &str) -> Option<u64> {
    NodeVisitor::from_input(input)
        .into_iter_p2_lcm()
        .into_iter()
        .map(|mut iter| {
            let mut i = 0_u64;
            for _ in iter.by_ref() {
                i += 1;
            }
            i
        })
        .reduce(|a, b| a.lcm(&b))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(2));
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        assert_eq!(result, Some(6));
    }
}
