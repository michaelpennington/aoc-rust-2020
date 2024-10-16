use advent_of_code::util::point::{Dir, Turn};

advent_of_code::solution!(12);

pub fn part_one(input: &str) -> Option<u32> {
    let mut loc = (0i32, 0i32);
    let mut facing = Dir::E;
    for line in input.lines() {
        let (i, num) = line.split_at(1);
        let num = num.parse::<i32>().unwrap();
        match (i, facing) {
            ("N", _) | ("F", Dir::N) => loc.1 -= num,
            ("S", _) | ("F", Dir::S) => loc.1 += num,
            ("E", _) | ("F", Dir::E) => loc.0 += num,
            ("W", _) | ("F", Dir::W) => loc.0 -= num,
            ("L", _) => {
                for _ in 0..(num / 90) {
                    facing.turn(Turn::L);
                }
            }
            ("R", _) => {
                for _ in 0..(num / 90) {
                    facing.turn(Turn::R);
                }
            }
            _ => unreachable!(),
        }
    }
    Some(loc.0.unsigned_abs() + loc.1.unsigned_abs())
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut loc = (0i32, 0i32);
    let mut wp = (10i32, -1i32);
    for line in input.lines() {
        let (i, num) = line.split_at(1);
        let num = num.parse::<i32>().unwrap();
        match i {
            "N" => wp.1 -= num,
            "S" => wp.1 += num,
            "E" => wp.0 += num,
            "W" => wp.0 -= num,
            "L" => {
                for _ in 0..(num / 90) {
                    std::mem::swap(&mut wp.0, &mut wp.1);
                    wp.1 = -wp.1;
                }
            }
            "R" => {
                for _ in 0..(num / 90) {
                    std::mem::swap(&mut wp.0, &mut wp.1);
                    wp.0 = -wp.0;
                }
            }
            "F" => {
                loc.0 += wp.0 * num;
                loc.1 += wp.1 * num;
            }
            _ => unreachable!(),
        }
    }

    Some(loc.0.unsigned_abs() + loc.1.unsigned_abs())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(25));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(286));
    }
}
