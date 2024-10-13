advent_of_code::solution!(5);

fn pass_to_id(pass: &str) -> u32 {
    let new = pass
        .replace("B", "1")
        .replace("F", "0")
        .replace("R", "1")
        .replace("L", "0");
    u32::from_str_radix(&new, 2).unwrap()
}

pub fn part_one(input: &str) -> Option<u32> {
    input.lines().map(pass_to_id).max()
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut ids = input.lines().map(pass_to_id).collect::<Vec<_>>();
    ids.sort();
    ids.windows(2)
        .find(|ids| ids[1] - ids[0] == 2)
        .map(|ids| ids[1] - 1)
}
