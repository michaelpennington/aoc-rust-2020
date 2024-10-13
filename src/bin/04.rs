advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<usize> {
    Some(
        input
            .split("\n\n")
            .filter(|pp| {
                pp.contains("byr:")
                    && pp.contains("iyr:")
                    && pp.contains("eyr:")
                    && pp.contains("hgt:")
                    && pp.contains("hcl:")
                    && pp.contains("ecl:")
                    && pp.contains("pid:")
            })
            .count(),
    )
}

const EYE_COLORS: [&str; 7] = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

pub fn part_two(input: &str) -> Option<usize> {
    Some(
        input
            .split("\n\n")
            .filter(|pp| {
                pp.contains("byr:")
                    && pp.contains("iyr:")
                    && pp.contains("eyr:")
                    && pp.contains("hgt:")
                    && pp.contains("hcl:")
                    && pp.contains("ecl:")
                    && pp.contains("pid:")
            })
            .filter(|pp| {
                pp.split_whitespace().all(|f| {
                    let (key, val) = f.split_once(":").unwrap();
                    match key {
                        "byr" => val.parse().is_ok_and(|byr| (1920..=2002).contains(&byr)),
                        "iyr" => val.parse().is_ok_and(|iyr| (2010..=2020).contains(&iyr)),
                        "eyr" => val.parse().is_ok_and(|eyr| (2020..=2030).contains(&eyr)),
                        "hgt" => val
                            .strip_suffix("cm")
                            .map(|hgt| hgt.parse().is_ok_and(|hgt| (150..=193).contains(&hgt)))
                            .or_else(|| {
                                val.strip_suffix("in").map(|hgt| {
                                    hgt.parse().is_ok_and(|hgt| (59..=76).contains(&hgt))
                                })
                            })
                            .unwrap_or(false),
                        "hcl" => val.strip_prefix('#').is_some_and(|hex| {
                            hex.chars().all(|c| {
                                c.is_ascii_hexdigit()
                                    && (c.is_ascii_digit() || c.is_ascii_lowercase())
                            })
                        }),
                        "ecl" => EYE_COLORS.contains(&val),
                        "pid" => val.len() == 9 && val.chars().all(|c| c.is_ascii_digit()),
                        "cid" => true,
                        _ => panic!("Bad key {key}"),
                    }
                })
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
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(4));
    }
}
