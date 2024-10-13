use std::{collections::HashSet, str::FromStr};

use advent_of_code::util::point::Pt;

advent_of_code::solution!(3);

struct Forest {
    trees: HashSet<Pt<usize>>,
    width: usize,
    height: usize,
}

impl Forest {
    fn contains(&self, pt: &Pt<usize>) -> bool {
        assert!(pt.y < self.height);
        let mut pt = *pt;
        pt.x %= self.width;
        self.trees.contains(&pt)
    }
}

impl FromStr for Forest {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let height = s.lines().count();
        let width = s.lines().next().unwrap().len();
        let mut trees = HashSet::new();
        for (y, line) in s.lines().enumerate() {
            for (x, c) in line.char_indices() {
                if c == '#' {
                    trees.insert(Pt { x, y });
                }
            }
        }
        Ok(Self {
            height,
            width,
            trees,
        })
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let forest = input.parse::<Forest>().unwrap();
    let inc = Pt { x: 3, y: 1 };
    let mut loc = Pt { x: 0, y: 0 };
    let mut total = 0;
    while loc.y < forest.height {
        if forest.contains(&loc) {
            total += 1;
        }
        loc += inc;
    }
    Some(total)
}

pub fn part_two(input: &str) -> Option<usize> {
    let forest = input.parse::<Forest>().unwrap();
    let incs = [
        Pt { x: 1, y: 1 },
        Pt { x: 3, y: 1 },
        Pt { x: 5, y: 1 },
        Pt { x: 7, y: 1 },
        Pt { x: 1, y: 2 },
    ];
    Some(
        incs.into_iter()
            .map(|inc| {
                let mut loc = Pt { x: 0, y: 0 };
                let mut total = 0;
                while loc.y < forest.height {
                    if forest.contains(&loc) {
                        total += 1;
                    }
                    loc += inc;
                }
                total
            })
            .product(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(336));
    }
}
