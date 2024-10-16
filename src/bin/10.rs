use std::collections::HashMap;

advent_of_code::solution!(10);

pub fn part_one(input: &str) -> Option<usize> {
    let mut nums = input
        .lines()
        .map(|l| l.parse::<u32>().unwrap())
        .chain(std::iter::once(0))
        .collect::<Vec<_>>();
    nums.sort();
    let max = nums.last().unwrap() + 3;
    nums.push(max);
    let num_ones = nums.windows(2).filter(|ns| ns[1] - ns[0] == 1).count();
    let num_threes = nums.windows(2).filter(|ns| ns[1] - ns[0] == 3).count();
    Some(num_ones * num_threes)
}

#[derive(Debug)]
struct ChargerList {
    nums: Vec<u32>,
    cache: HashMap<usize, usize>,
}

impl ChargerList {
    fn num_ways_here_to_end(&mut self, index: usize) -> usize {
        if let Some(v) = self.cache.get(&index) {
            *v
        } else if index >= self.nums.len() - 1 {
            1
        } else {
            let current = self.nums[index];
            match (
                self.nums.get(index + 1),
                self.nums.get(index + 2),
                self.nums.get(index + 3),
            ) {
                (Some(_), None, Some(_)) | (None, _, _) => unreachable!(),
                (Some(v), None, None) => {
                    if current + 3 < *v {
                        panic!("space greater than 3");
                    } else {
                        self.cache.insert(index, 1);
                        1
                    }
                }
                (Some(v1), Some(v2), None) => {
                    if current + 3 < *v1 {
                        panic!("space greater than 3");
                    } else if current + 3 < *v2 {
                        self.cache.insert(index, 1);
                        1
                    } else {
                        self.cache.insert(index, 2);
                        2
                    }
                }
                (Some(v1), Some(v2), Some(v3)) => {
                    use std::cmp::Ordering::*;
                    match (
                        (current + 3).cmp(v1),
                        (current + 3).cmp(v2),
                        (current + 3).cmp(v3),
                    ) {
                        (Greater, Less | Equal, Equal)
                        | (Equal, Less, Equal | Greater)
                        | (Equal, Equal | Greater, _)
                        | (_, _, Greater)
                        | (Less, _, _) => unreachable!(),

                        (Greater | Equal, Less, Less) => {
                            let ans = self.num_ways_here_to_end(index + 1);
                            self.cache.insert(index, ans);
                            ans
                        }

                        (Greater, Greater | Equal, Less) => {
                            let ans = self.num_ways_here_to_end(index + 1)
                                + self.num_ways_here_to_end(index + 2);
                            self.cache.insert(index, ans);
                            ans
                        }

                        (Greater, Greater, Equal) => {
                            let ans = self.num_ways_here_to_end(index + 1)
                                + self.num_ways_here_to_end(index + 2)
                                + self.num_ways_here_to_end(index + 3);
                            self.cache.insert(index, ans);
                            ans
                        }
                    }
                }
            }
        }
    }
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut nums = input
        .lines()
        .map(|l| l.parse::<u32>().unwrap())
        .chain(std::iter::once(0))
        .collect::<Vec<_>>();
    nums.sort();
    let max = nums.last().unwrap() + 3;
    nums.push(max);
    let mut list = ChargerList {
        nums,
        cache: HashMap::new(),
    };
    Some(list.num_ways_here_to_end(0))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_one_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(220));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two_one() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(19208));
    }
}
