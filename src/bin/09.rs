use std::cmp::Ordering;

advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<u64> {
    let list = input
        .lines()
        .map(|l| l.parse::<u64>().unwrap())
        .collect::<Vec<_>>();
    const WINDOW_SIZE: usize = if cfg!(test) { 5 } else { 25 };
    for window in list.windows(WINDOW_SIZE + 1) {
        if !window[..WINDOW_SIZE]
            .iter()
            .enumerate()
            .flat_map(|(i, n1)| window[i + 1..WINDOW_SIZE].iter().map(move |n2| (n1, n2)))
            .any(|(n1, n2)| n1 + n2 == window[WINDOW_SIZE])
        {
            return Some(window[WINDOW_SIZE]);
        }
    }
    None
}

pub fn part_two(input: &str) -> Option<u64> {
    let list = input
        .lines()
        .map(|l| l.parse::<u64>().unwrap())
        .collect::<Vec<_>>();
    const WINDOW_SIZE: usize = if cfg!(test) { 5 } else { 25 };
    let mut num = None;
    for window in list.windows(WINDOW_SIZE + 1) {
        if !window[..WINDOW_SIZE]
            .iter()
            .enumerate()
            .flat_map(|(i, n1)| window[i + 1..WINDOW_SIZE].iter().map(move |n2| (n1, n2)))
            .any(|(n1, n2)| n1 + n2 == window[WINDOW_SIZE])
        {
            num = Some(window[WINDOW_SIZE]);
        }
    }
    let num = num.unwrap();
    'outer: for start in 0..list.len() {
        for end in start + 1..list.len() {
            match list[start..end].iter().sum::<u64>().cmp(&num) {
                Ordering::Less => {}
                Ordering::Equal => {
                    let min = list[start..end].iter().min().unwrap();
                    let max = list[start..end].iter().max().unwrap();
                    return Some(min + max);
                }
                Ordering::Greater => {
                    continue 'outer;
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
        assert_eq!(result, Some(127));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(62));
    }
}
