advent_of_code::solution!(18);

fn apply_op(v1: u64, v2: u64, op: char) -> u64 {
    match op {
        '+' => v1 + v2,
        '*' => v1 * v2,
        _ => panic!("Unknown op {op}"),
    }
}

fn precedence_pt1(c: char) -> i32 {
    match c {
        '+' | '*' => 1,
        _ => -1,
    }
}

fn precedence_pt2(c: char) -> i32 {
    match c {
        '+' => 2,
        '*' => 1,
        _ => -1,
    }
}

fn eval<T>(s: &str, precedence: T) -> u64
where
    T: Fn(char) -> i32,
{
    let mut values = Vec::new();
    let mut ops = Vec::new();
    let mut buf = [0; 4];
    for c in s.chars() {
        match c {
            ' ' => continue,
            '(' => ops.push('('),
            c if c.is_ascii_digit() => values.push(c.encode_utf8(&mut buf).parse::<u64>().unwrap()),
            ')' => {
                while ops.last().is_some_and(|&o| o != '(') {
                    let v1 = values.pop().unwrap();
                    let v2 = values.pop().unwrap();
                    let op = ops.pop().unwrap();
                    values.push(apply_op(v1, v2, op));
                }
                ops.pop();
            }
            c if c == '+' || c == '*' => {
                while ops.last().is_some_and(|&o| precedence(c) <= precedence(o)) {
                    let v1 = values.pop().unwrap();
                    let v2 = values.pop().unwrap();
                    let op = ops.pop().unwrap();
                    values.push(apply_op(v1, v2, op));
                }
                ops.push(c);
            }
            _ => panic!("unexpected token {c}"),
        }
    }
    while !ops.is_empty() {
        let v1 = values.pop().unwrap();
        let v2 = values.pop().unwrap();
        let op = ops.pop().unwrap();
        values.push(apply_op(v1, v2, op));
    }
    values.pop().unwrap()
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(input.lines().map(|l| eval(l, precedence_pt1)).sum())
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(input.lines().map(|l| eval(l, precedence_pt2)).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(26457));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(694173));
    }
}
