use std::collections::HashMap;

advent_of_code::solution!(15);

pub fn part_one(input: &str) -> Option<usize> {
    let mut seen = HashMap::new();
    let mut start = 0;
    for (i, num) in input
        .trim()
        .split(',')
        .map(|n| n.parse::<usize>().unwrap())
        .enumerate()
    {
        start = i + 2;
        seen.insert(num, i + 1);
    }
    let mut next_num = 0;
    for i in start.. {
        if i == 2020 {
            return Some(next_num);
        }
        next_num = match seen.insert(next_num, i) {
            Some(last_i) => i - last_i,
            None => 0,
        };
    }
    None
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut seen = HashMap::new();
    let mut start = 0;
    const BOUNDARY: usize = 50000;
    let mut low_seen = [0usize; BOUNDARY];
    for (i, num) in input
        .trim()
        .split(',')
        .map(|n| n.parse::<usize>().unwrap())
        .enumerate()
    {
        start = i + 2;
        low_seen[num] = i + 1;
    }
    let mut next_num = 0;
    for i in start.. {
        if i == 30_000_000 {
            return Some(next_num);
        }
        next_num = if next_num < BOUNDARY {
            let mut num = i;
            std::mem::swap(&mut num, &mut low_seen[next_num]);
            if num == 0 {
                0
            } else {
                i - num
            }
        } else {
            seen.insert(next_num, i)
                .map(|last_i| i - last_i)
                .unwrap_or(0)
        };
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(436));
    }

    #[test]
    fn test_part_one_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(1));
    }

    #[test]
    fn test_part_one_two() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(10));
    }

    #[test]
    fn test_part_one_three() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        assert_eq!(result, Some(27));
    }

    #[test]
    fn test_part_one_four() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 4,
        ));
        assert_eq!(result, Some(78));
    }

    #[test]
    fn test_part_one_five() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 5,
        ));
        assert_eq!(result, Some(438));
    }

    #[test]
    fn test_part_one_six() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 6,
        ));
        assert_eq!(result, Some(1836));
    }

    // #[test]
    // fn test_part_two() {
    //     let result = part_two(&advent_of_code::template::read_file("examples", DAY));
    //     assert_eq!(result, Some(175594));
    // }
    //
    // #[test]
    // fn test_part_two_one() {
    //     let result = part_two(&advent_of_code::template::read_file_part(
    //         "examples", DAY, 1,
    //     ));
    //     assert_eq!(result, Some(2578));
    // }
    //
    // #[test]
    // fn test_part_two_two() {
    //     let result = part_two(&advent_of_code::template::read_file_part(
    //         "examples", DAY, 2,
    //     ));
    //     assert_eq!(result, Some(3544142));
    // }
    //
    // #[test]
    // fn test_part_two_three() {
    //     let result = part_two(&advent_of_code::template::read_file_part(
    //         "examples", DAY, 3,
    //     ));
    //     assert_eq!(result, Some(261214));
    // }
    //
    // #[test]
    // fn test_part_two_four() {
    //     let result = part_two(&advent_of_code::template::read_file_part(
    //         "examples", DAY, 4,
    //     ));
    //     assert_eq!(result, Some(6895259));
    // }
    //
    // #[test]
    // fn test_part_two_five() {
    //     let result = part_two(&advent_of_code::template::read_file_part(
    //         "examples", DAY, 5,
    //     ));
    //     assert_eq!(result, Some(18));
    // }
    //
    // #[test]
    // fn test_part_two_six() {
    //     let result = part_two(&advent_of_code::template::read_file_part(
    //         "examples", DAY, 6,
    //     ));
    //     assert_eq!(result, Some(362));
    // }
}
