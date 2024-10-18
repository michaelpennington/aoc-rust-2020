use std::{collections::HashMap, str::FromStr};

use anyhow::{anyhow, bail};

advent_of_code::solution!(14);

#[derive(Debug, Clone, Copy)]
struct BitMask {
    ones: u64,
    zeroes: u64,
}

impl Default for BitMask {
    fn default() -> Self {
        Self {
            ones: 0,
            zeroes: u64::MAX,
        }
    }
}

impl BitMask {
    fn mask(&self, a: &u64) -> u64 {
        (a | self.ones) & self.zeroes
    }
}

impl FromStr for BitMask {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ones_s = s.replace("X", "0");
        let zeroes_s = s.replace("X", "1");
        let ones = u64::from_str_radix(&ones_s, 2)?;
        let zeroes = u64::from_str_radix(&zeroes_s, 2)?;
        Ok(Self { ones, zeroes })
    }
}

#[derive(Debug, Default)]
struct Memory {
    mask: BitMask,
    mem: HashMap<u64, u64>,
}

impl Memory {
    fn process(&mut self, s: &str) -> anyhow::Result<()> {
        if let Some(mask) = s.strip_prefix("mask = ") {
            self.mask = mask.parse::<BitMask>()?;
            Ok(())
        } else if let Some((addr, val)) = s.strip_prefix("mem[").and_then(|s| s.split_once("] = "))
        {
            let addr = addr.parse()?;
            let val = val.parse()?;
            self.mem.insert(addr, self.mask.mask(&val));
            Ok(())
        } else {
            Err(anyhow!("Unknown string {s}"))
        }
    }

    fn sum(&self) -> u64 {
        self.mem.values().sum()
    }
}

#[derive(Debug, Default)]
struct MemoryV2 {
    mask: BitMaskV2,
    mem: HashMap<u64, u64>,
}

impl MemoryV2 {
    fn process(&mut self, s: &str) -> anyhow::Result<()> {
        if let Some(mask) = s.strip_prefix("mask = ") {
            self.mask = mask.parse::<BitMaskV2>()?;
            Ok(())
        } else if let Some((addr, val)) = s.strip_prefix("mem[").and_then(|s| s.split_once("] = "))
        {
            let addr = addr.parse()?;
            let val = val.parse()?;
            for addr in self.mask.mask_iter(addr) {
                self.mem.insert(addr, val);
            }
            Ok(())
        } else {
            Err(anyhow!("Unknown string {s}"))
        }
    }

    fn sum(&self) -> u64 {
        self.mem.values().sum()
    }
}

#[derive(Debug)]
struct BitMaskV2 {
    ones: u64,
    xs: Vec<u64>,
    num_xs: u64,
}

impl Default for BitMaskV2 {
    fn default() -> Self {
        Self {
            ones: u64::MAX,
            xs: Vec::new(),
            num_xs: 0,
        }
    }
}

#[derive(Debug)]
struct MaskIter<'a> {
    mask: &'a BitMaskV2,
    pos: u64,
    num_values: u64,
    input: u64,
}

impl BitMaskV2 {
    fn mask_iter(&self, input: u64) -> MaskIter {
        let input = input | self.ones;
        MaskIter {
            mask: self,
            pos: 0,
            num_values: 2u64.pow(self.num_xs as u32),
            input,
        }
    }
}

impl Iterator for MaskIter<'_> {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos < self.num_values {
            let mut out_val = self.input;
            for x in 0..self.mask.num_xs {
                let bit = 1 << self.mask.xs[x as usize];
                if self.pos & (1 << x) == 0 {
                    out_val &= !bit;
                } else {
                    out_val |= bit;
                }
            }
            self.pos += 1;
            Some(out_val)
        } else {
            None
        }
    }
}

impl FromStr for BitMaskV2 {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut ones = 0;
        let mut xs = Vec::new();
        for (index, c) in s.char_indices().map(|(i, c)| (35 - i as u64, c)) {
            match c {
                'X' => {
                    xs.push(index);
                }
                '0' => {}
                '1' => {
                    ones |= 1 << index;
                }
                _ => bail!("Unknown char {c} in bitmask"),
            }
        }
        let num_xs = xs.len() as u64;
        xs.reverse();
        Ok(Self { ones, xs, num_xs })
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut memory = Memory::default();
    for line in input.lines() {
        memory.process(line).unwrap();
    }
    Some(memory.sum())
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut mem = MemoryV2::default();
    for line in input.lines() {
        mem.process(line).unwrap();
    }
    Some(mem.sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(165));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(208));
    }
}
