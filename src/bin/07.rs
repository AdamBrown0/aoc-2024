advent_of_code::solution!(7);

/// pretty much just brute force it?
/// split, do i + i+1

pub fn part_one(input: &str) -> Option<u32> {
    for line in input.lines() {
        let parts: Vec<&str> = input.split(':').collect();
    }
    None
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}