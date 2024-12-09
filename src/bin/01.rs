advent_of_code::solution!(1);

type Parsed = (Vec<u32>, Vec<u32>);

pub fn parse(input: &str) -> Parsed {
    let mut left: Vec<u32> = vec![];
    let mut right: Vec<u32> = vec![];
    for line in input.lines() {
        let mut parts = line.split_whitespace();
        left.push(parts.next().unwrap().parse().unwrap());
        right.push(parts.next().unwrap().parse().unwrap());
    }
    (left, right)
}

pub fn part_one(input: Parsed) -> Option<u32> {
    let mut left = input.0;
    let mut right = input.1;

    left.sort();
    right.sort();
    let mut total_distance = 0;
    for (l, r) in left.iter().zip(right.iter()) {
        total_distance += l.abs_diff(*r);
    }
    Some(total_distance)
}

pub fn part_two(input: Parsed) -> Option<u32> {
    let left = input.0;
    let right = input.1;
    let score: u32 = left
        .iter()
        .map(|x| right.iter().filter(|&y| y == x).count() as u32 * x)
        .sum();
    Some(score)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(parse(&advent_of_code::template::read_file("examples", DAY)));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(parse(&advent_of_code::template::read_file("examples", DAY)));
        assert_eq!(result, Some(31));
    }
}
