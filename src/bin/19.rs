use std::{
    collections::{HashMap, VecDeque},
    str::FromStr,
};

advent_of_code::solution!(19);

#[derive(Debug)]
struct Rules {
    chars: HashMap<u8, char>,
    rules: HashMap<u8, Vec<Vec<u8>>>,
}

impl Rules {
    fn test(&self, s: &str, seq: &mut VecDeque<u8>) -> bool {
        if s.is_empty() || seq.is_empty() {
            s.is_empty() && seq.is_empty()
        } else if let Some(c) = self.chars.get(seq.front().unwrap()) {
            if *c == s.chars().next().unwrap() {
                seq.pop_front();
                self.test(&s[1..], seq)
            } else {
                false
            }
        } else {
            let rules = self.rules.get(seq.front().unwrap()).unwrap();
            rules.iter().any(|rule| {
                let mut seq = seq.clone();
                seq.pop_front();
                for &r in rule.iter().rev() {
                    seq.push_front(r)
                }
                self.test(s, &mut seq)
            })
        }
    }

    fn update(&mut self) {
        self.rules.insert(8, vec![vec![42], vec![42, 8]]);
        self.rules.insert(11, vec![vec![42, 31], vec![42, 11, 31]]);
    }

    fn matches_rule(&self, rule: u8, s: &str) -> bool {
        let mut seq = self.rules[&rule].first().unwrap().iter().copied().collect();
        self.test(s, &mut seq)
    }
}

impl FromStr for Rules {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = HashMap::new();
        let mut rules = HashMap::new();
        for line in s.lines() {
            let (rule_no, matches) = line.split_once(": ").unwrap();
            let rule_no = rule_no.parse()?;
            if matches.contains('"') {
                let c = matches.trim_matches('"').chars().next().unwrap();
                chars.insert(rule_no, c);
            } else {
                let new_rules = matches
                    .split(" | ")
                    .map(|rule| {
                        rule.split_whitespace()
                            .map(|n| n.parse().unwrap())
                            .collect()
                    })
                    .collect();
                rules.insert(rule_no, new_rules);
            }
        }
        Ok(Self { chars, rules })
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let (rules, strings) = input.split_once("\n\n").unwrap();
    let rules = rules.parse::<Rules>().unwrap();
    Some(strings.lines().filter(|l| rules.matches_rule(0, l)).count())
}

pub fn part_two(input: &str) -> Option<usize> {
    let (rules, strings) = input.split_once("\n\n").unwrap();
    let mut rules = rules.parse::<Rules>().unwrap();
    rules.update();
    Some(strings.lines().filter(|l| rules.matches_rule(0, l)).count())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(12));
    }
}
