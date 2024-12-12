advent_of_code::solution!(5);

pub fn parse(input: &str) -> &str {
    input
}

pub fn part_one(_input: &str) -> Option<u32> {
    // let orderings = input.lines().take(1176);
    // let page_numbers = input.lines().skip(1177);
    // for line in page_numbers {
    //     // let _ = line.split(',').skip(1).map(); // each value runs through orderings n
    // }
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
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
