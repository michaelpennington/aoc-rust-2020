use std::collections::{HashMap, HashSet};

advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<usize> {
    Some(
        input
            .split("\n\n")
            .map(|s| {
                s.lines()
                    .flat_map(|l| l.chars())
                    .collect::<HashSet<_>>()
                    .len()
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(
        input
            .split("\n\n")
            .map(|g| {
                let mut map = HashMap::new();
                let mut num_lines = 0;
                for line in g.lines() {
                    for c in line.chars() {
                        *map.entry(c).or_insert(0) += 1;
                    }
                    num_lines += 1;
                }
                map.into_values().filter(|v| *v == num_lines).count()
            })
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
