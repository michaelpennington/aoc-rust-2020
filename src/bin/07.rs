use std::collections::{HashMap, HashSet};

advent_of_code::solution!(7);

#[derive(Debug)]
struct Bags<'a> {
    rules: HashMap<&'a str, Vec<(&'a str, u32)>>,
    contained_by: HashMap<&'a str, Vec<&'a str>>,
}

impl<'a> Bags<'a> {
    fn parse_from_str(s: &'a str) -> Self {
        let mut rules = HashMap::with_capacity(s.lines().count());
        let mut contained_by: HashMap<_, Vec<_>> = HashMap::new();
        for l in s.lines() {
            let (bag, contents) = l.split_once(" bags contain ").unwrap();
            let mut contains = Vec::new();
            for content in contents.split(", ") {
                let (num, rest) = content.split_once(' ').unwrap();
                let rest = rest
                    .trim_end_matches('.')
                    .trim_end_matches("bag")
                    .trim_end_matches("bags")
                    .trim();
                if let Ok(num) = num.parse() {
                    contains.push((rest, num));
                    contained_by.entry(rest).or_default().push(bag);
                }
            }
            rules.insert(bag, contains);
        }
        Self {
            rules,
            contained_by,
        }
    }

    fn num_containing(&self, bag: &'a str) -> u32 {
        let mut containing: HashSet<&'a str> = HashSet::new();
        let mut bags = self.contained_by.get(bag).unwrap().clone();
        containing.extend(&bags);
        loop {
            let new_bags = bags
                .iter()
                .flat_map(|b| self.contained_by.get(b))
                .flatten()
                .copied()
                .collect::<Vec<_>>();
            containing.extend(&new_bags);
            if new_bags.is_empty() {
                return containing.len() as u32;
            }
            bags = new_bags;
        }
    }

    fn num_contained(&self, bag: &'a str) -> u32 {
        if let Some(children) = self.rules.get(bag) {
            children
                .iter()
                .map(|(child, num)| num * (self.num_contained(child) + 1))
                .sum()
        } else {
            0
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let bags = Bags::parse_from_str(input);
    Some(bags.num_containing("shiny gold"))
}

pub fn part_two(input: &str) -> Option<u32> {
    let bags = Bags::parse_from_str(input);
    Some(bags.num_contained("shiny gold"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(32));
    }

    #[test]
    fn test_part_two_one() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(126));
    }
}
