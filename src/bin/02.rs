advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<usize> {
    Some(
        input
            .lines()
            .filter(|l| {
                let (first, last) = l.split_once(": ").unwrap();
                let mut pts = first.split_whitespace();
                let range = pts.next().unwrap();
                let c = pts.next().unwrap().chars().next().unwrap();
                let (start, end) = range
                    .split_once('-')
                    .map(|(s, e)| (s.parse::<usize>().unwrap(), e.parse::<usize>().unwrap()))
                    .unwrap();
                (start..=end).contains(&last.chars().filter(|h| *h == c).count())
            })
            .count(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(
        input
            .lines()
            .filter(|l| {
                let (first, last) = l.split_once(": ").unwrap();
                let mut pts = first.split_whitespace();
                let range = pts.next().unwrap();
                let c = pts.next().unwrap().chars().next().unwrap();
                let (start, end) = range
                    .split_once('-')
                    .map(|(s, e)| {
                        (
                            s.parse::<usize>().unwrap() - 1,
                            e.parse::<usize>().unwrap() - 1,
                        )
                    })
                    .unwrap();
                let (a, b) = (
                    last.chars().nth(start).unwrap(),
                    last.chars().nth(end).unwrap(),
                );
                (a == c && b != c) || (a != c && b == c)
            })
            .count(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1));
    }
}
