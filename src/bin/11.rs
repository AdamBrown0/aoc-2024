use std::collections::HashMap;

advent_of_code::solution!(11);

type Parsed<'a> = &'a str;

pub fn parse(input: &str) -> Parsed {
    input
}

fn rule(stone: u128, cache: &mut HashMap<u128, Vec<u128>>) -> Option<Vec<u128>> {
    if let Some(cached_result) = cache.get(&stone) {
        return Some(cached_result.clone());
    }

    let stone_str = stone.to_string();
    let result = if stone == 0 {
        Some(vec![1])
    } else if stone_str.len() % 2 == 0 {
        let mid = stone_str.len() / 2;
        let left = stone_str[..mid].parse::<u128>().ok()?;
        let right = stone_str[mid..].parse::<u128>().ok()?;
        Some(vec![left, right])
    } else {
        Some(vec![stone.to_string().parse::<u128>().ok()? * 2024])
    };

    if let Some(ref res) = result {
        cache.insert(stone, res.clone());
    }

    result
}

pub fn part_one(input: Parsed) -> Option<u32> {
    let mut stones: Vec<u128> = input
        .split_whitespace()
        .map(|s| s.parse::<u128>().unwrap())
        .collect();
    let mut cache: HashMap<u128, Vec<u128>> = HashMap::new();
    for _ in 0..25 {
        let mut temp_stones: Vec<u128> = vec![];
        for &stone in &stones {
            // println!("Stone {}", stone);
            if let Some(new_stones) = rule(stone, &mut cache) {
                // println!("new_stones {:?}", new_stones);
                temp_stones.extend(new_stones);
            }
        }
        stones = temp_stones;
    }
    Some(stones.len() as u32)
}

pub fn part_two(input: Parsed) -> Option<u32> {
    let mut stones: Vec<u128> = input
        .split_whitespace()
        .map(|s| s.parse::<u128>().unwrap())
        .collect();
    let mut cache: HashMap<u128, Vec<u128>> = HashMap::new();
    for _ in 0..43 {
        let mut temp_stones: Vec<u128> = vec![];
        for &stone in &stones {
            // println!("Stone {}", stone);
            if let Some(new_stones) = rule(stone, &mut cache) {
                // println!("new_stones {:?}", new_stones);
                temp_stones.extend(new_stones);
            }
        }
        stones = temp_stones;
    }
    Some(stones.len() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(parse(&advent_of_code::template::read_file("examples", DAY)));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(parse(&advent_of_code::template::read_file("examples", DAY)));
        assert_eq!(result, Some(1));
    }
}
