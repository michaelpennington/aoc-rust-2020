use advent_of_code::util::euclid::crt;

advent_of_code::solution!(13);

pub fn part_one(input: &str) -> Option<u32> {
    let mut lines = input.lines();
    let time: u32 = lines.next().unwrap().parse().unwrap();
    lines
        .next()
        .unwrap()
        .split(',')
        .filter_map(|n| n.parse().ok())
        .map(|n| (n, time.div_ceil(n) * n))
        .min_by_key(|(_, t)| *t)
        .map(|(n, t)| n * (t - time))
}

pub fn part_two(input: &str) -> Option<i64> {
    let (a, m): (Vec<_>, Vec<_>) = input
        .lines()
        .nth(1)
        .unwrap()
        .split(',')
        .enumerate()
        .filter(|(_, n)| *n != "x")
        .map(|(i, n)| {
            let n = n.parse::<i64>().unwrap();
            (n - i as i64, n)
        })
        .unzip();
    crt(&a, &m)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(295));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1068781));
    }

    #[test]
    fn test_part_two_one() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(3417));
    }

    #[test]
    fn test_part_two_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(754018));
    }

    #[test]
    fn test_part_two_three() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        assert_eq!(result, Some(779210));
    }

    #[test]
    fn test_part_two_four() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 4,
        ));
        assert_eq!(result, Some(1261476));
    }

    #[test]
    fn test_part_two_five() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 5,
        ));
        assert_eq!(result, Some(1202161486));
    }
}
