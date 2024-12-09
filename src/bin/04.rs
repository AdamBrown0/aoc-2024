advent_of_code::solution!(4);

type Parsed<'a> = &'a str;

pub fn parse(input: &str) -> Parsed {
    input
}

pub fn part_one(_input: Parsed) -> Option<u32> {
    None
}

pub fn part_two(_input: Parsed) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
