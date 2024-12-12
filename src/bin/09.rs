use itertools::Itertools;

advent_of_code::solution!(9);

type Parsed = String;

pub fn parse(input: &str) -> Parsed {
    let mut rearranged = String::new();
    let mut count: usize = 0;
    for (fl, fs) in input.chars().tuples() {
        rearranged.push_str(
            &count
                .to_string()
                .repeat(fl.to_string().parse::<usize>().unwrap()),
        );
        rearranged.push_str(&".".repeat(fs.to_string().parse::<usize>().unwrap()));
        count += 1;
    }
    rearranged.push_str(
        &count.to_string().repeat(
            input
                .to_string()
                .pop()
                .unwrap()
                .to_string()
                .parse::<usize>()
                .unwrap(),
        ),
    );
    rearranged
}

pub fn part_one(input: Parsed) -> Option<u32> {
    let dc = input.matches('.').count();
    let split_pos = input.char_indices().nth_back(dc).unwrap().0;

    let mut end = "";
    while (end = &input[split_pos..]) != ".".repeat(dc) {}

    None
}

pub fn part_two(input: Parsed) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(parse(&advent_of_code::template::read_file("examples", DAY)));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(parse(&advent_of_code::template::read_file("examples", DAY)));
        assert_eq!(result, None);
    }
}
