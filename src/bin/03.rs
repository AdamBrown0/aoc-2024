use rayon::prelude::*;
use regex::Regex;

advent_of_code::solution!(3);

type Parsed<'a> = &'a str;

pub fn parse(input: &str) -> Parsed {
    input
}

pub fn part_one(input: Parsed) -> Option<u32> {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut sum = 0;
    for cap in re.captures_iter(input) {
        let x: u32 = cap[1].parse().unwrap();
        let y: u32 = cap[2].parse().unwrap();
        sum += x * y;
    }

    Some(sum)
}

pub fn part_two(input: Parsed) -> Option<u32> {
    let re = Regex::new(r"(?s)(?:^|do\(\))(.*?)(?:don't\(\)|$)").unwrap();
    let ret: u32 = re
        .captures_iter(input)
        .par_bridge()
        .map(|cap| part_one(cap.get(1).unwrap().as_str()).unwrap())
        .sum();

    Some(ret)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(48));
    }
}
