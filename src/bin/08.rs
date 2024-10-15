use std::{collections::HashSet, str::FromStr};

use strum::EnumString;

advent_of_code::solution!(8);

#[derive(Clone, Copy, Debug, EnumString, PartialEq, Eq)]
#[strum(serialize_all = "lowercase")]
enum Op {
    Nop,
    Acc,
    Jmp,
}

#[derive(Clone, Copy, Debug)]
struct Instruction {
    op: Op,
    val: isize,
}

impl Op {
    fn flip(&self) -> Self {
        match self {
            Op::Nop => Op::Jmp,
            Op::Acc => Op::Acc,
            Op::Jmp => Op::Nop,
        }
    }
}

impl FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (op, val) = s.split_once(' ').unwrap();
        let val = val.parse()?;
        let op = op.parse()?;
        Ok(Self { op, val })
    }
}

#[derive(Clone, Copy, Debug, Default)]
struct Computer {
    acc: isize,
    pc: usize,
}

impl Computer {
    fn calc(&mut self, code: &[Instruction]) -> Option<i32> {
        let mut seen = HashSet::new();
        while let Some(i) = code.get(self.pc) {
            if !seen.insert(self.pc) {
                return Some(self.acc as i32);
            }
            match i.op {
                Op::Nop => {}
                Op::Acc => self.acc += i.val,
                Op::Jmp => self.pc = self.pc.wrapping_add_signed(i.val.wrapping_sub(1)),
            }
            self.pc = self.pc.wrapping_add(1);
        }
        None
    }

    fn reset(&mut self) {
        self.acc = 0;
        self.pc = 0;
    }

    fn calc_w_flipped(&mut self, code: &[Instruction], index: usize) -> Option<i32> {
        let mut seen = HashSet::new();
        while let Some(i) = code.get(self.pc) {
            let op = if self.pc == index { i.op.flip() } else { i.op };
            if !seen.insert(self.pc) {
                self.reset();
                return None;
            }
            match op {
                Op::Nop => {}
                Op::Acc => self.acc += i.val,
                Op::Jmp => self.pc = self.pc.wrapping_add_signed(i.val.wrapping_sub(1)),
            }
            self.pc = self.pc.wrapping_add(1);
        }
        Some(self.acc as i32)
    }

    fn calc_pt2(&mut self, code: &[Instruction]) -> Option<i32> {
        (0..)
            .filter(|i| code.get(*i).unwrap().op != Op::Acc)
            .find_map(|i| self.calc_w_flipped(code, i))
    }
}

pub fn part_one(input: &str) -> Option<i32> {
    let code = input
        .lines()
        .map(|l| l.parse::<Instruction>().unwrap())
        .collect::<Vec<_>>();
    let mut computer = Computer::default();
    computer.calc(&code)
}

pub fn part_two(input: &str) -> Option<i32> {
    let code = input
        .lines()
        .map(|l| l.parse::<Instruction>().unwrap())
        .collect::<Vec<_>>();
    let mut computer = Computer::default();
    computer.calc_pt2(&code)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }
}
