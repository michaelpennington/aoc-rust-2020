use std::{
    collections::{HashMap, HashSet},
    ops::RangeInclusive,
};

use anyhow::bail;

advent_of_code::solution!(16);

#[derive(Debug)]
struct TicketReqs<'a> {
    requirements: HashMap<&'a str, [RangeInclusive<u32>; 2]>,
}

impl<'a> TicketReqs<'a> {
    fn new(s: &'a str) -> anyhow::Result<Self> {
        let mut requirements = HashMap::new();
        for line in s.lines() {
            let Some((req, (v1_from, v1_to), (v2_from, v2_to))) = line
                .split_once(": ")
                .and_then(|(req, vals)| vals.split_once(" or ").map(|(v1, v2)| (req, v1, v2)))
                .and_then(|(req, v1, v2)| {
                    v1.split_once('-')
                        .zip(v2.split_once('-'))
                        .map(|(v1, v2)| (req, v1, v2))
                })
            else {
                bail!("Invalid req {line}")
            };
            let (v1_from, v1_to, v2_from, v2_to) = (
                v1_from.parse()?,
                v1_to.parse()?,
                v2_from.parse()?,
                v2_to.parse()?,
            );
            requirements.insert(req, [v1_from..=v1_to, v2_from..=v2_to]);
        }
        Ok(Self { requirements })
    }

    fn is_valid(&self, value: &u32) -> bool {
        self.requirements
            .values()
            .any(|req| req[0].contains(value) || req[1].contains(value))
    }

    fn is_not_valid_for(&'a self, value: &'a u32) -> impl Iterator<Item = &'a str> {
        self.requirements
            .iter()
            .filter(|(_, req)| !req[0].contains(value) && !req[1].contains(value))
            .map(|(v, _)| *v)
    }

    fn candidates(&'a self) -> impl Iterator<Item = &'a str> {
        self.requirements.keys().copied()
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut pts = input.split("\n\n");
    let reqs = pts.next().unwrap();
    let tickets = pts.nth(1).unwrap();
    let reqs = TicketReqs::new(reqs).unwrap();
    Some(
        tickets
            .lines()
            .skip(1)
            .flat_map(|l| l.split(','))
            .map(|n| n.parse().unwrap())
            .filter(|num| !reqs.is_valid(num))
            .sum(),
    )
}

#[derive(Debug)]
struct Candidates<'a>(Vec<HashSet<&'a str>>);

impl<'a> Candidates<'a> {
    fn new(candidates: HashSet<&'a str>, num: usize) -> Self {
        Self(vec![candidates; num])
    }

    fn remove(&mut self, cand: &str, index: usize) {
        self.0[index].remove(cand);
        while self.dedup() {}
    }

    fn dedup(&mut self) -> bool {
        let to_remove = self
            .0
            .iter()
            .enumerate()
            .filter(|(_, c)| c.len() == 1)
            .map(|(i, c)| (i, *c.iter().next().unwrap()))
            .collect::<HashMap<_, _>>();
        let mut removed = false;
        for (index, cand) in to_remove {
            removed |= self
                .0
                .iter_mut()
                .enumerate()
                .filter(|(i, _)| *i != index)
                .any(|(_, c)| c.remove(cand));
        }
        removed
    }
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut pts = input.split("\n\n");
    let reqs = pts.next().unwrap();
    let my_ticket = pts
        .next()
        .unwrap()
        .lines()
        .nth(1)
        .unwrap()
        .split(',')
        .map(|n| n.parse::<u32>().unwrap())
        .collect::<Vec<_>>();
    let reqs = TicketReqs::new(reqs).unwrap();
    let mut candidates = Candidates::new(reqs.candidates().collect(), my_ticket.len());
    for line in pts.next().unwrap().lines().skip(1) {
        for (index, cand) in line
            .split(',')
            .map(|n| n.parse::<u32>().unwrap())
            .enumerate()
        {
            if reqs.is_valid(&cand) {
                for candidate in reqs.is_not_valid_for(&cand) {
                    candidates.remove(candidate, index);
                }
            }
        }
    }
    Some(
        candidates
            .0
            .into_iter()
            .enumerate()
            .filter(|(_, c)| c.iter().any(|c| c.contains("departure")))
            .map(|(i, _)| my_ticket[i] as u64)
            .product(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71));
    }
}
