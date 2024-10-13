advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let nums = input
        .lines()
        .map(|l| l.parse::<u32>().unwrap())
        .collect::<Vec<_>>();
    for (i, a) in nums.iter().enumerate() {
        for b in &nums[i + 1..] {
            if a + b == 2020 {
                return Some(a * b);
            }
        }
    }
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    let nums = input
        .lines()
        .map(|l| l.parse::<u32>().unwrap())
        .collect::<Vec<_>>();
    for (i, a) in nums.iter().enumerate() {
        for (j, b) in nums[i + 1..].iter().enumerate() {
            for c in &nums[i + j + 1..] {
                if a + b + c == 2020 {
                    return Some(a * b * c);
                }
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(514579));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(241861950));
    }
}
