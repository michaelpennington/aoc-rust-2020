use std::{
    ops::{Index, IndexMut},
    str::FromStr,
};

use advent_of_code::util::point::Pt;
use anyhow::bail;

advent_of_code::solution!(11);

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
enum Space {
    Floor,
    Empty,
    Occupied,
}

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
struct Layout {
    spaces: Vec<Vec<Space>>,
    width: usize,
    height: usize,
}

impl FromStr for Layout {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let height = s.lines().count();
        let width = s.lines().next().unwrap().len();
        let mut spaces = Vec::with_capacity(height);
        for line in s.lines() {
            let mut new_line = Vec::with_capacity(width);
            for c in line.chars() {
                match c {
                    '.' => new_line.push(Space::Floor),
                    'L' => new_line.push(Space::Empty),
                    '#' => new_line.push(Space::Occupied),
                    _ => bail!("Unknown char {c}"),
                }
            }
            spaces.push(new_line);
        }
        Ok(Self {
            spaces,
            width,
            height,
        })
    }
}

const DIRS: [Pt<isize>; 8] = [
    Pt { x: -1, y: -1 },
    Pt { x: -1, y: 0 },
    Pt { x: -1, y: 1 },
    Pt { x: 0, y: -1 },
    Pt { x: 0, y: 1 },
    Pt { x: 1, y: -1 },
    Pt { x: 1, y: 0 },
    Pt { x: 1, y: 1 },
];

impl Layout {
    fn num_adj_occupied(&self, pt: Pt<usize>) -> usize {
        (pt.x.saturating_sub(1)..=(pt.x + 1).min(self.width - 1))
            .flat_map(|x| {
                (pt.y.saturating_sub(1)..=(pt.y + 1).min(self.height - 1)).map(move |y| Pt { x, y })
            })
            .filter(|&np| np != pt && self[np] == Space::Occupied)
            .count()
    }

    fn num_near_occupied(&self, pt: Pt<usize>) -> usize {
        let mut total = 0;
        for dir in DIRS {
            let mut new_pt = pt;
            while let Some(pt) = new_pt
                .checked_add_signed(&dir)
                .filter(|p| p.x < self.width && p.y < self.height)
            {
                match self[pt] {
                    Space::Floor => {}
                    Space::Empty => break,
                    Space::Occupied => {
                        total += 1;
                        break;
                    }
                }
                new_pt = pt;
            }
        }
        total
    }

    fn calc(&self) -> Self {
        let mut copy = self.clone();
        for pt in (0..self.height)
            .flat_map(|y| (0..self.width).map(move |x| Pt { x, y }))
            .filter(|&pt| self[pt] != Space::Floor)
        {
            match (self[pt], self.num_adj_occupied(pt)) {
                (Space::Empty, 0) => copy[pt] = Space::Occupied,
                (Space::Occupied, n) if n >= 4 => copy[pt] = Space::Empty,
                _ => {}
            }
        }
        copy
    }

    fn calc_v2(&self) -> Self {
        let mut copy = self.clone();
        for pt in (0..self.height)
            .flat_map(|y| (0..self.width).map(move |x| Pt { x, y }))
            .filter(|&pt| self[pt] != Space::Floor)
        {
            match (self[pt], self.num_near_occupied(pt)) {
                (Space::Empty, 0) => copy[pt] = Space::Occupied,
                (Space::Occupied, n) if n >= 5 => copy[pt] = Space::Empty,
                _ => {}
            }
        }
        copy
    }

    fn num_occupied(&self) -> usize {
        self.spaces
            .iter()
            .flatten()
            .filter(|&&s| s == Space::Occupied)
            .count()
    }
}

impl Index<Pt<usize>> for Layout {
    type Output = Space;

    fn index(&self, index: Pt<usize>) -> &Self::Output {
        &self.spaces[index.y][index.x]
    }
}

impl IndexMut<Pt<usize>> for Layout {
    fn index_mut(&mut self, index: Pt<usize>) -> &mut Self::Output {
        &mut self.spaces[index.y][index.x]
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut layout = input.parse::<Layout>().unwrap();
    loop {
        let new_layout = layout.calc();
        if layout == new_layout {
            return Some(layout.num_occupied());
        }
        layout = new_layout;
    }
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut layout = input.parse::<Layout>().unwrap();
    loop {
        let new_layout = layout.calc_v2();
        if layout == new_layout {
            return Some(layout.num_occupied());
        }
        layout = new_layout;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(37));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(26));
    }
}
