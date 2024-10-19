use std::{collections::HashSet, str::FromStr};

use advent_of_code::util::point::Pt3;

advent_of_code::solution!(17);

#[derive(Clone, Debug)]
struct Conway3D {
    cubes: HashSet<Pt3<i32>>,
    extents: [(i32, i32); 3],
}

impl FromStr for Conway3D {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let z = 0;
        let mut cubes = HashSet::new();
        let mut x_extent = (0, 0);
        let mut y_extent = (0, 0);
        let z_extent = (0, 0);
        for (y, line) in s.lines().enumerate() {
            let y = y as i32;
            for (x, c) in line.char_indices() {
                let x = x as i32;
                if c == '#' {
                    x_extent.0 = x_extent.0.min(x);
                    x_extent.1 = x_extent.1.max(x);
                    y_extent.0 = y_extent.0.min(y);
                    y_extent.1 = y_extent.1.max(y);
                    cubes.insert(Pt3 { x, y, z });
                }
            }
        }
        Ok(Self {
            cubes,
            extents: [x_extent, y_extent, z_extent],
        })
    }
}

impl Conway3D {
    fn generation(&self) -> Self {
        let mut out = Self {
            cubes: HashSet::with_capacity(self.cubes.len()),
            extents: self.extents,
        };
        for x in self.extents[0].0 - 1..=self.extents[0].1 + 1 {
            for y in self.extents[1].0 - 1..=self.extents[1].1 + 1 {
                for z in self.extents[2].0 - 1..=self.extents[2].1 + 1 {
                    let pt = Pt3 { x, y, z };
                    match (self.cubes.contains(&pt), self.num_neighbors(&pt)) {
                        (true, 2 | 3) | (false, 3) => {
                            out.cubes.insert(pt);
                            out.extents[0].0 = out.extents[0].0.min(x);
                            out.extents[0].1 = out.extents[0].1.max(x);
                            out.extents[1].0 = out.extents[1].0.min(y);
                            out.extents[1].1 = out.extents[1].1.max(y);
                            out.extents[2].0 = out.extents[2].0.min(z);
                            out.extents[2].1 = out.extents[2].1.max(z);
                        }
                        _ => {}
                    }
                }
            }
        }
        out
    }

    fn num_neighbors(&self, pt: &Pt3<i32>) -> usize {
        (pt.x - 1..=pt.x + 1)
            .flat_map(|x| {
                (pt.y - 1..=pt.y + 1)
                    .flat_map(move |y| (pt.z - 1..=pt.z + 1).map(move |z| Pt3 { x, y, z }))
            })
            .filter(|p| p != pt)
            .filter(|p| self.cubes.contains(p))
            .count()
    }

    fn count(&self) -> usize {
        self.cubes.len()
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut conway = input.parse::<Conway3D>().unwrap();
    for _ in 0..6 {
        conway = conway.generation();
    }

    Some(conway.count())
}

#[derive(Clone, Debug)]
struct Conway4D {
    cubes: HashSet<[i32; 4]>,
    extents: [(i32, i32); 4],
}

impl FromStr for Conway4D {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let z = 0;
        let mut cubes = HashSet::new();
        let mut x_extent = (0, 0);
        let mut y_extent = (0, 0);
        let z_extent = (0, 0);
        let w = 0;
        let w_extent = (0, 0);
        for (y, line) in s.lines().enumerate() {
            let y = y as i32;
            for (x, c) in line.char_indices() {
                let x = x as i32;
                if c == '#' {
                    x_extent.0 = x_extent.0.min(x);
                    x_extent.1 = x_extent.1.max(x);
                    y_extent.0 = y_extent.0.min(y);
                    y_extent.1 = y_extent.1.max(y);
                    cubes.insert([x, y, z, w]);
                }
            }
        }
        Ok(Self {
            cubes,
            extents: [x_extent, y_extent, z_extent, w_extent],
        })
    }
}

impl Conway4D {
    fn generation(&self) -> Self {
        let mut out = Self {
            cubes: HashSet::with_capacity(self.cubes.len()),
            extents: self.extents,
        };
        for x in self.extents[0].0 - 1..=self.extents[0].1 + 1 {
            for y in self.extents[1].0 - 1..=self.extents[1].1 + 1 {
                for z in self.extents[2].0 - 1..=self.extents[2].1 + 1 {
                    for w in self.extents[3].0 - 1..=self.extents[3].1 + 1 {
                        let pt = [x, y, z, w];
                        match (self.cubes.contains(&pt), self.num_neighbors(&pt)) {
                            (true, 2 | 3) | (false, 3) => {
                                out.cubes.insert(pt);
                                out.extents[0].0 = out.extents[0].0.min(x);
                                out.extents[0].1 = out.extents[0].1.max(x);
                                out.extents[1].0 = out.extents[1].0.min(y);
                                out.extents[1].1 = out.extents[1].1.max(y);
                                out.extents[2].0 = out.extents[2].0.min(z);
                                out.extents[2].1 = out.extents[2].1.max(z);
                                out.extents[3].0 = out.extents[3].0.min(w);
                                out.extents[3].1 = out.extents[3].1.max(w);
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
        out
    }

    fn num_neighbors(&self, pt: &[i32; 4]) -> usize {
        (pt[0] - 1..=pt[0] + 1)
            .flat_map(|x| {
                (pt[1] - 1..=pt[1] + 1).flat_map(move |y| {
                    (pt[2] - 1..=pt[2] + 1)
                        .flat_map(move |z| (pt[3] - 1..=pt[3] + 1).map(move |w| [x, y, z, w]))
                })
            })
            .filter(|p| p != pt)
            .filter(|p| self.cubes.contains(p))
            .count()
    }

    fn count(&self) -> usize {
        self.cubes.len()
    }
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut conway = input.parse::<Conway4D>().unwrap();
    for _ in 0..6 {
        conway = conway.generation();
    }

    Some(conway.count())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(112));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(848));
    }
}
